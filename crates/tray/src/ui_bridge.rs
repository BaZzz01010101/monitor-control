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
        assert!(source.contains("in-out property <string> tb-shortcut: \"None\";"));
        assert!(source.contains("in property <string> tb-shortcut-preview: \"\";"));
        assert!(source.contains("in property <bool> tb-shortcut-awaiting-final-key: false;"));
        assert!(source.contains("in property <string> tb-shortcut-error-text: \"\";"));
        assert!(source.contains("in-out property <string> dp-shortcut: \"None\";"));
        assert!(source.contains("in property <string> dp-shortcut-preview: \"\";"));
        assert!(source.contains("in property <bool> dp-shortcut-awaiting-final-key: false;"));
        assert!(source.contains("in property <string> dp-shortcut-error-text: \"\";"));
        assert!(source.contains("in-out property <string> hdmi-shortcut: \"None\";"));
        assert!(source.contains("in property <string> hdmi-shortcut-preview: \"\";"));
        assert!(source.contains("in property <bool> hdmi-shortcut-awaiting-final-key: false;"));
        assert!(source.contains("in property <string> hdmi-shortcut-error-text: \"\";"));
        assert!(source.contains("if (root.settings-open) : SettingsPane"));
        assert!(source.contains("if (!root.settings-open) : VerticalBox"));
        assert!(source.contains("autostart-enabled <=> root.autostart-enabled;"));
        assert!(source.contains("tb-shortcut <=> root.tb-shortcut;"));
        assert!(source.contains("tb-shortcut-preview: root.tb-shortcut-preview;"));
        assert!(
            source.contains("tb-shortcut-awaiting-final-key: root.tb-shortcut-awaiting-final-key;")
        );
        assert!(source.contains("tb-shortcut-error-text: root.tb-shortcut-error-text;"));
        assert!(source.contains("dp-shortcut <=> root.dp-shortcut;"));
        assert!(source.contains("dp-shortcut-preview: root.dp-shortcut-preview;"));
        assert!(
            source.contains("dp-shortcut-awaiting-final-key: root.dp-shortcut-awaiting-final-key;")
        );
        assert!(source.contains("dp-shortcut-error-text: root.dp-shortcut-error-text;"));
        assert!(source.contains("hdmi-shortcut <=> root.hdmi-shortcut;"));
        assert!(source.contains("hdmi-shortcut-preview: root.hdmi-shortcut-preview;"));
        assert!(source
            .contains("hdmi-shortcut-awaiting-final-key: root.hdmi-shortcut-awaiting-final-key;"));
        assert!(source.contains("hdmi-shortcut-error-text: root.hdmi-shortcut-error-text;"));
        assert!(source.contains("callback deactivate-tb-shortcut-capture();"));
        assert!(source.contains("callback deactivate-dp-shortcut-capture();"));
        assert!(source.contains("callback deactivate-hdmi-shortcut-capture();"));
        assert!(source.contains("callback normalize-shortcut-key(string) -> string;"));
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
        assert!(source.contains("in-out property <string> tb-shortcut: \"None\";"));
        assert!(source.contains("in property <string> tb-shortcut-error-text: \"\";"));
        assert!(source.contains("in-out property <string> dp-shortcut: \"None\";"));
        assert!(source.contains("in property <string> dp-shortcut-error-text: \"\";"));
        assert!(source.contains("in-out property <string> hdmi-shortcut: \"None\";"));
        assert!(source.contains("in property <string> hdmi-shortcut-error-text: \"\";"));
        assert!(source.contains("toggle-autostart"));
        assert!(source.contains("General"));
        assert!(source.contains("Shortcuts"));
        assert!(source.contains("Switch to TB"));
        assert!(source.contains("Switch to DP"));
        assert!(source.contains("Switch to HDMI"));
        assert!(source.contains("ShortcutCaptureField"));
        assert!(source.contains("value <=> root.tb-shortcut;"));
        assert!(source.contains("value <=> root.dp-shortcut;"));
        assert!(source.contains("value <=> root.hdmi-shortcut;"));
        assert!(source.contains("preview-text: root.tb-shortcut-preview;"));
        assert!(source.contains("preview-text: root.dp-shortcut-preview;"));
        assert!(source.contains("preview-text: root.hdmi-shortcut-preview;"));
        assert!(source.contains("awaiting-final-key: root.tb-shortcut-awaiting-final-key;"));
        assert!(source.contains("awaiting-final-key: root.dp-shortcut-awaiting-final-key;"));
        assert!(source.contains("awaiting-final-key: root.hdmi-shortcut-awaiting-final-key;"));
        assert!(source.contains("root.deactivate-tb-shortcut-capture();"));
        assert!(source.contains("root.deactivate-dp-shortcut-capture();"));
        assert!(source.contains("root.deactivate-hdmi-shortcut-capture();"));
        assert!(source.contains("root.commit-tb-shortcut(text);"));
        assert!(source.contains("root.commit-dp-shortcut(text);"));
        assert!(source.contains("normalize-shortcut-key(text) => {"));
        assert!(source.contains("return root.normalize-shortcut-key(text);"));
        assert!(source.contains("accessible-name: \"Switch to TB shortcut\";"));
        assert!(source.contains("accessible-name: \"Switch to DP shortcut\";"));
        assert!(source.contains("accessible-name: \"Switch to HDMI shortcut\";"));
        assert!(source.contains("text: root.tb-shortcut-error-text;"));
        assert!(source.contains("text: root.dp-shortcut-error-text;"));
        assert!(source.contains("text: root.hdmi-shortcut-error-text;"));
        assert!(source.contains("Start with Windows"));
        assert!(source.contains("root-layout := VerticalLayout"));
        assert!(source.contains("checked <=> root.autostart-enabled;"));
        assert!(!source.contains("Autostart"));
        assert!(!source.contains("text: \"Settings\";"));
        assert!(source.contains("arrow_left_ui.svg"));
        assert!(source.contains("ScrollView"));
        assert!(source.contains("header := HorizontalLayout"));
        assert!(source.contains("scroll := ScrollView"));
        assert!(source.contains("body := VerticalBox"));
        assert!(source.contains("focus-on-tab-navigation: false;"));
        assert!(source.contains("init => {"));
        assert!(source.contains("pane-focus.focus();"));
        assert!(source.contains("viewport-width: self.visible-width;"));
        assert!(source.contains("padding: 16px;"));
        assert!(source.contains("padding-top: 12px;"));
        assert!(source.contains("mouse-cursor: pointer;"));
        assert!(source.contains("background: transparent;"));
        assert!(source
            .contains("focus-scope := FocusScope {\n        width: 0px;\n        height: 0px;"));
        assert!(
            source.contains("color: touch.has-hover || focus-scope.has-focus ? #0a365e : #2563eb;")
        );
        assert!(source.contains("key-pressed(event) => {"));
        assert!(source.contains("event.text == Key.Escape"));
        assert!(source.contains("root.back();"));
        assert!(!source.contains("x: 16px;"));
        assert!(!source.contains("y: 16px;"));
        assert!(!source.contains("#eaf3ff"));
    }

    #[test]
    fn shortcut_capture_field_declares_keyboard_capture_behavior() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("ui")
                .join("ShortcutCaptureField.slint"),
        )
        .expect("failed to read ShortcutCaptureField.slint");

        assert!(source.contains("in-out property <string> value: \"None\";"));
        assert!(source.contains("in property <string> preview-text: \"\";"));
        assert!(source.contains("in property <bool> awaiting-final-key: false;"));
        assert!(source.contains("callback deactivate-capture();"));
        assert!(source.contains("callback preview-shortcut(string);"));
        assert!(source.contains("callback commit-shortcut(string);"));
        assert!(source.contains("callback clear-shortcut();"));
        assert!(source.contains("callback normalize-shortcut-key(string) -> string;"));
        assert!(source.contains("FocusScope"));
        assert!(source.contains("changed has-focus => {"));
        assert!(source.contains("key-pressed(event)"));
        assert!(source.contains("key-released(event)"));
        assert!(source.contains("focus-lost(reason)"));
        assert!(source.contains("border-width: focus-scope.has-focus ? 2px : 1px;"));
        assert!(source.contains("focus-scope.has-focus ? #1d4ed8"));
        assert!(source.contains("Key.Return"));
        assert!(source.contains("Key.Backspace"));
        assert!(source.contains("Key.Escape"));
        assert!(source.contains("root.deactivate-capture();"));
        assert!(source.contains("root.preview-shortcut("));
        assert!(source.contains("root.commit-shortcut("));
        assert!(source.contains("root.clear-shortcut();"));
        assert!(source.contains("root.normalize-shortcut-key(text)"));
        assert!(source.contains("event.modifiers.meta"));
        assert!(source.contains("event.text == Key.Space"));
        assert!(source.contains("event.text == Key.Tab"));
        assert!(source.contains("event.text == Key.Backtab"));
        assert!(source.contains("Alt+Shift"));
        assert!(source.contains("Ctrl+Shift"));
        assert!(source.contains("function committed-modifier-prefix("));
        assert!(source.contains("if prefix == \"\" {"));
        assert!(source.contains("root.preview-text"));
        assert!(!source.contains("callback activate-capture();"));
        assert!(!source.contains("callback cancel-capture();"));
        assert!(!source.contains("root.cancel-capture();"));
        assert!(!source.contains("clear-focus();"));
        assert!(!source.contains("LineEdit"));
        assert!(!source.contains("TextInput"));
    }
}
