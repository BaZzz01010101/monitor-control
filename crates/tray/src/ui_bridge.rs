use log::trace;
use std::sync::mpsc::Sender;

use anyhow::Context;

use crate::app_controller::{InputRoute, ShortcutTarget, UiAction, UiState};
use crate::shortcut_capture::normalize_shortcut_key;

slint::include_modules!();

pub struct UiBridge {
    window: MainWindow,
}

impl UiBridge {
    pub fn new(action_tx: Sender<UiAction>) -> anyhow::Result<Self> {
        let window = MainWindow::new().context("failed to create Slint main window")?;
        wire_callbacks(&window, action_tx);
        Ok(Self { window })
    }

    pub fn window(&self) -> &MainWindow {
        &self.window
    }

    pub fn show(&self) -> anyhow::Result<()> {
        self.window.show().context("failed to show Slint window")
    }

    pub fn hide(&self) -> anyhow::Result<()> {
        self.window.hide().context("failed to hide Slint window")
    }

    pub fn apply_state(&self, state: &UiState) {
        trace!("bridging state to ui");
        self.window
            .set_settings_open(state.active_pane == crate::app_controller::UiPane::Settings);
        self.window
            .set_monitor_title(state.monitor_title.clone().into());
        self.window
            .set_input_summary(state.input_summary.clone().into());
        self.window.set_hdr_status(state.hdr_status.clone().into());
        self.window
            .set_brightness_value(state.brightness.value as i32);
        self.window
            .set_brightness_maximum(state.brightness.maximum as i32);
        self.window.set_brightness_enabled(state.brightness.enabled);
        self.window.set_contrast_value(state.contrast.value as i32);
        self.window
            .set_contrast_maximum(state.contrast.maximum as i32);
        self.window.set_contrast_enabled(state.contrast.enabled);
        self.window
            .set_selected_input(input_route_to_ui_index(state.selected_input));
        self.window.set_input_enabled(state.input_enabled);
        self.window.set_autostart_enabled(state.autostart_enabled);
        self.window
            .set_tb_shortcut(state.tb_shortcut.value.clone().into());
        self.window
            .set_tb_shortcut_preview(state.tb_shortcut.preview.clone().into());
        self.window
            .set_tb_shortcut_awaiting_final_key(state.tb_shortcut.awaiting_final_key);
        self.window
            .set_tb_shortcut_error_text(state.tb_shortcut.error_text.clone().into());
        self.window
            .set_dp_shortcut(state.dp_shortcut.value.clone().into());
        self.window
            .set_dp_shortcut_preview(state.dp_shortcut.preview.clone().into());
        self.window
            .set_dp_shortcut_awaiting_final_key(state.dp_shortcut.awaiting_final_key);
        self.window
            .set_dp_shortcut_error_text(state.dp_shortcut.error_text.clone().into());
        self.window
            .set_hdmi_shortcut(state.hdmi_shortcut.value.clone().into());
        self.window
            .set_hdmi_shortcut_preview(state.hdmi_shortcut.preview.clone().into());
        self.window
            .set_hdmi_shortcut_awaiting_final_key(state.hdmi_shortcut.awaiting_final_key);
        self.window
            .set_hdmi_shortcut_error_text(state.hdmi_shortcut.error_text.clone().into());
        self.window
            .set_status_text(state.status_text.clone().into());
    }
}

fn wire_callbacks(window: &MainWindow, action_tx: Sender<UiAction>) {
    let tx = action_tx.clone();
    window.on_open_settings(move || {
        let _ = tx.send(UiAction::OpenSettings);
    });

    let tx = action_tx.clone();
    window.on_close_settings(move || {
        let _ = tx.send(UiAction::CloseSettings);
    });

    let tx = action_tx.clone();
    window.on_toggle_autostart(move || {
        let _ = tx.send(UiAction::ToggleAutostart);
    });

    let tx = action_tx.clone();
    window.on_deactivate_tb_shortcut_capture(move || {
        let _ = tx.send(UiAction::DeactivateShortcutCapture(ShortcutTarget::UsbC));
    });

    let tx = action_tx.clone();
    window.on_preview_tb_shortcut(move |text| {
        let _ = tx.send(UiAction::PreviewShortcut {
            target: ShortcutTarget::UsbC,
            preview: text.to_string(),
        });
    });

    let tx = action_tx.clone();
    window.on_commit_tb_shortcut(move |text| {
        let _ = tx.send(UiAction::CommitShortcut {
            target: ShortcutTarget::UsbC,
            shortcut: text.to_string(),
        });
    });

    let tx = action_tx.clone();
    window.on_clear_tb_shortcut(move || {
        let _ = tx.send(UiAction::ClearShortcut(ShortcutTarget::UsbC));
    });

    let tx = action_tx.clone();
    window.on_deactivate_dp_shortcut_capture(move || {
        let _ = tx.send(UiAction::DeactivateShortcutCapture(
            ShortcutTarget::DisplayPort,
        ));
    });

    let tx = action_tx.clone();
    window.on_preview_dp_shortcut(move |text| {
        let _ = tx.send(UiAction::PreviewShortcut {
            target: ShortcutTarget::DisplayPort,
            preview: text.to_string(),
        });
    });

    let tx = action_tx.clone();
    window.on_commit_dp_shortcut(move |text| {
        let _ = tx.send(UiAction::CommitShortcut {
            target: ShortcutTarget::DisplayPort,
            shortcut: text.to_string(),
        });
    });

    let tx = action_tx.clone();
    window.on_clear_dp_shortcut(move || {
        let _ = tx.send(UiAction::ClearShortcut(ShortcutTarget::DisplayPort));
    });

    let tx = action_tx.clone();
    window.on_deactivate_hdmi_shortcut_capture(move || {
        let _ = tx.send(UiAction::DeactivateShortcutCapture(ShortcutTarget::Hdmi));
    });

    let tx = action_tx.clone();
    window.on_preview_hdmi_shortcut(move |text| {
        let _ = tx.send(UiAction::PreviewShortcut {
            target: ShortcutTarget::Hdmi,
            preview: text.to_string(),
        });
    });

    let tx = action_tx.clone();
    window.on_commit_hdmi_shortcut(move |text| {
        let _ = tx.send(UiAction::CommitShortcut {
            target: ShortcutTarget::Hdmi,
            shortcut: text.to_string(),
        });
    });

    let tx = action_tx.clone();
    window.on_clear_hdmi_shortcut(move || {
        let _ = tx.send(UiAction::ClearShortcut(ShortcutTarget::Hdmi));
    });

    window.on_normalize_shortcut_key(move |text| normalize_shortcut_key(&text).into());

    let tx = action_tx.clone();
    window.on_brightness_preview(move |value| {
        let _ = tx.send(UiAction::PreviewFeature {
            feature: crate::app_controller::FeatureId::Brightness,
            value: value.max(0) as u32,
        });
    });

    let tx = action_tx.clone();
    window.on_brightness_commit(move |value| {
        let _ = tx.send(UiAction::CommitFeature {
            feature: crate::app_controller::FeatureId::Brightness,
            value: value.max(0) as u32,
        });
    });

    let tx = action_tx.clone();
    window.on_contrast_preview(move |value| {
        let _ = tx.send(UiAction::PreviewFeature {
            feature: crate::app_controller::FeatureId::Contrast,
            value: value.max(0) as u32,
        });
    });

    let tx = action_tx.clone();
    window.on_contrast_commit(move |value| {
        let _ = tx.send(UiAction::CommitFeature {
            feature: crate::app_controller::FeatureId::Contrast,
            value: value.max(0) as u32,
        });
    });

    let tx = action_tx.clone();
    window.on_select_input(move |value| {
        let route = match value {
            1 => InputRoute::UsbC,
            2 => InputRoute::DisplayPort,
            3 => InputRoute::Hdmi,
            _ => InputRoute::None,
        };
        let _ = tx.send(UiAction::SetInput(route));
    });
}

fn input_route_to_ui_index(route: InputRoute) -> i32 {
    match route {
        InputRoute::None => 0,
        InputRoute::UsbC => 1,
        InputRoute::DisplayPort => 2,
        InputRoute::Hdmi => 3,
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, sync::Once};

    use crate::app_controller::{FeatureId, FeatureState, ShortcutFieldState, UiPane};

    use super::*;

    static SLINT_BACKEND: Once = Once::new();

    fn bridge() -> (UiBridge, mpsc::Receiver<UiAction>) {
        SLINT_BACKEND.call_once(|| {
            std::env::set_var("SLINT_BACKEND", "winit-software");
        });
        let (tx, rx) = mpsc::channel();
        (UiBridge::new(tx).expect("bridge should initialize"), rx)
    }

    #[test]
    fn bridge_maps_state_to_properties_and_callbacks_to_actions() {
        let (bridge, rx) = bridge();
        let state = UiState {
            active_pane: UiPane::Settings,
            monitor_title: "Dell U4025QW".into(),
            input_summary: "USB-C".into(),
            hdr_status: "Windows HDR: multiple displays, mixed".into(),
            brightness: FeatureState {
                value: 42,
                maximum: 120,
                text: "42".into(),
                enabled: true,
            },
            contrast: FeatureState {
                value: 77,
                maximum: 100,
                text: "77".into(),
                enabled: true,
            },
            selected_input: InputRoute::UsbC,
            input_enabled: true,
            autostart_enabled: true,
            status_text: "Monitor diagnostics: capabilities unavailable".into(),
            tb_shortcut: ShortcutFieldState {
                value: "Ctrl+Alt+T".into(),
                preview: "Ctrl+".into(),
                awaiting_final_key: true,
                error_text: "Pick one more key".into(),
            },
            ..UiState::default()
        };

        bridge.apply_state(&state);
        let window = bridge.window();

        assert!(window.get_settings_open());
        assert_eq!(window.get_monitor_title().to_string(), "Dell U4025QW");
        assert_eq!(window.get_input_summary().to_string(), "USB-C");
        assert_eq!(
            window.get_hdr_status().to_string(),
            "Windows HDR: multiple displays, mixed"
        );
        assert_eq!(window.get_brightness_value(), 42);
        assert_eq!(window.get_brightness_maximum(), 120);
        assert!(window.get_brightness_enabled());
        assert_eq!(window.get_contrast_value(), 77);
        assert_eq!(window.get_selected_input(), 1);
        assert!(window.get_input_enabled());
        assert!(window.get_autostart_enabled());
        assert_eq!(window.get_tb_shortcut().to_string(), "Ctrl+Alt+T");
        assert_eq!(window.get_tb_shortcut_preview().to_string(), "Ctrl+");
        assert!(window.get_tb_shortcut_awaiting_final_key());
        assert_eq!(
            window.get_tb_shortcut_error_text().to_string(),
            "Pick one more key"
        );
        assert_eq!(
            window.get_status_text().to_string(),
            "Monitor diagnostics: capabilities unavailable"
        );

        window.invoke_open_settings();
        window.invoke_toggle_autostart();
        window.invoke_select_input(2);
        window.invoke_brightness_preview(63);
        window.invoke_commit_hdmi_shortcut("Ctrl+Alt+H".into());

        assert_eq!(rx.recv().unwrap(), UiAction::OpenSettings);
        assert_eq!(rx.recv().unwrap(), UiAction::ToggleAutostart);
        assert_eq!(
            rx.recv().unwrap(),
            UiAction::SetInput(InputRoute::DisplayPort)
        );
        assert_eq!(
            rx.recv().unwrap(),
            UiAction::PreviewFeature {
                feature: FeatureId::Brightness,
                value: 63,
            }
        );
        assert_eq!(
            rx.recv().unwrap(),
            UiAction::CommitShortcut {
                target: ShortcutTarget::Hdmi,
                shortcut: "Ctrl+Alt+H".into(),
            }
        );
    }
}
