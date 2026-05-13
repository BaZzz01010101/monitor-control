use std::sync::mpsc::Sender;

use anyhow::Context;

use crate::app_controller::{InputRoute, UiAction, UiState};

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
    use std::path::Path;

    #[test]
    fn main_window_uses_the_compact_target_width() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("ui")
                .join("MainWindow.slint"),
        )
        .expect("failed to read MainWindow.slint");

        assert!(source.contains("width: 570px;"));
        assert!(source.contains("height: 480px;"));
    }

    #[test]
    fn main_window_declares_settings_pane_navigation() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("ui")
                .join("MainWindow.slint"),
        )
        .expect("failed to read MainWindow.slint");

        assert!(source.contains("in property <bool> settings-open: false;"));
        assert!(source.contains("callback open-settings();"));
        assert!(source.contains("callback close-settings();"));
        assert!(source.contains("callback toggle-autostart();"));
        assert!(source.contains("in-out property <bool> autostart-enabled: false;"));
        assert!(source.contains("if (root.settings-open) : SettingsPane"));
        assert!(source.contains("if (!root.settings-open) : VerticalBox"));
        assert!(source.contains("autostart-enabled <=> root.autostart-enabled;"));
        assert!(source.contains("StatusBar {"));
    }

    #[test]
    fn input_option_buttons_are_focusable_and_keyboard_activatable() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("ui")
                .join("InputOptionButton.slint"),
        )
        .expect("failed to read InputOptionButton.slint");

        assert!(source.contains("FocusScope"));
        assert!(source.contains("forward-focus:"));
        assert!(source.contains("accessible-role: button;"));
        assert!(source.contains("key-pressed(event)"));
    }

    #[test]
    fn header_card_uses_a_settings_trigger_instead_of_refresh_text() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("ui")
                .join("HeaderCard.slint"),
        )
        .expect("failed to read HeaderCard.slint");

        assert!(source.contains("callback open-settings;"));
        assert!(!source.contains("text: \"Refresh\";"));
        assert!(source.contains("gear_ui.svg"));
        assert!(source.contains("mouse-cursor: pointer;"));
    }

    #[test]
    fn settings_pane_contains_only_navigation_shell() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("ui")
                .join("SettingsPane.slint"),
        )
        .expect("failed to read SettingsPane.slint");

        assert!(source.contains("callback back;"));
        assert!(source.contains("Switch"));
        assert!(source.contains("in-out property <bool> autostart-enabled: false;"));
        assert!(source.contains("toggle-autostart"));
        assert!(source.contains("General"));
        assert!(source.contains("Start with Windows"));
        assert!(source.contains("height: content.preferred-height;"));
        assert!(source.contains("checked <=> root.autostart-enabled;"));
        assert!(!source.contains("Autostart"));
        assert!(!source.contains("text: \"Settings\";"));
        assert!(source.contains("arrow_left_ui.svg"));
        assert!(source.contains("x: 16px;"));
        assert!(source.contains("y: 16px;"));
        assert!(source.contains("width: 100%;"));
        assert!(source.contains("mouse-cursor: pointer;"));
        assert!(source.contains("background: transparent;"));
        assert!(
            source.contains("color: touch.has-hover || focus-scope.has-focus ? #0a365e : #2563eb;")
        );
        assert!(!source.contains("#eaf3ff"));
    }
}
