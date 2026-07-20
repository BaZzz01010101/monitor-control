use std::{ffi::c_void, mem::size_of, sync::Arc};

use dell_controller_core::DisplayMonitorHandle;
use log::{debug, warn};
use parking_lot::Mutex;
use windows::{
    core::{factory, imp::define_interface, imp::interface_hierarchy, IInspectable, Interface},
    Foundation::TypedEventHandler,
    Graphics::Display::DisplayInformation,
    System::{DispatcherQueue, DispatcherQueueController, DispatcherQueueHandler},
    Win32::{
        Foundation::HWND,
        Graphics::Gdi::HMONITOR,
        System::WinRT::{
            CreateDispatcherQueueController, DispatcherQueueOptions, DQTAT_COM_STA,
            DQTYPE_THREAD_DEDICATED,
        },
    },
};

use crate::{app_controller::WorkerRequest, worker::WorkerHandle};

type SnapshotCallback = Arc<dyn Fn() + Send + Sync>;

trait AdvancedColorEventSource {
    type Handle: Copy + Eq;
    type Subscription;

    fn subscribe(
        &mut self,
        handle: Self::Handle,
        callback: SnapshotCallback,
    ) -> Result<Self::Subscription, String>;

    fn unsubscribe(&mut self, subscription: Self::Subscription);
}

struct WatcherCoordinator<S: AdvancedColorEventSource> {
    source: S,
    callback: SnapshotCallback,
    target: Option<S::Handle>,
    subscription: Option<S::Subscription>,
}

impl<S: AdvancedColorEventSource> WatcherCoordinator<S> {
    fn new(source: S, callback: SnapshotCallback) -> Self {
        Self {
            source,
            callback,
            target: None,
            subscription: None,
        }
    }

    fn update_target(&mut self, target: Option<S::Handle>) -> Result<(), String> {
        if self.target == target {
            return Ok(());
        }

        if let Some(subscription) = self.subscription.take() {
            self.source.unsubscribe(subscription);
        }
        self.target = None;

        let Some(target) = target else {
            return Ok(());
        };
        let subscription = self.source.subscribe(target, self.callback.clone())?;
        self.target = Some(target);
        self.subscription = Some(subscription);
        Ok(())
    }

    #[cfg(test)]
    fn source(&self) -> &S {
        &self.source
    }

    #[cfg(test)]
    fn into_source(mut self) -> S {
        if let Some(subscription) = self.subscription.take() {
            self.source.unsubscribe(subscription);
        }
        self.source
    }
}

struct WindowsAdvancedColorEventSource;

struct WindowsAdvancedColorSubscription {
    display_information: DisplayInformation,
    token: i64,
}

impl AdvancedColorEventSource for WindowsAdvancedColorEventSource {
    type Handle = DisplayMonitorHandle;
    type Subscription = WindowsAdvancedColorSubscription;

    fn subscribe(
        &mut self,
        handle: Self::Handle,
        callback: SnapshotCallback,
    ) -> Result<Self::Subscription, String> {
        let display_information = display_information_for_monitor(handle)
            .map_err(|error| format!("GetForMonitor failed: {error}"))?;
        let handler = TypedEventHandler::<DisplayInformation, IInspectable>::new(move |_, _| {
            callback();
            Ok(())
        });
        let token = display_information
            .AdvancedColorInfoChanged(&handler)
            .map_err(|error| format!("AdvancedColorInfoChanged registration failed: {error}"))?;
        Ok(WindowsAdvancedColorSubscription {
            display_information,
            token,
        })
    }

    fn unsubscribe(&mut self, subscription: Self::Subscription) {
        if let Err(error) = subscription
            .display_information
            .RemoveAdvancedColorInfoChanged(subscription.token)
        {
            warn!("advanced-color watcher unregistration failed: {error}");
        }
    }
}

pub struct AdvancedColorWatcher {
    controller: Option<DispatcherQueueController>,
    queue: DispatcherQueue,
    coordinator: Arc<Mutex<WatcherCoordinator<WindowsAdvancedColorEventSource>>>,
    requested_target: Option<DisplayMonitorHandle>,
}

impl AdvancedColorWatcher {
    pub fn new(worker: WorkerHandle) -> Result<Self, String> {
        let options = DispatcherQueueOptions {
            dwSize: size_of::<DispatcherQueueOptions>() as u32,
            threadType: DQTYPE_THREAD_DEDICATED,
            apartmentType: DQTAT_COM_STA,
        };
        let controller = unsafe { CreateDispatcherQueueController(options) }
            .map_err(|error| format!("dedicated DispatcherQueue creation failed: {error}"))?;
        let queue = controller
            .DispatcherQueue()
            .map_err(|error| format!("DispatcherQueue lookup failed: {error}"))?;
        let callback = Arc::new(move || {
            if worker.send(WorkerRequest::ReadSnapshot).is_err() {
                debug!("advanced-color event ignored because the worker has stopped");
            }
        });
        let coordinator = Arc::new(Mutex::new(WatcherCoordinator::new(
            WindowsAdvancedColorEventSource,
            callback,
        )));

        Ok(Self {
            controller: Some(controller),
            queue,
            coordinator,
            requested_target: None,
        })
    }

    pub fn update_target(&mut self, target: Option<DisplayMonitorHandle>) -> Result<(), String> {
        if self.controller.is_none() {
            return Err("advanced-color watcher is shut down".into());
        }
        if self.requested_target == target {
            return Ok(());
        }

        let coordinator = self.coordinator.clone();
        let handler = DispatcherQueueHandler::new(move || {
            if let Err(error) = coordinator.lock().update_target(target) {
                warn!("advanced-color event synchronization unavailable: {error}");
            }
            Ok(())
        });
        let queued = self
            .queue
            .TryEnqueue(&handler)
            .map_err(|error| format!("advanced-color watcher enqueue failed: {error}"))?;
        if !queued {
            return Err("advanced-color watcher queue is shutting down".into());
        }

        self.requested_target = target;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        let Some(controller) = self.controller.take() else {
            return Ok(());
        };
        self.requested_target = None;

        let coordinator = self.coordinator.clone();
        let handler = DispatcherQueueHandler::new(move || {
            if let Err(error) = coordinator.lock().update_target(None) {
                warn!("advanced-color watcher shutdown cleanup failed: {error}");
            }
            Ok(())
        });
        let cleanup_error = match self.queue.TryEnqueue(&handler) {
            Ok(true) => None,
            Ok(false) => Some("advanced-color watcher queue was already shutting down".to_owned()),
            Err(error) => Some(format!(
                "advanced-color watcher cleanup enqueue failed: {error}"
            )),
        };

        let shutdown = controller
            .ShutdownQueueAsync()
            .map_err(|error| format!("advanced-color DispatcherQueue shutdown failed: {error}"))?;
        shutdown.join().map_err(|error| {
            format!("advanced-color DispatcherQueue shutdown wait failed: {error}")
        })?;

        if let Some(error) = cleanup_error {
            return Err(error);
        }
        Ok(())
    }
}

impl Drop for AdvancedColorWatcher {
    fn drop(&mut self) {
        if self.controller.is_some() {
            warn!("advanced-color watcher dropped without explicit shutdown");
        }
    }
}

define_interface!(
    IDisplayInformationStaticsInterop,
    IDisplayInformationStaticsInterop_Vtbl,
    0x7449121c_382b_4705_8da7_a795ba482013
);
interface_hierarchy!(
    IDisplayInformationStaticsInterop,
    windows::core::IUnknown,
    IInspectable
);

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct IDisplayInformationStaticsInterop_Vtbl {
    base__: windows::core::IInspectable_Vtbl,
    _get_for_window: unsafe extern "system" fn(
        *mut c_void,
        HWND,
        *const windows::core::GUID,
        *mut *mut c_void,
    ) -> windows::core::HRESULT,
    get_for_monitor: unsafe extern "system" fn(
        *mut c_void,
        HMONITOR,
        *const windows::core::GUID,
        *mut *mut c_void,
    ) -> windows::core::HRESULT,
}

impl IDisplayInformationStaticsInterop {
    unsafe fn get_for_monitor<T: Interface>(&self, monitor: HMONITOR) -> windows::core::Result<T> {
        let mut result = std::ptr::null_mut();
        unsafe {
            (Interface::vtable(self).get_for_monitor)(
                Interface::as_raw(self),
                monitor,
                &T::IID,
                &mut result,
            )
            .and_then(|| windows::core::Type::from_abi(result))
        }
    }
}

fn display_information_for_monitor(
    monitor: DisplayMonitorHandle,
) -> windows::core::Result<DisplayInformation> {
    let interop = factory::<DisplayInformation, IDisplayInformationStaticsInterop>()?;
    unsafe { interop.get_for_monitor(monitor.as_hmonitor()) }
}

#[cfg(test)]
mod tests {
    use super::{AdvancedColorEventSource, AdvancedColorWatcher, WatcherCoordinator};
    use crate::worker::spawn_worker;
    use std::sync::{mpsc, Arc, Mutex};

    #[derive(Default)]
    struct FakeSource {
        registered: Vec<u64>,
        unregistered: Vec<u64>,
        callback: Option<Arc<dyn Fn() + Send + Sync>>,
    }

    impl AdvancedColorEventSource for FakeSource {
        type Handle = u64;
        type Subscription = u64;

        fn subscribe(
            &mut self,
            handle: u64,
            callback: Arc<dyn Fn() + Send + Sync>,
        ) -> Result<Self::Subscription, String> {
            self.registered.push(handle);
            self.callback = Some(callback);
            Ok(handle)
        }

        fn unsubscribe(&mut self, subscription: u64) {
            self.unregistered.push(subscription);
        }
    }

    #[test]
    fn registration_is_replaced_only_when_the_monitor_changes() {
        let callback = Arc::new(|| {});
        let mut coordinator = WatcherCoordinator::new(FakeSource::default(), callback);
        let first = 1;
        let second = 2;

        coordinator.update_target(Some(first)).unwrap();
        coordinator.update_target(Some(first)).unwrap();
        coordinator.update_target(Some(second)).unwrap();
        coordinator.update_target(None).unwrap();

        let source = coordinator.into_source();
        assert_eq!(source.registered, vec![first, second]);
        assert_eq!(source.unregistered, vec![first, second]);
    }

    #[test]
    fn event_callback_requests_one_snapshot_per_callback() {
        let callback_count = Arc::new(Mutex::new(0_u32));
        let callback_count_for_event = callback_count.clone();
        let callback = Arc::new(move || {
            *callback_count_for_event.lock().unwrap() += 1;
        });
        let mut coordinator = WatcherCoordinator::new(FakeSource::default(), callback);

        coordinator.update_target(Some(1)).unwrap();
        let callback = coordinator.source().callback.clone().unwrap();
        callback();
        callback();

        assert_eq!(*callback_count.lock().unwrap(), 2);
    }

    #[test]
    fn dispatcher_queue_shutdown_is_explicit_and_idempotent() {
        let (event_tx, _event_rx) = mpsc::channel();
        let worker = spawn_worker(event_tx);
        let mut watcher = AdvancedColorWatcher::new(worker).unwrap();

        watcher.shutdown().unwrap();
        watcher.shutdown().unwrap();
    }
}
