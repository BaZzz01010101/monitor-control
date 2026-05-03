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
        self.window.set_monitor_title(state.monitor_title.clone().into());
        self.window.set_input_summary(state.input_summary.clone().into());
        self.window.set_hdr_status(state.hdr_status.clone().into());
        self.window.set_brightness_value(state.brightness.value as i32);
        self.window
            .set_brightness_maximum(state.brightness.maximum as i32);
        self.window.set_brightness_enabled(state.brightness.enabled);
        self.window.set_contrast_value(state.contrast.value as i32);
        self.window.set_contrast_maximum(state.contrast.maximum as i32);
        self.window.set_contrast_enabled(state.contrast.enabled);
        self.window
            .set_selected_input(input_route_to_ui_index(state.selected_input));
        self.window.set_input_enabled(state.input_enabled);
        self.window.set_status_text(state.status_text.clone().into());
    }
}

fn wire_callbacks(window: &MainWindow, action_tx: Sender<UiAction>) {
    let tx = action_tx.clone();
    window.on_refresh(move || {
        let _ = tx.send(UiAction::Refresh);
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

    window.on_select_input(move |value| {
        let route = match value {
            1 => InputRoute::UsbC,
            2 => InputRoute::DisplayPort,
            3 => InputRoute::Hdmi,
            _ => InputRoute::None,
        };
        let _ = action_tx.send(UiAction::SetInput(route));
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
