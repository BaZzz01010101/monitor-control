#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    cell::RefCell,
    mem::size_of,
    path::PathBuf,
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use dell_controller_tray::{
    advanced_color_watcher::AdvancedColorWatcher,
    app_controller::{
        AppController, ControllerEffect, ShortcutTarget, UiAction, UiPane, WorkerEvent,
        WorkerRequest,
    },
    debug_ui::{capture_window_to_bmp, debug_ui_output_path, launch_mode_from_args, LaunchMode},
    hotkeys::ShortcutHotkeys,
    logger::{log_or_stderr, FileLogger},
    persistence::{
        PersistedAppState, PersistedSettings, PersistedWindowPosition, PersistenceStore,
    },
    tray_shell::TrayShell,
    ui_bridge::UiBridge,
    worker::{spawn_worker, WorkerHandle},
};
use global_hotkey::{GlobalHotKeyEvent, HotKeyState as GlobalHotKeyState};
use log::{debug, error, info};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use slint::{
    winit_030::{winit::event::WindowEvent, EventResult, WinitWindowAccessor},
    CloseRequestResponse, ComponentHandle, PhysicalPosition, Timer, TimerMode,
};
use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowPlacement, MessageBoxW, SetForegroundWindow, ShowWindow,
        MB_ICONERROR, MB_OK, SW_RESTORE, WINDOWPLACEMENT,
    },
};

const DEBUG_UI_CAPTURE_DELAY_MS: u64 = 2_000;
const EVENT_PUMP_INTERVAL_MS: u64 = 16;
const ACTIVE_SNAPSHOT_INTERVAL_MS: u64 = 1_000;
const INACTIVE_SNAPSHOT_INTERVAL_MS: u64 = 30_000;
const MONITOR_REFRESH_INTERVAL_MS: u64 = 300_000;
const MINIMIZED_WINDOW_SENTINEL_COORDINATE: i32 = -32_000;

struct AppRuntime {
    controller: AppController,
    ui: UiBridge,
    worker: WorkerHandle,
    advanced_color_watcher: Option<AdvancedColorWatcher>,
    worker_events: Receiver<WorkerEvent>,
    ui_action_tx: Sender<UiAction>,
    ui_actions: Receiver<UiAction>,
    tray: Option<TrayShell>,
    hotkeys: ShortcutHotkeys,
    pump_timer: Timer,
    debug_capture_timer: Timer,
    launch_mode: LaunchMode,
    debug_ui_output_path: Option<PathBuf>,
    logger: Option<FileLogger>,
    persistence: Option<PersistenceStore>,
    startup_autostart_target: Option<bool>,
    startup_selected_monitor_key: Option<String>,
    window_visible: bool,
    window_shown_once: bool,
    window_move_tracking_installed: bool,
    last_snapshot_request_ms: Option<u64>,
    last_monitor_refresh_ms: Option<u64>,
}

impl AppRuntime {
    fn new(
        launch_mode: LaunchMode,
        debug_ui_output_path: Option<PathBuf>,
        logger: Option<FileLogger>,
    ) -> anyhow::Result<Rc<RefCell<Self>>> {
        let (ui_action_tx, ui_action_rx) = mpsc::channel();
        let ui = UiBridge::new(ui_action_tx.clone())?;
        let close_request_tx = ui_action_tx.clone();
        ui.window().window().on_close_requested(move || {
            let _ = close_request_tx.send(UiAction::HideWindow);
            CloseRequestResponse::HideWindow
        });

        let (worker_event_tx, worker_events) = mpsc::channel();
        let worker = spawn_worker(worker_event_tx);
        let advanced_color_watcher = match AdvancedColorWatcher::new(worker.clone()) {
            Ok(watcher) => Some(watcher),
            Err(error) => {
                log_or_stderr(
                    logger.as_ref(),
                    format!("advanced-color event synchronization unavailable: {error}"),
                );
                None
            }
        };
        let tray = match launch_mode {
            LaunchMode::Normal => Some(TrayShell::new()?),
            LaunchMode::DebugUi => None,
        };
        let hotkeys = ShortcutHotkeys::new()?;
        let persistence = match PersistenceStore::for_current_user() {
            Ok(store) => Some(store),
            Err(error) => {
                log_or_stderr(logger.as_ref(), format!("persistence unavailable: {error}"));
                None
            }
        };

        let runtime = Rc::new(RefCell::new(Self {
            controller: AppController::default(),
            ui,
            worker,
            advanced_color_watcher,
            worker_events,
            ui_action_tx,
            ui_actions: ui_action_rx,
            tray,
            hotkeys,
            pump_timer: Timer::default(),
            debug_capture_timer: Timer::default(),
            launch_mode,
            debug_ui_output_path,
            logger,
            persistence,
            startup_autostart_target: None,
            startup_selected_monitor_key: None,
            window_visible: false,
            window_shown_once: false,
            window_move_tracking_installed: false,
            last_snapshot_request_ms: None,
            last_monitor_refresh_ms: None,
        }));

        runtime.borrow_mut().hydrate_from_persistence();
        Self::start_event_pump(runtime.clone());
        Ok(runtime)
    }

    fn shutdown(&mut self) {
        let Some(mut watcher) = self.advanced_color_watcher.take() else {
            return;
        };
        if let Err(error) = watcher.shutdown() {
            log_or_stderr(
                self.logger.as_ref(),
                format!("advanced-color event synchronization shutdown failed: {error}"),
            );
        }
    }

    fn bootstrap(runtime: Rc<RefCell<Self>>) -> anyhow::Result<()> {
        let now = now_ms();

        {
            let mut app = runtime.borrow_mut();
            let mut effects = Vec::new();
            if let Some(key) = app.startup_selected_monitor_key.clone() {
                effects.push(ControllerEffect::Worker(WorkerRequest::SelectMonitor {
                    key,
                }));
            }
            if app.launch_mode == LaunchMode::DebugUi {
                effects.extend(app.controller.handle_action(UiAction::OpenWindow, now));
            } else {
                effects.push(ControllerEffect::Worker(WorkerRequest::RefreshAll));
            }
            app.apply_effects(effects)?;
            if let Some(enabled) = app.startup_autostart_target {
                app.apply_effects(vec![ControllerEffect::Worker(
                    WorkerRequest::SetAutostart {
                        enabled,
                        quiet: true,
                    },
                )])?;
            }
            app.last_snapshot_request_ms = Some(now);
            app.last_monitor_refresh_ms = Some(now);
            app.sync_view();
        }

        if runtime.borrow().launch_mode == LaunchMode::DebugUi {
            Self::start_debug_capture(runtime);
        }

        info!("bootstrap complete");
        Ok(())
    }

    fn start_event_pump(runtime: Rc<RefCell<Self>>) {
        let timer_runtime = runtime.clone();
        runtime.borrow().pump_timer.start(
            TimerMode::Repeated,
            Duration::from_millis(EVENT_PUMP_INTERVAL_MS),
            move || {
                let result = timer_runtime.borrow_mut().pump_once();
                if let Err(error) = result {
                    timer_runtime
                        .borrow()
                        .log(format!("event pump failed: {error:#}"));
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
                    Err(error) => timer_runtime
                        .borrow()
                        .log(format!("debug-ui screenshot failed: {error:#}")),
                }
                slint::quit_event_loop().ok();
            },
        );
    }

    fn pump_once(&mut self) -> anyhow::Result<()> {
        let now = now_ms();

        while let Ok(action) = self.ui_actions.try_recv() {
            self.handle_action(action, now)
                .unwrap_or_else(|error| error!("handle_action failed: {error:#}"));
        }

        if let Some(tray) = self.tray.as_mut() {
            let actions = tray.drain_actions(now);
            for action in actions {
                self.handle_action(action, now)
                    .unwrap_or_else(|error| error!("tray action failed: {error:#}"));
            }
        }

        self.process_hotkey_events(now)?;

        while let Ok(event) = self.worker_events.try_recv() {
            let should_persist_settings = matches!(event, WorkerEvent::AutostartState { .. });
            if let WorkerEvent::Snapshot(snapshot) = &event {
                if let Some(watcher) = self.advanced_color_watcher.as_mut() {
                    if let Err(error) = watcher.update_target(snapshot.advanced_color_monitor) {
                        self.log(format!(
                            "advanced-color event synchronization unavailable: {error}"
                        ));
                    }
                }
            }
            self.controller.apply_worker_event(event, now);
            if should_persist_settings {
                self.persist_settings();
            }
        }

        let effects = self.controller.flush_pending(now);
        if let Err(error) = self.apply_effects(effects) {
            error!("apply_effects failed: {error:#}");
        }
        if let Err(error) = self.queue_periodic_snapshot(now) {
            error!("periodic snapshot failed: {error:#}");
        }
        self.sync_view();

        Ok(())
    }

    fn handle_action(&mut self, action: UiAction, now_ms: u64) -> anyhow::Result<()> {
        let should_persist_settings = matches!(&action, UiAction::SelectMonitor(_));
        match action {
            UiAction::CommitShortcut { target, shortcut } => {
                return self.commit_shortcut(target, shortcut);
            }
            UiAction::ClearShortcut(target) => {
                return self.clear_shortcut(target);
            }
            UiAction::PersistWindowState => {
                self.persist_window_state();
                return Ok(());
            }
            _ => {}
        }

        let should_focus_existing_window =
            should_focus_existing_window(action.clone(), self.window_visible);
        let effects = self.controller.handle_action(action, now_ms);
        if effects
            .iter()
            .any(|effect| matches!(effect, ControllerEffect::Worker(WorkerRequest::RefreshAll)))
        {
            self.last_monitor_refresh_ms = Some(now_ms);
        }
        self.apply_effects(effects)?;
        if should_persist_settings {
            self.persist_settings();
        }
        if should_focus_existing_window {
            self.focus_window()?;
        }
        Ok(())
    }

    fn commit_shortcut(&mut self, target: ShortcutTarget, shortcut: String) -> anyhow::Result<()> {
        let previous_value = self.controller.shortcut_value(target).to_string();
        match self.hotkeys.register_shortcut(target, &shortcut) {
            Ok(outcome) => {
                self.controller.apply_shortcut_registration_success(
                    target,
                    shortcut,
                    outcome.displaced_target,
                );
            }
            Err(error) => {
                self.controller
                    .apply_shortcut_registration_failure(target, previous_value, error);
            }
        }
        self.persist_settings();
        self.sync_view();
        Ok(())
    }

    fn clear_shortcut(&mut self, target: ShortcutTarget) -> anyhow::Result<()> {
        let previous_value = self.controller.shortcut_value(target).to_string();
        match self.hotkeys.clear_shortcut(target) {
            Ok(()) => self.controller.apply_shortcut_clear_success(target),
            Err(error) => {
                self.controller
                    .apply_shortcut_registration_failure(target, previous_value, error);
            }
        }
        self.persist_settings();
        self.sync_view();
        Ok(())
    }

    fn apply_effects(&mut self, effects: Vec<ControllerEffect>) -> anyhow::Result<()> {
        for effect in effects {
            debug!("effect: {effect:?}");
            match effect {
                ControllerEffect::ShowWindow => {
                    self.ui.show()?;
                    self.window_visible = true;
                    self.window_shown_once = true;
                    self.ensure_window_move_tracking();
                }
                ControllerEffect::HideWindow => {
                    self.persist_window_state();
                    self.ui.hide()?;
                    self.window_visible = false;
                }
                ControllerEffect::Quit => {
                    self.persist_window_state();
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
        if monitor_refresh_due(self.last_monitor_refresh_ms, now_ms) {
            self.last_monitor_refresh_ms = Some(now_ms);
            self.last_snapshot_request_ms = Some(now_ms);
            self.apply_effects(vec![ControllerEffect::Worker(WorkerRequest::RefreshAll)])?;
            return Ok(());
        }

        if snapshot_poll_due(self.last_snapshot_request_ms, now_ms, active) {
            self.last_snapshot_request_ms = Some(now_ms);
            self.apply_effects(vec![ControllerEffect::Worker(WorkerRequest::ReadSnapshot)])?;
        }
        Ok(())
    }

    fn process_hotkey_events(&mut self, now_ms: u64) -> anyhow::Result<()> {
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state != GlobalHotKeyState::Pressed {
                continue;
            }

            let Some(target) = self.hotkeys.target_for_event_id(event.id()) else {
                continue;
            };
            let settings_open = self.controller.ui_state().active_pane == UiPane::Settings;
            if !should_execute_registered_shortcut(settings_open, self.window_is_active()) {
                continue;
            }

            debug!("hotkey: {target:?} -> input switch");
            let effects = self
                .controller
                .handle_action(UiAction::SetInput(target.input_route()), now_ms);
            self.apply_effects(effects)?;
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

    fn focus_window(&self) -> anyhow::Result<()> {
        let window = self.ui.window().window();
        window.set_minimized(false);

        let hwnd = hwnd_from_window(window)?;
        unsafe {
            let _ = ShowWindow(hwnd, SW_RESTORE);
            let _ = SetForegroundWindow(hwnd);
        }

        Ok(())
    }

    fn sync_view(&mut self) {
        let state = self.controller.ui_state();
        self.ui.apply_state(&state);
        if let Some(tray) = self.tray.as_mut() {
            tray.set_autostart_checked(state.autostart_enabled);
            if let Err(error) =
                tray.set_monitor_choices(&state.monitor_choices, &state.selected_monitor_key)
            {
                self.log(format!("failed to update monitor tray menu: {error:#}"));
            }
        }
    }

    fn hydrate_from_persistence(&mut self) {
        let Some(store) = self.persistence.clone() else {
            return;
        };

        match store.load_settings_with_warning() {
            Ok(outcome) => {
                let settings = outcome.value;
                self.controller.hydrate_persisted_settings(&settings);
                self.startup_autostart_target = Some(settings.autostart_enabled);
                if !settings.selected_monitor_key.is_empty() {
                    self.startup_selected_monitor_key = Some(settings.selected_monitor_key.clone());
                }
                self.register_startup_shortcuts(&settings);
                if let Some(warning) = outcome.warning {
                    self.log(&warning);
                    self.controller
                        .apply_worker_event(WorkerEvent::Status(warning), now_ms());
                }
            }
            Err(error) => {
                self.log(format!("failed to load persisted settings: {error:#}"));
            }
        }

        match store.load_state_with_warning() {
            Ok(outcome) => {
                self.restore_window_position(&outcome.value);
                if let Some(warning) = outcome.warning {
                    self.log(&warning);
                    self.controller
                        .apply_worker_event(WorkerEvent::Status(warning), now_ms());
                }
            }
            Err(error) => self.log(format!("failed to load persisted app state: {error:#}")),
        }
    }

    fn register_startup_shortcuts(&mut self, settings: &PersistedSettings) {
        for (target, shortcut) in [
            (ShortcutTarget::UsbC, settings.tb_shortcut.as_str()),
            (ShortcutTarget::DisplayPort, settings.dp_shortcut.as_str()),
            (ShortcutTarget::Hdmi, settings.hdmi_shortcut.as_str()),
        ] {
            if shortcut.is_empty() || shortcut == "None" {
                continue;
            }

            match self.hotkeys.register_shortcut(target, shortcut) {
                Ok(outcome) => {
                    self.controller.apply_shortcut_registration_success(
                        target,
                        shortcut.to_string(),
                        outcome.displaced_target,
                    );
                }
                Err(error) => {
                    self.controller.apply_shortcut_registration_failure(
                        target,
                        "None".into(),
                        error,
                    );
                }
            }
        }
    }

    fn restore_window_position(&self, state: &PersistedAppState) {
        let Some(position) = state.window_position else {
            return;
        };
        if !should_restore_window_position(position) {
            return;
        }

        self.ui
            .window()
            .window()
            .set_position(PhysicalPosition::new(position.x, position.y));
    }

    fn persist_settings(&self) {
        let Some(store) = self.persistence.as_ref() else {
            return;
        };

        if let Err(error) = store.save_settings(&self.controller.persisted_settings()) {
            self.log(format!("failed to save settings: {error:#}"));
        }
    }

    fn persist_window_state(&self) {
        let Some(store) = self.persistence.as_ref() else {
            return;
        };
        let Some(position) = self.current_persistable_window_position() else {
            return;
        };
        let state = PersistedAppState {
            window_position: Some(position),
        };
        if let Err(error) = store.save_state(&state) {
            self.log(format!("failed to save app state: {error:#}"));
        }
    }

    fn current_persistable_window_position(&self) -> Option<PersistedWindowPosition> {
        if !self.window_shown_once {
            return None;
        }

        match window_placement(self.ui.window().window()) {
            Ok(placement) => persisted_window_position_from_placement(true, &placement),
            Err(error) => {
                self.log(format!("failed to read window placement: {error:#}"));
                None
            }
        }
    }

    fn ensure_window_move_tracking(&mut self) {
        if self.window_move_tracking_installed {
            return;
        }

        self.window_move_tracking_installed = true;
        let ui_action_tx = self.ui_action_tx.clone();
        self.ui
            .window()
            .window()
            .on_winit_window_event(move |_, event| {
                if matches!(event, WindowEvent::Moved(_)) {
                    let _ = ui_action_tx.send(UiAction::PersistWindowState);
                }
                EventResult::Propagate
            });
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

    fn log(&self, message: impl AsRef<str>) {
        log_or_stderr(self.logger.as_ref(), message);
    }
}

fn main() {
    env_logger::init();
    info!("starting dell-controller-tray");
    let logger = match FileLogger::for_current_user() {
        Ok(logger) => Some(logger),
        Err(error) => {
            eprintln!("file logging unavailable: {error:#}");
            None
        }
    };

    if let Err(error) = run_app(logger.clone()) {
        report_fatal_error(logger.as_ref(), &error);
        std::process::exit(1);
    }
}

fn run_app(logger: Option<FileLogger>) -> anyhow::Result<()> {
    let launch_mode = launch_mode_from_args(std::env::args().skip(1));
    if launch_mode == LaunchMode::DebugUi {
        std::env::set_var("SLINT_BACKEND", "winit-software");
    }
    let debug_output_path = match launch_mode {
        LaunchMode::DebugUi => Some(debug_ui_output_path(
            &std::env::current_dir().context("failed to determine current working directory")?,
            now_ms(),
        )),
        LaunchMode::Normal => None,
    };

    let runtime = AppRuntime::new(launch_mode, debug_output_path, logger)
        .context("failed to initialize tray app")?;
    AppRuntime::bootstrap(runtime.clone()).context("failed to bootstrap tray app")?;
    let event_loop_result = slint::run_event_loop_until_quit();
    runtime.borrow_mut().shutdown();
    event_loop_result.context("failed to run Slint event loop")?;
    info!("dell-controller-tray stopped");
    Ok(())
}

fn report_fatal_error(logger: Option<&FileLogger>, error: &anyhow::Error) {
    let message = format!("Dell Controller failed to start:\n{error:#}");
    log_or_stderr(logger, &message);
    show_fatal_message_box(&message);
}

fn show_fatal_message_box(message: &str) {
    let title = wide_null("Dell Controller");
    let message = wide_null(message);
    unsafe {
        let _ = MessageBoxW(
            None,
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONERROR,
        );
    }
}

fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
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

fn window_placement(window: &slint::Window) -> anyhow::Result<WINDOWPLACEMENT> {
    let hwnd = hwnd_from_window(window)?;
    let mut placement = WINDOWPLACEMENT {
        length: size_of::<WINDOWPLACEMENT>() as u32,
        ..Default::default()
    };
    unsafe { GetWindowPlacement(hwnd, &mut placement) }
        .ok()
        .context("failed to read window placement")?;
    Ok(placement)
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

fn monitor_refresh_due(last_monitor_refresh_ms: Option<u64>, now_ms: u64) -> bool {
    last_monitor_refresh_ms
        .is_none_or(|last_ms| now_ms.saturating_sub(last_ms) >= MONITOR_REFRESH_INTERVAL_MS)
}

fn should_focus_existing_window(action: UiAction, window_visible: bool) -> bool {
    matches!(action, UiAction::OpenWindow) && window_visible
}

fn should_execute_registered_shortcut(settings_open: bool, window_active: bool) -> bool {
    !(settings_open && window_active)
}

fn persisted_window_position_from_placement(
    window_shown_once: bool,
    placement: &WINDOWPLACEMENT,
) -> Option<PersistedWindowPosition> {
    if !window_shown_once {
        return None;
    }

    Some(PersistedWindowPosition {
        x: placement.rcNormalPosition.left,
        y: placement.rcNormalPosition.top,
    })
}

fn should_restore_window_position(position: PersistedWindowPosition) -> bool {
    !(position.x <= MINIMIZED_WINDOW_SENTINEL_COORDINATE
        && position.y <= MINIMIZED_WINDOW_SENTINEL_COORDINATE)
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

    #[test]
    fn monitor_refresh_due_uses_the_topology_interval() {
        assert!(monitor_refresh_due(None, 1_000));
        assert!(!monitor_refresh_due(Some(1_000), 300_999));
        assert!(monitor_refresh_due(Some(1_000), 301_000));
    }

    #[test]
    fn open_window_focuses_existing_windows_instead_of_reopening_them() {
        assert!(should_focus_existing_window(UiAction::OpenWindow, true));
        assert!(!should_focus_existing_window(UiAction::OpenWindow, false));
        assert!(!should_focus_existing_window(UiAction::Refresh, true));
    }

    #[test]
    fn shortcut_execution_is_blocked_only_for_the_active_settings_window() {
        assert!(!should_execute_registered_shortcut(true, true));
        assert!(should_execute_registered_shortcut(true, false));
        assert!(should_execute_registered_shortcut(false, true));
    }

    #[test]
    fn persisted_window_position_uses_the_normal_window_placement() {
        let placement = windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT {
            length: std::mem::size_of::<windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT>()
                as u32,
            showCmd: windows::Win32::UI::WindowsAndMessaging::SW_SHOWMINIMIZED.0 as u32,
            rcNormalPosition: windows::Win32::Foundation::RECT {
                left: 120,
                top: 340,
                right: 920,
                bottom: 940,
            },
            ..Default::default()
        };

        assert_eq!(
            persisted_window_position_from_placement(true, &placement),
            Some(PersistedWindowPosition { x: 120, y: 340 })
        );
    }

    #[test]
    fn minimized_sentinel_coordinates_are_not_restored() {
        assert!(!should_restore_window_position(PersistedWindowPosition {
            x: -32_000,
            y: -32_000,
        }));
        assert!(should_restore_window_position(PersistedWindowPosition {
            x: -1_920,
            y: 120,
        }));
    }
}
