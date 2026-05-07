use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::mpsc::{self, Receiver},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use dell_controller_tray::{
    app_controller::{AppController, ControllerEffect, UiAction, WorkerEvent, WorkerRequest},
    debug_ui::{capture_window_to_bmp, debug_ui_output_path, launch_mode_from_args, LaunchMode},
    tray_shell::TrayShell,
    ui_bridge::UiBridge,
    worker::{spawn_worker, WorkerHandle},
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use slint::{CloseRequestResponse, ComponentHandle, Timer, TimerMode};
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::GetForegroundWindow};

const DEBUG_UI_CAPTURE_DELAY_MS: u64 = 2_000;
const EVENT_PUMP_INTERVAL_MS: u64 = 16;
const ACTIVE_SNAPSHOT_INTERVAL_MS: u64 = 1_000;
const INACTIVE_SNAPSHOT_INTERVAL_MS: u64 = 30_000;

struct AppRuntime {
    controller: AppController,
    ui: UiBridge,
    worker: WorkerHandle,
    worker_events: Receiver<WorkerEvent>,
    ui_actions: Receiver<UiAction>,
    tray: Option<TrayShell>,
    pump_timer: Timer,
    debug_capture_timer: Timer,
    launch_mode: LaunchMode,
    debug_ui_output_path: Option<PathBuf>,
    window_visible: bool,
    last_snapshot_request_ms: Option<u64>,
}

impl AppRuntime {
    fn new(
        launch_mode: LaunchMode,
        debug_ui_output_path: Option<PathBuf>,
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        let (ui_action_tx, ui_action_rx) = mpsc::channel();
        let ui = UiBridge::new(ui_action_tx.clone())?;
        ui.window().window().on_close_requested(move || {
            let _ = ui_action_tx.send(UiAction::HideWindow);
            CloseRequestResponse::HideWindow
        });

        let (worker_event_tx, worker_events) = mpsc::channel();
        let worker = spawn_worker(worker_event_tx);
        let tray = match launch_mode {
            LaunchMode::Normal => Some(TrayShell::new()?),
            LaunchMode::DebugUi => None,
        };

        let runtime = Rc::new(RefCell::new(Self {
            controller: AppController::default(),
            ui,
            worker,
            worker_events,
            ui_actions: ui_action_rx,
            tray,
            pump_timer: Timer::default(),
            debug_capture_timer: Timer::default(),
            launch_mode,
            debug_ui_output_path,
            window_visible: false,
            last_snapshot_request_ms: None,
        }));

        Self::start_event_pump(runtime.clone());
        Ok(runtime)
    }

    fn bootstrap(runtime: Rc<RefCell<Self>>) -> anyhow::Result<()> {
        let now = now_ms();

        {
            let mut app = runtime.borrow_mut();
            let effects = if app.launch_mode == LaunchMode::DebugUi {
                app.controller.handle_action(UiAction::OpenWindow, now)
            } else {
                vec![ControllerEffect::Worker(WorkerRequest::RefreshAll)]
            };
            app.apply_effects(effects)?;
            app.apply_effects(vec![ControllerEffect::Worker(
                WorkerRequest::RefreshAutostart,
            )])?;
            app.last_snapshot_request_ms = Some(now);
            app.sync_view();
        }

        if runtime.borrow().launch_mode == LaunchMode::DebugUi {
            Self::start_debug_capture(runtime);
        }

        Ok(())
    }

    fn start_event_pump(runtime: Rc<RefCell<Self>>) {
        let timer_runtime = runtime.clone();
        runtime.borrow().pump_timer.start(
            TimerMode::Repeated,
            Duration::from_millis(EVENT_PUMP_INTERVAL_MS),
            move || {
                if let Err(error) = timer_runtime.borrow_mut().pump_once() {
                    eprintln!("event pump failed: {error}");
                }
            },
        );
    }

    fn start_debug_capture(runtime: Rc<RefCell<Self>>) {
        let timer_runtime = runtime.clone();
        runtime.borrow().debug_capture_timer.start(
            TimerMode::SingleShot,
            Duration::from_millis(DEBUG_UI_CAPTURE_DELAY_MS),
            move || {
                let result = timer_runtime.borrow().capture_debug_ui();
                match result {
                    Ok(path) => println!("debug-ui screenshot written to {}", path.display()),
                    Err(error) => eprintln!("debug-ui screenshot failed: {error}"),
                }
                slint::quit_event_loop().ok();
            },
        );
    }

    fn pump_once(&mut self) -> anyhow::Result<()> {
        let now = now_ms();

        while let Ok(action) = self.ui_actions.try_recv() {
            let effects = self.controller.handle_action(action, now);
            self.apply_effects(effects)?;
        }

        if let Some(tray) = self.tray.as_mut() {
            let actions = tray.drain_actions(now);
            for action in actions {
                let effects = self.controller.handle_action(action, now);
                self.apply_effects(effects)?;
            }
        }

        while let Ok(event) = self.worker_events.try_recv() {
            self.controller.apply_worker_event(event, now);
        }

        let effects = self.controller.flush_pending(now);
        self.apply_effects(effects)?;
        self.queue_periodic_snapshot(now)?;
        self.sync_view();

        Ok(())
    }

    fn apply_effects(&mut self, effects: Vec<ControllerEffect>) -> anyhow::Result<()> {
        for effect in effects {
            match effect {
                ControllerEffect::ShowWindow => {
                    self.ui.show()?;
                    self.window_visible = true;
                }
                ControllerEffect::HideWindow => {
                    self.ui.hide()?;
                    self.window_visible = false;
                }
                ControllerEffect::Quit => {
                    slint::quit_event_loop().ok();
                }
                ControllerEffect::Worker(request) => {
                    self.worker
                        .send(request)
                        .context("failed to send request to worker")?;
                }
            }
        }

        Ok(())
    }

    fn queue_periodic_snapshot(&mut self, now_ms: u64) -> anyhow::Result<()> {
        let active = self.window_is_active();
        if snapshot_poll_due(self.last_snapshot_request_ms, now_ms, active) {
            self.last_snapshot_request_ms = Some(now_ms);
            self.apply_effects(vec![ControllerEffect::Worker(WorkerRequest::ReadSnapshot)])?;
        }
        Ok(())
    }

    fn window_is_active(&self) -> bool {
        self.window_visible && self.window_is_foreground()
    }

    fn window_is_foreground(&self) -> bool {
        hwnd_from_window(self.ui.window().window())
            .map(|hwnd| unsafe { GetForegroundWindow() == hwnd })
            .unwrap_or(false)
    }

    fn sync_view(&mut self) {
        let state = self.controller.ui_state();
        self.ui.apply_state(&state);
        if let Some(tray) = self.tray.as_ref() {
            tray.set_autostart_checked(state.autostart_enabled);
        }
    }

    fn capture_debug_ui(&self) -> anyhow::Result<PathBuf> {
        let path = self
            .debug_ui_output_path
            .clone()
            .context("missing debug-ui output path")?;
        let hwnd = hwnd_from_window(self.ui.window().window())?;
        capture_window_to_bmp(hwnd, &path)?;
        Ok(path)
    }
}

fn main() {
    let launch_mode = launch_mode_from_args(std::env::args().skip(1));
    if launch_mode == LaunchMode::DebugUi {
        std::env::set_var("SLINT_BACKEND", "winit-software");
    }
    let debug_output_path = match launch_mode {
        LaunchMode::DebugUi => Some(debug_ui_output_path(
            &std::env::current_dir().expect("failed to determine current working directory"),
            now_ms(),
        )),
        LaunchMode::Normal => None,
    };

    let runtime =
        AppRuntime::new(launch_mode, debug_output_path).expect("failed to initialize tray app");
    AppRuntime::bootstrap(runtime).expect("failed to bootstrap tray app");
    slint::run_event_loop_until_quit().expect("failed to run Slint event loop");
}

fn hwnd_from_window(window: &slint::Window) -> anyhow::Result<HWND> {
    let raw = window
        .window_handle()
        .window_handle()
        .context("failed to read Slint window handle")?
        .as_raw();

    match raw {
        RawWindowHandle::Win32(handle) => Ok(HWND(handle.hwnd.get() as *mut core::ffi::c_void)),
        _ => Err(anyhow::anyhow!(
            "Slint did not expose a Win32 window handle"
        )),
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn snapshot_poll_interval_ms(window_active: bool) -> u64 {
    if window_active {
        ACTIVE_SNAPSHOT_INTERVAL_MS
    } else {
        INACTIVE_SNAPSHOT_INTERVAL_MS
    }
}

fn snapshot_poll_due(
    last_snapshot_request_ms: Option<u64>,
    now_ms: u64,
    window_active: bool,
) -> bool {
    last_snapshot_request_ms.is_none_or(|last_ms| {
        now_ms.saturating_sub(last_ms) >= snapshot_poll_interval_ms(window_active)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_poll_interval_tracks_window_activity() {
        assert_eq!(snapshot_poll_interval_ms(true), 1_000);
        assert_eq!(snapshot_poll_interval_ms(false), 30_000);
    }

    #[test]
    fn snapshot_poll_due_uses_the_selected_interval() {
        assert!(snapshot_poll_due(Some(1_000), 2_000, true));
        assert!(!snapshot_poll_due(Some(1_000), 1_999, true));
        assert!(snapshot_poll_due(Some(1_000), 31_000, false));
        assert!(!snapshot_poll_due(Some(1_000), 30_999, false));
        assert!(snapshot_poll_due(None, 1_000, true));
    }
}
