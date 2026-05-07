use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use dell_controller_core::{
    ddc::VcpCode, enumerate_monitors, hdr, profiles::profile_for_model, startup, CommandQueue,
    RetryPolicy, WindowsDdcBackend, WindowsMonitor,
};

use crate::{
    app_controller::{
        FeatureSnapshot, InputRoute, MonitorSnapshot, WorkerEvent, WorkerRequest, BRIGHTNESS_CODE,
        CONTRAST_CODE, INPUT_DP_VALUE, INPUT_HDMI_VALUE, INPUT_USB_C_VALUE,
    },
    monitor_text::{compact_input_label, monitor_heading},
};

const INPUT_CODE: u8 = 0x60;

#[derive(Clone)]
pub struct WorkerHandle {
    request_tx: Sender<WorkerRequest>,
}

impl WorkerHandle {
    pub fn send(&self, request: WorkerRequest) -> Result<(), mpsc::SendError<WorkerRequest>> {
        self.request_tx.send(request)
    }
}

pub fn spawn_worker(event_tx: Sender<WorkerEvent>) -> WorkerHandle {
    let (request_tx, request_rx) = mpsc::channel();
    thread::spawn(move || run_worker(request_rx, event_tx));
    WorkerHandle { request_tx }
}

fn run_worker(request_rx: Receiver<WorkerRequest>, event_tx: Sender<WorkerEvent>) {
    let mut device = WindowsWorkerDevice::default();
    run_worker_loop(request_rx, event_tx, &mut device);
}

fn run_worker_loop<D: WorkerDevice>(
    request_rx: Receiver<WorkerRequest>,
    event_tx: Sender<WorkerEvent>,
    device: &mut D,
) {
    let mut pending = PendingWorkerRequests::default();

    loop {
        if pending.is_empty() {
            let Ok(request) = request_rx.recv() else {
                break;
            };
            pending.push(request);
        }

        pending.drain_available(&request_rx);
        if let Some(task) = pending.pop_next_task() {
            execute_worker_task(task, device, &event_tx);
        }
    }
}

trait WorkerDevice {
    fn refresh_monitors(&mut self) -> Result<(), String>;
    fn snapshot(&mut self) -> MonitorSnapshot;
    fn write_vcp(&mut self, code: u8, value: u32) -> Result<(), String>;
    fn autostart_enabled(&self) -> bool;
    fn set_autostart_enabled(&mut self, enabled: bool) -> Result<(), String>;
}

#[derive(Default)]
struct WindowsWorkerDevice {
    monitors: Vec<WindowsMonitor>,
}

impl WorkerDevice for WindowsWorkerDevice {
    fn refresh_monitors(&mut self) -> Result<(), String> {
        match enumerate_monitors() {
            Ok(found) => {
                self.monitors = found;
                Ok(())
            }
            Err(error) => {
                self.monitors.clear();
                Err(error.to_string())
            }
        }
    }

    fn snapshot(&mut self) -> MonitorSnapshot {
        build_snapshot(&self.monitors)
    }

    fn write_vcp(&mut self, code: u8, value: u32) -> Result<(), String> {
        write_feature(&self.monitors, code, value)
    }

    fn autostart_enabled(&self) -> bool {
        startup::is_autostart_enabled().unwrap_or(false)
    }

    fn set_autostart_enabled(&mut self, enabled: bool) -> Result<(), String> {
        startup::set_autostart_enabled(enabled).map_err(|error| error.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorkerTask {
    RefreshAll,
    ReadSnapshot,
    RefreshAutostart,
    ToggleAutostart,
    WriteVcp {
        code: u8,
        value: u32,
        input_status: bool,
    },
}

#[derive(Default)]
struct PendingWorkerRequests {
    requests: VecDeque<WorkerRequest>,
}

impl From<Vec<WorkerRequest>> for PendingWorkerRequests {
    fn from(requests: Vec<WorkerRequest>) -> Self {
        Self {
            requests: requests.into(),
        }
    }
}

impl PendingWorkerRequests {
    fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    fn push(&mut self, request: WorkerRequest) {
        self.requests.push_back(request);
    }

    fn drain_available(&mut self, request_rx: &Receiver<WorkerRequest>) {
        while let Ok(request) = request_rx.try_recv() {
            self.push(request);
        }
    }

    fn pop_next_task(&mut self) -> Option<WorkerTask> {
        let request = self.requests.pop_front()?;
        match request {
            WorkerRequest::RefreshAll => Some(WorkerTask::RefreshAll),
            WorkerRequest::ReadSnapshot => {
                self.requests
                    .retain(|request| !matches!(request, WorkerRequest::ReadSnapshot));
                Some(WorkerTask::ReadSnapshot)
            }
            WorkerRequest::RefreshAutostart => Some(WorkerTask::RefreshAutostart),
            WorkerRequest::ToggleAutostart => Some(WorkerTask::ToggleAutostart),
            WorkerRequest::WriteFeature { code, value } => {
                Some(self.coalesce_write(code, value, false))
            }
            WorkerRequest::SetInput { value } => Some(self.coalesce_write(INPUT_CODE, value, true)),
        }
    }

    fn coalesce_write(&mut self, code: u8, value: u32, input_status: bool) -> WorkerTask {
        let mut latest_value = value;
        let mut latest_input_status = input_status;
        let mut retained = VecDeque::new();

        while let Some(request) = self.requests.pop_front() {
            match write_request(&request) {
                Some(write) if write.code == code => {
                    latest_value = write.value;
                    latest_input_status = write.input_status;
                }
                _ => retained.push_back(request),
            }
        }

        self.requests = retained;
        WorkerTask::WriteVcp {
            code,
            value: latest_value,
            input_status: latest_input_status,
        }
    }
}

#[derive(Clone, Copy)]
struct VcpWrite {
    code: u8,
    value: u32,
    input_status: bool,
}

fn write_request(request: &WorkerRequest) -> Option<VcpWrite> {
    match request {
        WorkerRequest::WriteFeature { code, value } => Some(VcpWrite {
            code: *code,
            value: *value,
            input_status: false,
        }),
        WorkerRequest::SetInput { value } => Some(VcpWrite {
            code: INPUT_CODE,
            value: *value,
            input_status: true,
        }),
        _ => None,
    }
}

fn execute_worker_task<D: WorkerDevice>(
    task: WorkerTask,
    device: &mut D,
    event_tx: &Sender<WorkerEvent>,
) {
    match task {
        WorkerTask::RefreshAll => match device.refresh_monitors() {
            Ok(()) => {
                let _ = event_tx.send(WorkerEvent::Snapshot(device.snapshot()));
                let _ = event_tx.send(WorkerEvent::Status("Monitors refreshed".into()));
            }
            Err(error) => {
                let _ = event_tx.send(WorkerEvent::Snapshot(device.snapshot()));
                let _ = event_tx.send(WorkerEvent::Error(format!(
                    "Monitor refresh failed: {error}"
                )));
            }
        },
        WorkerTask::ReadSnapshot => {
            let _ = event_tx.send(WorkerEvent::Snapshot(device.snapshot()));
        }
        WorkerTask::RefreshAutostart => {
            let _ = event_tx.send(WorkerEvent::AutostartState {
                enabled: device.autostart_enabled(),
                status: String::new(),
            });
        }
        WorkerTask::ToggleAutostart => {
            let next = !device.autostart_enabled();
            match device.set_autostart_enabled(next) {
                Ok(()) => {
                    let _ = event_tx.send(WorkerEvent::AutostartState {
                        enabled: next,
                        status: if next {
                            "Autostart enabled".into()
                        } else {
                            "Autostart disabled".into()
                        },
                    });
                }
                Err(error) => {
                    let _ = event_tx.send(WorkerEvent::Error(format!(
                        "Autostart update failed: {error}"
                    )));
                }
            }
        }
        WorkerTask::WriteVcp {
            code,
            value,
            input_status,
        } => match device.write_vcp(code, value) {
            Ok(()) if input_status => {
                let _ = event_tx.send(WorkerEvent::Status(format!("Input set to {value:#X}")));
            }
            Ok(()) => {}
            Err(message) => {
                let _ = event_tx.send(WorkerEvent::Error(message));
            }
        },
    }
}

fn write_feature(monitors: &[WindowsMonitor], code: u8, value: u32) -> Result<(), String> {
    let Some(monitor) = monitors.first() else {
        return Err("No monitor available".into());
    };

    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    queue
        .set(VcpCode::new(code), value)
        .map_err(|error| format!("Set failed: {error}"))
}

fn build_snapshot(monitors: &[WindowsMonitor]) -> MonitorSnapshot {
    let Some(monitor) = monitors.first() else {
        return MonitorSnapshot {
            monitor_title: "No DDC/CI monitor detected".into(),
            input_summary: "Input".into(),
            hdr_status: "Windows HDR: unavailable".into(),
            brightness: FeatureSnapshot {
                value: 0,
                maximum: 100,
                available: false,
            },
            contrast: FeatureSnapshot {
                value: 0,
                maximum: 100,
                available: false,
            },
            selected_input: InputRoute::None,
            input_enabled: false,
            has_monitor: false,
        };
    };

    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    let brightness = read_feature(&queue, BRIGHTNESS_CODE);
    let contrast = read_feature(&queue, CONTRAST_CODE);
    let (input_summary, selected_input) = read_input(monitor, &queue);

    MonitorSnapshot {
        monitor_title: monitor_heading(&monitor.info.description, monitor.info.model.as_deref()),
        input_summary,
        hdr_status: hdr_status_text(),
        brightness,
        contrast,
        selected_input,
        input_enabled: true,
        has_monitor: true,
    }
}

fn read_feature(
    queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
    code: u8,
) -> FeatureSnapshot {
    match queue.get(VcpCode::new(code)) {
        Ok(feature) => FeatureSnapshot {
            value: feature.current.min(feature.maximum),
            maximum: feature.maximum.max(1),
            available: true,
        },
        Err(_) => FeatureSnapshot {
            value: 0,
            maximum: 100,
            available: false,
        },
    }
}

fn read_input(
    monitor: &WindowsMonitor,
    queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
) -> (String, InputRoute) {
    match queue.get(VcpCode::new(INPUT_CODE)) {
        Ok(feature) => {
            let profile = profile_for_model(monitor.info.model.as_deref());
            let label = profile
                .as_ref()
                .and_then(|profile| profile.control("input"))
                .and_then(|control| control.value_label(feature.current))
                .unwrap_or("unknown");
            (
                compact_input_label(label).into(),
                input_route_from_value(feature.current),
            )
        }
        Err(_) => ("Input".into(), InputRoute::None),
    }
}

fn input_route_from_value(value: u32) -> InputRoute {
    match value & 0xFF {
        INPUT_DP_VALUE => InputRoute::DisplayPort,
        INPUT_USB_C_VALUE => InputRoute::UsbC,
        INPUT_HDMI_VALUE => InputRoute::Hdmi,
        _ => InputRoute::None,
    }
}

fn hdr_status_text() -> String {
    match hdr::hdr_states() {
        Ok(states) if !states.is_empty() => {
            let state = &states[0];
            format!(
                "Windows HDR: {}",
                if state.enabled {
                    "On"
                } else if state.supported {
                    "Available, off"
                } else {
                    "Not supported"
                }
            )
        }
        _ => "Windows HDR: unavailable".into(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    #[derive(Default)]
    struct RecordingWorkerDevice {
        calls: Vec<String>,
    }

    impl WorkerDevice for RecordingWorkerDevice {
        fn refresh_monitors(&mut self) -> Result<(), String> {
            self.calls.push("refresh".into());
            Ok(())
        }

        fn snapshot(&mut self) -> MonitorSnapshot {
            self.calls.push("snapshot".into());
            MonitorSnapshot {
                monitor_title: "Dell U4025QW".into(),
                input_summary: "DP".into(),
                hdr_status: "Windows HDR: On".into(),
                brightness: FeatureSnapshot {
                    value: 50,
                    maximum: 100,
                    available: true,
                },
                contrast: FeatureSnapshot {
                    value: 80,
                    maximum: 100,
                    available: true,
                },
                selected_input: InputRoute::DisplayPort,
                input_enabled: true,
                has_monitor: true,
            }
        }

        fn write_vcp(&mut self, code: u8, value: u32) -> Result<(), String> {
            self.calls.push(format!("write:{code:02X}:{value}"));
            Ok(())
        }

        fn autostart_enabled(&self) -> bool {
            false
        }

        fn set_autostart_enabled(&mut self, enabled: bool) -> Result<(), String> {
            self.calls.push(format!("autostart:{enabled}"));
            Ok(())
        }
    }

    #[test]
    fn queued_writes_for_the_same_vcp_execute_only_the_newest_value() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::WriteFeature {
                code: BRIGHTNESS_CODE,
                value: 51,
            },
            WorkerRequest::WriteFeature {
                code: CONTRAST_CODE,
                value: 72,
            },
            WorkerRequest::WriteFeature {
                code: BRIGHTNESS_CODE,
                value: 64,
            },
        ]);

        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: BRIGHTNESS_CODE,
                value: 64,
                input_status: false,
            })
        );
        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: CONTRAST_CODE,
                value: 72,
                input_status: false,
            })
        );
        assert_eq!(pending.pop_next_task(), None);
    }

    #[test]
    fn input_writes_coalesce_with_raw_writes_for_the_input_vcp_code() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::WriteFeature {
                code: INPUT_CODE,
                value: INPUT_DP_VALUE,
            },
            WorkerRequest::SetInput {
                value: INPUT_HDMI_VALUE,
            },
        ]);

        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: INPUT_CODE,
                value: INPUT_HDMI_VALUE,
                input_status: true,
            })
        );
        assert_eq!(pending.pop_next_task(), None);
    }

    #[test]
    fn queued_snapshot_requests_coalesce_to_one_snapshot_task() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::ReadSnapshot,
            WorkerRequest::ReadSnapshot,
            WorkerRequest::WriteFeature {
                code: BRIGHTNESS_CODE,
                value: 62,
            },
        ]);

        assert_eq!(pending.pop_next_task(), Some(WorkerTask::ReadSnapshot));
        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: BRIGHTNESS_CODE,
                value: 62,
                input_status: false,
            })
        );
        assert_eq!(pending.pop_next_task(), None);
    }

    #[test]
    fn write_tasks_do_not_emit_snapshots() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut device = RecordingWorkerDevice::default();

        execute_worker_task(
            WorkerTask::WriteVcp {
                code: BRIGHTNESS_CODE,
                value: 64,
                input_status: false,
            },
            &mut device,
            &event_tx,
        );

        assert_eq!(device.calls, vec!["write:10:64"]);
        assert!(event_rx.try_recv().is_err());
    }
}
