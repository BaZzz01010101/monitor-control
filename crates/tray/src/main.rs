extern crate native_windows_gui as nwg;

use std::cell::RefCell;

use dell_controller_core::{
    ddc::VcpCode, enumerate_monitors, hdr, profiles::profile_for_model, startup, CommandQueue,
    RetryPolicy, WindowsDdcBackend, WindowsMonitor,
};
use nwg::NativeUi;

#[derive(Default)]
pub struct TrayApp {
    message_window: nwg::MessageWindow,
    control_window: nwg::Window,
    icon: nwg::Icon,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    open_item: nwg::MenuItem,
    refresh_item: nwg::MenuItem,
    autostart_item: nwg::MenuItem,
    exit_item: nwg::MenuItem,
    monitor_label: nwg::Label,
    hdr_label: nwg::Label,
    brightness_label: nwg::Label,
    contrast_label: nwg::Label,
    input_label: nwg::Label,
    status_label: nwg::Label,
    picture_title: nwg::Label,
    input_title: nwg::Label,
    picture_frame: nwg::Frame,
    input_frame: nwg::Frame,
    refresh_button: nwg::Button,
    brightness_down: nwg::Button,
    brightness_up: nwg::Button,
    contrast_down: nwg::Button,
    contrast_up: nwg::Button,
    input_dp: nwg::Button,
    input_usbc: nwg::Button,
    input_hdmi: nwg::Button,
    monitors: RefCell<Vec<WindowsMonitor>>,
}

impl TrayApp {
    fn active_monitor(&self) -> Option<WindowsMonitor> {
        self.monitors.borrow().first().cloned()
    }

    fn show_control_window(&self) {
        self.control_window.set_visible(true);
        self.control_window.set_focus();
        self.refresh_status();
    }

    fn hide_control_window(&self) {
        self.control_window.set_visible(false);
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn reload_monitors(&self) {
        match enumerate_monitors() {
            Ok(monitors) => {
                *self.monitors.borrow_mut() = monitors;
                self.set_status("Monitors refreshed");
                self.refresh_status();
            }
            Err(error) => self.set_status(&format!("Monitor refresh failed: {error}")),
        }
    }

    fn refresh_autostart_check(&self) {
        if let Ok(enabled) = startup::is_autostart_enabled() {
            self.autostart_item.set_checked(enabled);
        }
    }

    fn toggle_autostart(&self) {
        let next = !startup::is_autostart_enabled().unwrap_or(false);
        match startup::set_autostart_enabled(next) {
            Ok(()) => {
                self.autostart_item.set_checked(next);
                self.set_status(if next {
                    "Autostart enabled"
                } else {
                    "Autostart disabled"
                });
            }
            Err(error) => self.set_status(&format!("Autostart update failed: {error}")),
        }
    }

    fn refresh_status(&self) {
        let Some(monitor) = self.active_monitor() else {
            self.monitor_label.set_text("No DDC/CI monitor detected");
            self.brightness_label.set_text("Brightness: n/a");
            self.contrast_label.set_text("Contrast: n/a");
            self.input_label.set_text("Input: n/a");
            self.refresh_hdr();
            return;
        };

        let model = monitor.info.model.as_deref().unwrap_or("unknown");
        self.monitor_label
            .set_text(&format!("{} ({model})", monitor.info.description));

        let queue: CommandQueue<_> =
            CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
        self.set_feature_label(&queue, 0x10, "Brightness", &self.brightness_label);
        self.set_feature_label(&queue, 0x12, "Contrast", &self.contrast_label);
        self.set_input_label(&monitor, &queue);
        self.refresh_hdr();
    }

    fn set_feature_label(
        &self,
        queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
        code: u8,
        label: &str,
        target: &nwg::Label,
    ) {
        match queue.get(VcpCode::new(code)) {
            Ok(feature) => target.set_text(&format!("{label}: {}", feature.current)),
            Err(_) => target.set_text(&format!("{label}: unavailable")),
        }
    }

    fn set_input_label(
        &self,
        monitor: &WindowsMonitor,
        queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
    ) {
        match queue.get(VcpCode::new(0x60)) {
            Ok(feature) => {
                let profile = profile_for_model(monitor.info.model.as_deref());
                let label = profile
                    .as_ref()
                    .and_then(|profile| profile.control("input"))
                    .and_then(|control| control.value_label(feature.current))
                    .unwrap_or("unknown");
                self.input_label
                    .set_text(&format!("Input: {label} ({:#X})", feature.current));
            }
            Err(_) => self.input_label.set_text("Input: unavailable"),
        }
    }

    fn refresh_hdr(&self) {
        match hdr::hdr_states() {
            Ok(states) if !states.is_empty() => {
                let state = &states[0];
                self.hdr_label.set_text(&format!(
                    "Windows HDR: {}",
                    if state.enabled {
                        "On"
                    } else if state.supported {
                        "Available, off"
                    } else {
                        "Not supported"
                    }
                ));
            }
            _ => self.hdr_label.set_text("Windows HDR: unavailable"),
        }
    }

    fn adjust_vcp(&self, code: u8, delta: i32) {
        let Some(monitor) = self.active_monitor() else {
            self.set_status("No monitor available");
            return;
        };
        let queue: CommandQueue<_> =
            CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
        match queue.get(VcpCode::new(code)) {
            Ok(feature) => {
                let current = feature.current as i32;
                let max = feature.maximum as i32;
                let next = (current + delta).clamp(0, max) as u32;
                match queue.set(VcpCode::new(code), next) {
                    Ok(()) => {
                        self.set_status(&format!("Set 0x{code:02X} to {next}"));
                        self.refresh_status();
                    }
                    Err(error) => self.set_status(&format!("Set failed: {error}")),
                }
            }
            Err(error) => self.set_status(&format!("Read failed: {error}")),
        }
    }

    fn set_input(&self, value: u32) {
        let Some(monitor) = self.active_monitor() else {
            self.set_status("No monitor available");
            return;
        };
        let queue: CommandQueue<_> =
            CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
        match queue.set(VcpCode::new(0x60), value) {
            Ok(()) => {
                self.set_status(&format!("Input set to {value:#X}"));
                self.refresh_status();
            }
            Err(error) => self.set_status(&format!("Input switch failed: {error}")),
        }
    }

    fn set_status(&self, message: &str) {
        self.status_label.set_text(message);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

mod ui {
    use std::{cell::RefCell, ops::Deref, rc::Rc};

    use native_windows_gui as nwg;

    use super::*;

    pub struct TrayAppUi {
        inner: Rc<TrayApp>,
        handlers: RefCell<Vec<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<TrayAppUi> for TrayApp {
        fn build_ui(mut data: TrayApp) -> Result<TrayAppUi, nwg::NwgError> {
            use nwg::Event as E;

            nwg::Icon::builder()
                .source_system(Some(nwg::OemIcon::Information))
                .size(Some((16, 16)))
                .build(&mut data.icon)?;

            nwg::MessageWindow::builder().build(&mut data.message_window)?;

            nwg::Window::builder()
                .size((460, 360))
                .position((300, 300))
                .title("Dell Controller")
                .build(&mut data.control_window)?;
            data.control_window.set_visible(false);

            nwg::TrayNotification::builder()
                .parent(&data.message_window)
                .icon(Some(&data.icon))
                .tip(Some("Dell Controller"))
                .build(&mut data.tray)?;

            nwg::Menu::builder()
                .popup(true)
                .parent(&data.message_window)
                .build(&mut data.tray_menu)?;

            nwg::MenuItem::builder()
                .text("Open")
                .parent(&data.tray_menu)
                .build(&mut data.open_item)?;
            nwg::MenuItem::builder()
                .text("Refresh")
                .parent(&data.tray_menu)
                .build(&mut data.refresh_item)?;
            nwg::MenuItem::builder()
                .text("Start with Windows")
                .parent(&data.tray_menu)
                .build(&mut data.autostart_item)?;
            nwg::MenuItem::builder()
                .text("Exit")
                .parent(&data.tray_menu)
                .build(&mut data.exit_item)?;

            build_label(
                &mut data.monitor_label,
                &data.control_window,
                "Detecting monitor...",
                16,
                14,
                420,
                24,
            )?;
            build_label(
                &mut data.hdr_label,
                &data.control_window,
                "Windows HDR: unknown",
                16,
                42,
                240,
                24,
            )?;
            build_button(
                &mut data.refresh_button,
                &data.control_window,
                "Refresh",
                348,
                38,
                88,
                28,
            )?;
            build_label(
                &mut data.picture_title,
                &data.control_window,
                "Picture",
                16,
                78,
                120,
                22,
            )?;
            nwg::Frame::builder()
                .position((16, 110))
                .size((420, 90))
                .parent(&data.control_window)
                .flags(nwg::FrameFlags::VISIBLE | nwg::FrameFlags::BORDER)
                .build(&mut data.picture_frame)?;
            build_label(
                &mut data.brightness_label,
                &data.picture_frame,
                "Brightness: n/a",
                16,
                16,
                170,
                24,
            )?;
            build_label(
                &mut data.contrast_label,
                &data.picture_frame,
                "Contrast: n/a",
                16,
                52,
                170,
                24,
            )?;
            build_button(
                &mut data.brightness_down,
                &data.picture_frame,
                "-",
                220,
                12,
                44,
                28,
            )?;
            build_button(
                &mut data.brightness_up,
                &data.picture_frame,
                "+",
                272,
                12,
                44,
                28,
            )?;
            build_button(
                &mut data.contrast_down,
                &data.picture_frame,
                "-",
                220,
                48,
                44,
                28,
            )?;
            build_button(
                &mut data.contrast_up,
                &data.picture_frame,
                "+",
                272,
                48,
                44,
                28,
            )?;
            build_label(
                &mut data.input_title,
                &data.control_window,
                "Input",
                16,
                218,
                120,
                22,
            )?;
            nwg::Frame::builder()
                .position((16, 250))
                .size((420, 72))
                .parent(&data.control_window)
                .flags(nwg::FrameFlags::VISIBLE | nwg::FrameFlags::BORDER)
                .build(&mut data.input_frame)?;
            build_label(
                &mut data.input_label,
                &data.input_frame,
                "Input: n/a",
                16,
                10,
                380,
                24,
            )?;
            build_button(&mut data.input_dp, &data.input_frame, "DP", 16, 38, 80, 28)?;
            build_button(
                &mut data.input_usbc,
                &data.input_frame,
                "USB-C",
                104,
                38,
                80,
                28,
            )?;
            build_button(
                &mut data.input_hdmi,
                &data.input_frame,
                "HDMI",
                192,
                38,
                80,
                28,
            )?;
            build_label(
                &mut data.status_label,
                &data.control_window,
                "Ready",
                16,
                326,
                420,
                24,
            )?;

            let ui = TrayAppUi {
                inner: Rc::new(data),
                handlers: RefCell::new(Vec::new()),
            };

            ui.inner.reload_monitors();
            ui.inner.refresh_autostart_check();

            let tray_events = Rc::downgrade(&ui.inner);
            let tray_handler = nwg::full_bind_event_handler(
                &ui.inner.message_window.handle,
                move |evt, _evt_data, handle| {
                    if let Some(app) = tray_events.upgrade() {
                        match evt {
                            E::OnContextMenu if handle == app.tray => app.show_menu(),
                            E::OnMenuItemSelected if handle == app.open_item => {
                                app.show_control_window()
                            }
                            E::OnMenuItemSelected if handle == app.refresh_item => {
                                app.reload_monitors()
                            }
                            E::OnMenuItemSelected if handle == app.autostart_item => {
                                app.toggle_autostart()
                            }
                            E::OnMenuItemSelected if handle == app.exit_item => app.exit(),
                            _ => {}
                        }
                    }
                },
            );
            ui.handlers.borrow_mut().push(tray_handler);

            let control_events = Rc::downgrade(&ui.inner);
            let control_handler = nwg::full_bind_event_handler(
                &ui.inner.control_window.handle,
                move |evt, _evt_data, handle| {
                    if let Some(app) = control_events.upgrade() {
                        match evt {
                            E::OnWindowClose if handle == app.control_window => {
                                app.hide_control_window()
                            }
                            E::OnButtonClick if handle == app.refresh_button => {
                                app.reload_monitors()
                            }
                            E::OnButtonClick if handle == app.brightness_down => {
                                app.adjust_vcp(0x10, -5)
                            }
                            E::OnButtonClick if handle == app.brightness_up => {
                                app.adjust_vcp(0x10, 5)
                            }
                            E::OnButtonClick if handle == app.contrast_down => {
                                app.adjust_vcp(0x12, -5)
                            }
                            E::OnButtonClick if handle == app.contrast_up => {
                                app.adjust_vcp(0x12, 5)
                            }
                            E::OnButtonClick if handle == app.input_dp => app.set_input(0x0F),
                            E::OnButtonClick if handle == app.input_usbc => app.set_input(0x19),
                            E::OnButtonClick if handle == app.input_hdmi => app.set_input(0x11),
                            _ => {}
                        }
                    }
                },
            );
            ui.handlers.borrow_mut().push(control_handler);

            Ok(ui)
        }
    }

    impl Drop for TrayAppUi {
        fn drop(&mut self) {
            for handler in self.handlers.borrow_mut().drain(..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for TrayAppUi {
        type Target = TrayApp;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    fn build_label(
        label: &mut nwg::Label,
        parent: impl Into<nwg::ControlHandle>,
        text: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), nwg::NwgError> {
        nwg::Label::builder()
            .text(text)
            .position((x, y))
            .size((width, height))
            .parent(parent)
            .build(label)
    }

    fn build_button(
        button: &mut nwg::Button,
        parent: impl Into<nwg::ControlHandle>,
        text: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), nwg::NwgError> {
        nwg::Button::builder()
            .text(text)
            .position((x, y))
            .size((width, height))
            .parent(parent)
            .build(button)
    }
}

fn main() {
    nwg::init().expect("failed to initialize Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("failed to set default font");
    let _ui = TrayApp::build_ui(Default::default()).expect("failed to build tray UI");
    nwg::dispatch_thread_events();
}
