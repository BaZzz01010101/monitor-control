extern crate native_windows_gui as nwg;

use std::{
    cell::RefCell,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dell_controller_core::{
    ddc::VcpCode, enumerate_monitors, hdr, profiles::profile_for_model, startup, CommandQueue,
    RetryPolicy, WindowsDdcBackend, WindowsMonitor,
};
use nwg::NativeUi;
use windows::Win32::{
    Foundation::RECT as WinRect,
    UI::WindowsAndMessaging::{GetClientRect, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN},
};

mod debug_ui;
mod monitor_text;
mod tray_events;
mod ui_layout;
mod value_controls;

use debug_ui::{capture_window_to_bmp, debug_ui_output_path, launch_mode_from_args, LaunchMode};
use monitor_text::{compact_input_label, monitor_heading};
use tray_events::{TrayAction, TrayClickTracker};
use ui_layout::{
    body_font_spec, compute_layout, hdr_font_spec, heading_font_spec, measure_text,
    section_font_spec, Rect, TextMetrics, UiMetrics, FONT_POINT_SIZE, HDR_FONT_POINT_SIZE,
    HEADING_FONT_POINT_SIZE, SECTION_TITLE_FONT_POINT_SIZE,
};
use value_controls::{
    adjusted_feature_value, key_feature_delta, parse_feature_value, wheel_feature_delta,
    WriteThrottle,
};

const BRIGHTNESS_CODE: u8 = 0x10;
const CONTRAST_CODE: u8 = 0x12;
const INPUT_CODE: u8 = 0x60;
const INPUT_DP_VALUE: u32 = 0x0F;
const INPUT_USB_C_VALUE: u32 = 0x19;
const INPUT_HDMI_VALUE: u32 = 0x11;
const LIVE_WRITE_INTERVAL_MS: u64 = 140;
const DEBUG_UI_CAPTURE_DELAY_MS: u64 = 2_000;
const INITIAL_WINDOW_SIZE: (i32, i32) = (813, 489);
const WINDOW_BG: [u8; 3] = [247, 250, 253];
const PANEL_BG: [u8; 3] = [252, 253, 255];
const BUTTON_CONTENT_BG: [u8; 3] = [255, 255, 255];
const SEPARATOR_BG: [u8; 3] = [214, 214, 214];
const DISPLAY_TEXT_COLOR: [u8; 3] = [26, 26, 26];

#[derive(Default)]
pub struct TrayApp {
    message_window: nwg::MessageWindow,
    control_window: nwg::Window,
    heading_font: nwg::Font,
    hdr_font: nwg::Font,
    section_font: nwg::Font,
    icon: nwg::Icon,
    monitor_bitmap: nwg::Bitmap,
    refresh_bitmap: nwg::Bitmap,
    brightness_bitmap: nwg::Bitmap,
    contrast_bitmap: nwg::Bitmap,
    thunderbolt_bitmap: nwg::Bitmap,
    displayport_bitmap: nwg::Bitmap,
    hdmi_bitmap: nwg::Bitmap,
    indicator_unselected_bitmap: nwg::Bitmap,
    indicator_selected_bitmap: nwg::Bitmap,
    arrow_up_bitmap: nwg::Bitmap,
    arrow_down_bitmap: nwg::Bitmap,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    open_item: nwg::MenuItem,
    refresh_item: nwg::MenuItem,
    autostart_item: nwg::MenuItem,
    exit_item: nwg::MenuItem,
    live_write_timer: nwg::AnimationTimer,
    window_background: nwg::ImageFrame,
    header_frame: nwg::Frame,
    header_fill: nwg::ImageFrame,
    monitor_preview: nwg::ImageFrame,
    monitor_label: nwg::RichLabel,
    header_separator: nwg::ImageFrame,
    input_summary_label: nwg::RichLabel,
    hdr_label: nwg::RichLabel,
    refresh_button: nwg::Button,
    refresh_icon: nwg::ImageFrame,
    refresh_text: nwg::Label,
    picture_title: nwg::RichLabel,
    picture_frame: nwg::Frame,
    picture_fill: nwg::ImageFrame,
    picture_separator: nwg::ImageFrame,
    brightness_icon: nwg::ImageFrame,
    brightness_label: nwg::RichLabel,
    brightness_slider: nwg::TrackBar,
    brightness_value_frame: nwg::Frame,
    brightness_value: nwg::TextInput,
    brightness_down: nwg::Button,
    brightness_up: nwg::Button,
    contrast_icon: nwg::ImageFrame,
    contrast_label: nwg::RichLabel,
    contrast_slider: nwg::TrackBar,
    contrast_value_frame: nwg::Frame,
    contrast_value: nwg::TextInput,
    contrast_down: nwg::Button,
    contrast_up: nwg::Button,
    input_title: nwg::RichLabel,
    input_frame: nwg::Frame,
    input_fill: nwg::ImageFrame,
    input_tb_card: nwg::CheckBox,
    input_tb_radio: nwg::ImageFrame,
    input_tb_icon: nwg::ImageFrame,
    input_tb_label: nwg::Label,
    input_dp_card: nwg::CheckBox,
    input_dp_radio: nwg::ImageFrame,
    input_dp_icon: nwg::ImageFrame,
    input_dp_label: nwg::Label,
    input_hdmi_card: nwg::CheckBox,
    input_hdmi_radio: nwg::ImageFrame,
    input_hdmi_icon: nwg::ImageFrame,
    input_hdmi_label: nwg::Label,
    status_label: nwg::RichLabel,
    tray_clicks: RefCell<TrayClickTracker>,
    syncing_controls: RefCell<bool>,
    live_write_timer_running: RefCell<bool>,
    brightness_throttle: RefCell<WriteThrottle>,
    contrast_throttle: RefCell<WriteThrottle>,
    launch_mode: LaunchMode,
    debug_ui_output_path: Option<PathBuf>,
    monitors: RefCell<Vec<WindowsMonitor>>,
}

impl TrayApp {
    fn active_monitor(&self) -> Option<WindowsMonitor> {
        self.monitors.borrow().first().cloned()
    }

    fn show_control_window(&self) {
        self.refresh_status();
        self.control_window.set_visible(true);
        self.control_window.set_focus();
    }

    fn hide_control_window(&self) {
        self.control_window.set_visible(false);
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn start_debug_ui_mode(&self) {
        if self.launch_mode == LaunchMode::DebugUi {
            self.show_control_window();
            self.log_debug_ui_state();
            self.spawn_debug_ui_capture();
        }
    }

    fn handle_tray_left_up(&self) {
        if self.tray_clicks.borrow_mut().left_up_at(now_ms()) == Some(TrayAction::OpenWindow) {
            self.show_control_window();
        }
    }

    fn reload_monitors(&self) {
        match enumerate_monitors() {
            Ok(monitors) => {
                *self.monitors.borrow_mut() = monitors;
                self.refresh_status();
                self.set_status("Monitors refreshed");
            }
            Err(error) => {
                *self.monitors.borrow_mut() = Vec::new();
                self.refresh_status();
                self.set_status(&format!("Monitor refresh failed: {error}"));
            }
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
            set_display_text(&self.monitor_label, "No DDC/CI monitor detected");
            set_display_text(&self.input_summary_label, "Input");
            set_display_text(&self.hdr_label, "Windows HDR: unavailable");
            self.set_feature_unavailable(&self.brightness_value, &self.brightness_slider);
            self.set_feature_unavailable(&self.contrast_value, &self.contrast_slider);
            self.set_picture_controls_enabled(false);
            self.set_input_controls_enabled(false);
            self.set_input_selection(0);
            self.apply_layout();
            return;
        };

        set_display_text(
            &self.monitor_label,
            &monitor_heading(&monitor.info.description, monitor.info.model.as_deref()),
        );
        self.set_picture_controls_enabled(true);
        self.set_input_controls_enabled(true);

        let queue: CommandQueue<_> =
            CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
        self.set_feature_controls(
            &queue,
            BRIGHTNESS_CODE,
            &self.brightness_value,
            &self.brightness_slider,
        );
        self.set_feature_controls(
            &queue,
            CONTRAST_CODE,
            &self.contrast_value,
            &self.contrast_slider,
        );
        self.set_input_summary(&monitor, &queue);
        self.refresh_hdr();
        self.apply_layout();
    }

    fn apply_layout(&self) {
        let metrics = UiMetrics::default();
        let layout = compute_layout(&metrics, &self.layout_text_metrics());
        self.control_window
            .set_size(layout.client_size.width, layout.client_size.height);
        let (outer_width, outer_height) = self.control_window.size();
        center_window(&self.control_window, outer_width, outer_height);

        place_absolute(
            &self.window_background,
            Rect::new(0, 0, layout.client_size.width, layout.client_size.height),
        );
        place_absolute(&self.header_frame, layout.header_frame);
        place_relative(
            &self.header_fill,
            inset_rect(layout.header_frame, 1),
            layout.header_frame,
        );
        place_relative(
            &self.monitor_preview,
            layout.monitor_preview,
            layout.header_frame,
        );
        place_relative(
            &self.monitor_label,
            layout.monitor_title,
            inset_rect(layout.header_frame, 1),
        );
        place_relative(
            &self.header_separator,
            layout.header_separator,
            layout.header_frame,
        );
        place_relative(
            &self.input_summary_label,
            layout.input_summary,
            inset_rect(layout.header_frame, 1),
        );
        place_relative(
            &self.hdr_label,
            layout.hdr_status,
            inset_rect(layout.header_frame, 1),
        );
        place_relative(
            &self.refresh_button,
            layout.refresh_button,
            layout.header_frame,
        );
        place_relative(
            &self.refresh_icon,
            layout.refresh_icon,
            layout.refresh_button,
        );
        place_relative(
            &self.refresh_text,
            layout.refresh_text,
            layout.refresh_button,
        );

        place_absolute(&self.picture_title, layout.picture_title);
        place_absolute(&self.picture_frame, layout.picture_frame);
        place_relative(
            &self.picture_fill,
            inset_rect(layout.picture_frame, 1),
            layout.picture_frame,
        );
        place_relative(
            &self.brightness_icon,
            layout.brightness_icon,
            layout.picture_frame,
        );
        place_relative(
            &self.brightness_label,
            layout.brightness_label,
            inset_rect(layout.picture_frame, 1),
        );
        place_relative(
            &self.brightness_slider,
            layout.brightness_slider,
            layout.picture_frame,
        );
        place_relative(
            &self.brightness_value_frame,
            layout.brightness_value,
            layout.picture_frame,
        );
        place_relative(
            &self.brightness_value,
            inset_rect(layout.brightness_value, 1),
            layout.brightness_value,
        );
        place_relative(
            &self.brightness_up,
            layout.brightness_up,
            layout.picture_frame,
        );
        place_relative(
            &self.brightness_down,
            layout.brightness_down,
            layout.picture_frame,
        );
        place_relative(
            &self.contrast_icon,
            layout.contrast_icon,
            layout.picture_frame,
        );
        place_relative(
            &self.contrast_label,
            layout.contrast_label,
            inset_rect(layout.picture_frame, 1),
        );
        place_relative(
            &self.contrast_slider,
            layout.contrast_slider,
            layout.picture_frame,
        );
        place_relative(
            &self.contrast_value_frame,
            layout.contrast_value,
            layout.picture_frame,
        );
        place_relative(
            &self.contrast_value,
            inset_rect(layout.contrast_value, 1),
            layout.contrast_value,
        );
        place_relative(&self.contrast_up, layout.contrast_up, layout.picture_frame);
        place_relative(
            &self.contrast_down,
            layout.contrast_down,
            layout.picture_frame,
        );
        place_relative(
            &self.picture_separator,
            layout.picture_separator,
            layout.picture_frame,
        );

        place_absolute(&self.input_title, layout.input_title);
        place_absolute(&self.input_frame, layout.input_frame);
        place_relative(
            &self.input_fill,
            inset_rect(layout.input_frame, 1),
            layout.input_frame,
        );
        place_relative(
            &self.input_tb_card,
            layout.input_tb_card,
            layout.input_frame,
        );
        place_relative(
            &self.input_dp_card,
            layout.input_dp_card,
            layout.input_frame,
        );
        place_relative(
            &self.input_hdmi_card,
            layout.input_hdmi_card,
            layout.input_frame,
        );

        place_relative(
            &self.input_tb_radio,
            layout.input_tb_radio,
            layout.input_tb_card,
        );
        place_relative(
            &self.input_tb_icon,
            layout.input_tb_icon,
            layout.input_tb_card,
        );
        place_relative(
            &self.input_tb_label,
            layout.input_tb_label,
            layout.input_tb_card,
        );
        place_relative(
            &self.input_dp_radio,
            layout.input_dp_radio,
            layout.input_dp_card,
        );
        place_relative(
            &self.input_dp_icon,
            layout.input_dp_icon,
            layout.input_dp_card,
        );
        place_relative(
            &self.input_dp_label,
            layout.input_dp_label,
            layout.input_dp_card,
        );
        place_relative(
            &self.input_hdmi_radio,
            layout.input_hdmi_radio,
            layout.input_hdmi_card,
        );
        place_relative(
            &self.input_hdmi_icon,
            layout.input_hdmi_icon,
            layout.input_hdmi_card,
        );
        place_relative(
            &self.input_hdmi_label,
            layout.input_hdmi_label,
            layout.input_hdmi_card,
        );

        place_absolute(&self.status_label, layout.status_label);
    }

    fn layout_text_metrics(&self) -> TextMetrics {
        TextMetrics {
            monitor_title: measure_text(&self.monitor_label.text(), heading_font_spec()),
            input_summary: measure_text(&self.input_summary_label.text(), body_font_spec()),
            hdr_status: measure_text(&self.hdr_label.text(), hdr_font_spec()),
            refresh_label: measure_text("Refresh", body_font_spec()),
            picture_title: measure_text(&self.picture_title.text(), section_font_spec()),
            input_title: measure_text(&self.input_title.text(), section_font_spec()),
            brightness_label: measure_text(&self.brightness_label.text(), body_font_spec()),
            contrast_label: measure_text(&self.contrast_label.text(), body_font_spec()),
            status: measure_text(&self.status_label.text(), body_font_spec()),
            input_tb_label: measure_text(&self.input_tb_label.text(), body_font_spec()),
            input_dp_label: measure_text(&self.input_dp_label.text(), body_font_spec()),
            input_hdmi_label: measure_text(&self.input_hdmi_label.text(), body_font_spec()),
        }
    }

    fn set_feature_controls(
        &self,
        queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
        code: u8,
        target: &nwg::TextInput,
        slider: &nwg::TrackBar,
    ) {
        match queue.get(VcpCode::new(code)) {
            Ok(feature) => {
                let maximum = feature.maximum.max(1) as usize;
                let current = feature.current.min(feature.maximum) as usize;
                self.syncing_controls.replace(true);
                target.set_text(&feature.current.to_string());
                target.set_enabled(true);
                slider.set_enabled(true);
                slider.set_range_min(0);
                slider.set_range_max(maximum);
                slider.set_pos(current.min(maximum));
                self.syncing_controls.replace(false);
            }
            Err(_) => self.set_feature_unavailable(target, slider),
        }
    }

    fn set_feature_unavailable(&self, target: &nwg::TextInput, slider: &nwg::TrackBar) {
        self.syncing_controls.replace(true);
        target.set_text("n/a");
        target.set_enabled(false);
        slider.set_enabled(false);
        slider.set_pos(0);
        self.syncing_controls.replace(false);
    }

    fn set_input_summary(
        &self,
        monitor: &WindowsMonitor,
        queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
    ) {
        match queue.get(VcpCode::new(INPUT_CODE)) {
            Ok(feature) => {
                let profile = profile_for_model(monitor.info.model.as_deref());
                let label = profile
                    .as_ref()
                    .and_then(|profile| profile.control("input"))
                    .and_then(|control| control.value_label(feature.current))
                    .unwrap_or("unknown");
                set_display_text(&self.input_summary_label, compact_input_label(label));
                self.set_input_selection(feature.current);
            }
            Err(_) => {
                set_display_text(&self.input_summary_label, "Input");
                self.set_input_selection(0);
            }
        }
    }

    fn set_input_selection(&self, value: u32) {
        let value = value & 0xFF;
        self.syncing_controls.replace(true);
        self.set_input_indicator(&self.input_dp_radio, value == INPUT_DP_VALUE);
        self.set_input_indicator(&self.input_tb_radio, value == INPUT_USB_C_VALUE);
        self.set_input_indicator(&self.input_hdmi_radio, value == INPUT_HDMI_VALUE);
        self.input_dp_card
            .set_check_state(nwg::CheckBoxState::Unchecked);
        self.input_tb_card
            .set_check_state(nwg::CheckBoxState::Unchecked);
        self.input_hdmi_card
            .set_check_state(nwg::CheckBoxState::Unchecked);
        if value == INPUT_DP_VALUE {
            self.input_dp_card.set_focus();
        } else if value == INPUT_USB_C_VALUE {
            self.input_tb_card.set_focus();
        } else if value == INPUT_HDMI_VALUE {
            self.input_hdmi_card.set_focus();
        }
        self.syncing_controls.replace(false);
    }

    fn set_input_indicator(&self, target: &nwg::ImageFrame, selected: bool) {
        target.set_bitmap(Some(if selected {
            &self.indicator_selected_bitmap
        } else {
            &self.indicator_unselected_bitmap
        }));
    }

    fn set_picture_controls_enabled(&self, enabled: bool) {
        self.brightness_slider.set_enabled(enabled);
        self.contrast_slider.set_enabled(enabled);
        self.brightness_value.set_enabled(enabled);
        self.contrast_value.set_enabled(enabled);
        self.brightness_down.set_enabled(enabled);
        self.brightness_up.set_enabled(enabled);
        self.contrast_down.set_enabled(enabled);
        self.contrast_up.set_enabled(enabled);
    }

    fn set_input_controls_enabled(&self, enabled: bool) {
        self.input_tb_card.set_enabled(enabled);
        self.input_dp_card.set_enabled(enabled);
        self.input_hdmi_card.set_enabled(enabled);
        self.input_tb_radio.set_enabled(enabled);
        self.input_dp_radio.set_enabled(enabled);
        self.input_hdmi_radio.set_enabled(enabled);
        self.input_tb_icon.set_enabled(enabled);
        self.input_dp_icon.set_enabled(enabled);
        self.input_hdmi_icon.set_enabled(enabled);
        self.input_tb_label.set_enabled(enabled);
        self.input_dp_label.set_enabled(enabled);
        self.input_hdmi_label.set_enabled(enabled);
    }

    fn refresh_hdr(&self) {
        match hdr::hdr_states() {
            Ok(states) if !states.is_empty() => {
                let state = &states[0];
                set_display_text(
                    &self.hdr_label,
                    &format!(
                        "Windows HDR: {}",
                        if state.enabled {
                            "On"
                        } else if state.supported {
                            "Available, off"
                        } else {
                            "Not supported"
                        }
                    ),
                );
            }
            _ => set_display_text(&self.hdr_label, "Windows HDR: unavailable"),
        }
    }

    fn preview_slider_value(&self, code: u8, target: &nwg::TextInput, slider: &nwg::TrackBar) {
        if *self.syncing_controls.borrow() {
            return;
        }
        let value = slider.pos() as u32;
        target.set_text(&value.to_string());
        self.schedule_live_vcp_value(code, value);
    }

    fn commit_slider_value(&self, code: u8, target: &nwg::TextInput, slider: &nwg::TrackBar) {
        if *self.syncing_controls.borrow() {
            return;
        }
        let value = slider.pos() as u32;
        target.set_text(&value.to_string());
        self.force_write_vcp_value(code, value);
    }

    fn preview_text_value(&self, code: u8, target: &nwg::TextInput, slider: &nwg::TrackBar) {
        if *self.syncing_controls.borrow() {
            return;
        }
        if let Some(value) = parse_feature_value(&target.text(), slider.range_max() as u32) {
            self.syncing_controls.replace(true);
            slider.set_pos(value as usize);
            self.syncing_controls.replace(false);
            self.schedule_live_vcp_value(code, value);
        }
    }

    fn commit_text_value(&self, code: u8, target: &nwg::TextInput, slider: &nwg::TrackBar) {
        if *self.syncing_controls.borrow() {
            return;
        }
        match parse_feature_value(&target.text(), slider.range_max() as u32) {
            Some(value) => {
                self.syncing_controls.replace(true);
                target.set_text(&value.to_string());
                slider.set_pos(value as usize);
                self.syncing_controls.replace(false);
                self.force_write_vcp_value(code, value);
            }
            None => target.set_text(&slider.pos().to_string()),
        }
    }

    fn adjust_value_control(
        &self,
        code: u8,
        delta: i32,
        target: &nwg::TextInput,
        slider: &nwg::TrackBar,
    ) {
        if *self.syncing_controls.borrow() {
            return;
        }
        let maximum = slider.range_max() as u32;
        let current = parse_feature_value(&target.text(), maximum).unwrap_or(slider.pos() as u32);
        let value = adjusted_feature_value(current, maximum, delta);
        self.syncing_controls.replace(true);
        target.set_text(&value.to_string());
        slider.set_pos(value as usize);
        self.syncing_controls.replace(false);
        self.force_write_vcp_value(code, value);
    }

    fn handle_value_key(
        &self,
        code: u8,
        target: &nwg::TextInput,
        slider: &nwg::TrackBar,
        key: u32,
    ) {
        if let Some(delta) = key_feature_delta(key) {
            self.adjust_value_control(code, delta, target, slider);
        }
    }

    fn handle_value_wheel(
        &self,
        code: u8,
        target: &nwg::TextInput,
        slider: &nwg::TrackBar,
        wheel_delta: i32,
    ) {
        if let Some(delta) = wheel_feature_delta(wheel_delta) {
            self.adjust_value_control(code, delta, target, slider);
        }
    }

    fn schedule_live_vcp_value(&self, code: u8, value: u32) {
        let now = now_ms();
        let send_now = {
            let mut throttle = self.throttle_cell(code).borrow_mut();
            throttle.schedule(now, value)
        };

        if let Some(value) = send_now {
            self.write_vcp_value(code, value);
        }

        if self.any_pending_writes() {
            self.start_live_write_timer();
        } else {
            self.stop_live_write_timer();
        }
    }

    fn force_write_vcp_value(&self, code: u8, value: u32) {
        let now = now_ms();
        let value = {
            let mut throttle = self.throttle_cell(code).borrow_mut();
            throttle.force(now, value)
        };
        self.write_vcp_value(code, value);

        if self.any_pending_writes() {
            self.start_live_write_timer();
        } else {
            self.stop_live_write_timer();
        }
    }

    fn flush_pending_writes(&self) {
        let now = now_ms();
        for code in [BRIGHTNESS_CODE, CONTRAST_CODE] {
            let due = {
                let mut throttle = self.throttle_cell(code).borrow_mut();
                throttle.tick(now)
            };
            if let Some(value) = due {
                self.write_vcp_value(code, value);
            }
        }

        if self.any_pending_writes() {
            self.start_live_write_timer();
        } else {
            self.stop_live_write_timer();
        }
    }

    fn any_pending_writes(&self) -> bool {
        self.brightness_throttle.borrow().has_pending()
            || self.contrast_throttle.borrow().has_pending()
    }

    fn throttle_cell(&self, code: u8) -> &RefCell<WriteThrottle> {
        match code {
            BRIGHTNESS_CODE => &self.brightness_throttle,
            CONTRAST_CODE => &self.contrast_throttle,
            _ => panic!("unsupported throttled VCP code: 0x{code:02X}"),
        }
    }

    fn start_live_write_timer(&self) {
        if !*self.live_write_timer_running.borrow() {
            self.live_write_timer.start();
            self.live_write_timer_running.replace(true);
        }
    }

    fn stop_live_write_timer(&self) {
        if *self.live_write_timer_running.borrow() {
            self.live_write_timer.stop();
            self.live_write_timer_running.replace(false);
        }
    }

    fn write_vcp_value(&self, code: u8, value: u32) {
        let Some(monitor) = self.active_monitor() else {
            self.set_status("No monitor available");
            return;
        };

        let queue: CommandQueue<_> =
            CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
        match queue.set(VcpCode::new(code), value) {
            Ok(()) => self.set_status(&format!("Set 0x{code:02X} to {value}")),
            Err(error) => {
                self.set_status(&format!("Set failed: {error}"));
                self.refresh_status();
            }
        }
    }

    fn set_input(&self, value: u32) {
        if *self.syncing_controls.borrow() {
            return;
        }
        let Some(monitor) = self.active_monitor() else {
            self.set_status("No monitor available");
            return;
        };
        let queue: CommandQueue<_> =
            CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
        match queue.set(VcpCode::new(INPUT_CODE), value) {
            Ok(()) => {
                self.set_status(&format!("Input set to {value:#X}"));
                self.refresh_status();
            }
            Err(error) => self.set_status(&format!("Input switch failed: {error}")),
        }
    }

    fn set_status(&self, message: &str) {
        set_display_text(&self.status_label, message);
    }

    fn spawn_debug_ui_capture(&self) {
        let Some(path) = self.debug_ui_output_path.as_deref() else {
            self.exit();
            return;
        };

        let hwnd = match self
            .control_window
            .handle
            .hwnd()
            .ok_or_else(|| anyhow::anyhow!("failed to get control window handle"))
        {
            Ok(hwnd) => hwnd as usize,
            Err(error) => {
                eprintln!("debug-ui screenshot failed: {error}");
                self.exit();
                return;
            }
        };
        let path = path.to_path_buf();

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(DEBUG_UI_CAPTURE_DELAY_MS));

            let outcome = capture_window_to_bmp(
                windows::Win32::Foundation::HWND(hwnd as *mut core::ffi::c_void),
                &path,
            );

            match outcome {
                Ok(()) => println!("debug-ui screenshot written to {}", path.display()),
                Err(error) => eprintln!("debug-ui screenshot failed: {error}"),
            }

            std::process::exit(0);
        });
    }

    fn log_debug_ui_state(&self) {
        if self.launch_mode != LaunchMode::DebugUi {
            return;
        }

        let text = self.layout_text_metrics();
        let layout = compute_layout(&UiMetrics::default(), &text);
        eprintln!("debug-ui text metrics: {text:?}");
        eprintln!(
            "debug-ui texts: monitor={:?} input_summary={:?} hdr={:?} picture={:?} brightness={:?} contrast={:?} input={:?} status={:?}",
            self.monitor_label.text(),
            self.input_summary_label.text(),
            self.hdr_label.text(),
            self.picture_title.text(),
            self.brightness_label.text(),
            self.contrast_label.text(),
            self.input_title.text(),
            self.status_label.text(),
        );
        eprintln!(
            "debug-ui rects: monitor_title={:?} hdr_status={:?} picture_title={:?} brightness_label={:?} contrast_label={:?} input_title={:?} status={:?}",
            layout.monitor_title,
            layout.hdr_status,
            layout.picture_title,
            layout.brightness_label,
            layout.contrast_label,
            layout.input_title,
            layout.status_label,
        );
        if let Some((client_width, client_height)) = window_client_size(&self.control_window) {
            let (outer_width, outer_height) = self.control_window.size();
            eprintln!(
                "debug-ui sizing: target_client_logical={}x{} actual_client_physical={}x{} actual_window_logical={}x{}",
                layout.client_size.width,
                layout.client_size.height,
                client_width,
                client_height,
                outer_width,
                outer_height,
            );
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u128::from(u64::MAX)) as u64
}

mod ui {
    use std::{cell::RefCell, ops::Deref, rc::Rc};

    use native_windows_gui as nwg;
    use nwg::Event as E;

    use super::*;

    pub struct TrayAppUi {
        inner: Rc<TrayApp>,
        handlers: RefCell<Vec<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<TrayAppUi> for TrayApp {
        fn build_ui(mut data: TrayApp) -> Result<TrayAppUi, nwg::NwgError> {
            let metrics = UiMetrics::default();

            nwg::Icon::builder()
                .source_system(Some(nwg::OemIcon::Information))
                .size(Some((16, 16)))
                .build(&mut data.icon)?;

            build_bitmap(
                &mut data.monitor_bitmap,
                include_bytes!("../assets/icons/monitor_ui.png"),
                (
                    metrics.monitor_preview_box.width,
                    metrics.monitor_preview_box.height,
                ),
            )?;
            build_bitmap(
                &mut data.refresh_bitmap,
                include_bytes!("../assets/icons/refresh_ui.png"),
                (
                    metrics.refresh_icon_box.width,
                    metrics.refresh_icon_box.height,
                ),
            )?;
            build_bitmap(
                &mut data.brightness_bitmap,
                include_bytes!("../assets/icons/brightness_ui.png"),
                (metrics.row_icon_box.width, metrics.row_icon_box.height),
            )?;
            build_bitmap(
                &mut data.contrast_bitmap,
                include_bytes!("../assets/icons/contrast_ui.png"),
                (metrics.row_icon_box.width, metrics.row_icon_box.height),
            )?;
            build_bitmap(
                &mut data.thunderbolt_bitmap,
                include_bytes!("../assets/icons/thunderbolt_ui.png"),
                (
                    metrics.input_tb_icon_box.width,
                    metrics.input_tb_icon_box.height,
                ),
            )?;
            build_bitmap(
                &mut data.displayport_bitmap,
                include_bytes!("../assets/icons/displayport_ui.png"),
                (
                    metrics.input_dp_icon_box.width,
                    metrics.input_dp_icon_box.height,
                ),
            )?;
            build_bitmap(
                &mut data.hdmi_bitmap,
                include_bytes!("../assets/icons/hdmi_ui.png"),
                (
                    metrics.input_hdmi_icon_box.width,
                    metrics.input_hdmi_icon_box.height,
                ),
            )?;
            build_bitmap(
                &mut data.indicator_unselected_bitmap,
                include_bytes!("../assets/icons/indicator_unselected_ui.png"),
                (
                    metrics.input_indicator_box.width,
                    metrics.input_indicator_box.height,
                ),
            )?;
            build_bitmap(
                &mut data.indicator_selected_bitmap,
                include_bytes!("../assets/icons/indicator_selected_ui.png"),
                (
                    metrics.input_indicator_box.width,
                    metrics.input_indicator_box.height,
                ),
            )?;
            build_bitmap(
                &mut data.arrow_up_bitmap,
                include_bytes!("../assets/icons/arrow_up_ui.png"),
                (13, 13),
            )?;
            build_bitmap(
                &mut data.arrow_down_bitmap,
                include_bytes!("../assets/icons/arrow_down_ui.png"),
                (13, 13),
            )?;

            nwg::Font::builder()
                .family("Segoe UI")
                .size_absolute(HEADING_FONT_POINT_SIZE)
                .weight(700)
                .build(&mut data.heading_font)?;
            nwg::Font::builder()
                .family("Segoe UI")
                .size_absolute(HDR_FONT_POINT_SIZE)
                .build(&mut data.hdr_font)?;
            nwg::Font::builder()
                .family("Segoe UI")
                .size_absolute(SECTION_TITLE_FONT_POINT_SIZE)
                .weight(600)
                .build(&mut data.section_font)?;

            nwg::MessageWindow::builder().build(&mut data.message_window)?;

            nwg::Window::builder()
                .size(INITIAL_WINDOW_SIZE)
                .center(true)
                .title("Dell Controller")
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::MINIMIZE_BOX)
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

            nwg::AnimationTimer::builder()
                .parent(&data.control_window)
                .interval(Duration::from_millis(16))
                .active(false)
                .build(&mut data.live_write_timer)?;

            build_surface(&mut data.window_background, &data.control_window, WINDOW_BG)?;

            nwg::Frame::builder()
                .parent(&data.control_window)
                .flags(nwg::FrameFlags::VISIBLE | nwg::FrameFlags::BORDER)
                .build(&mut data.header_frame)?;
            build_surface(&mut data.header_fill, &data.header_frame, PANEL_BG)?;
            build_tinted_image_frame(
                &mut data.monitor_preview,
                &data.header_frame,
                &data.monitor_bitmap,
                PANEL_BG,
            )?;
            build_display_text(
                &mut data.monitor_label,
                &data.header_fill,
                "Detecting monitor...",
                Some(PANEL_BG),
            )?;
            data.monitor_label.set_font(Some(&data.heading_font));
            build_display_text(
                &mut data.input_summary_label,
                &data.header_fill,
                "Input",
                Some(PANEL_BG),
            )?;
            build_display_text(
                &mut data.hdr_label,
                &data.header_fill,
                "Windows HDR: unknown",
                Some(PANEL_BG),
            )?;
            data.hdr_label.set_font(Some(&data.hdr_font));
            build_surface(&mut data.header_separator, &data.header_frame, SEPARATOR_BG)?;
            build_button(&mut data.refresh_button, &data.header_frame, "")?;
            build_tinted_image_frame(
                &mut data.refresh_icon,
                &data.refresh_button,
                &data.refresh_bitmap,
                BUTTON_CONTENT_BG,
            )?;
            build_label(
                &mut data.refresh_text,
                &data.refresh_button,
                "Refresh",
                Some(BUTTON_CONTENT_BG),
            )?;

            build_display_text(
                &mut data.picture_title,
                &data.window_background,
                "Picture",
                Some(WINDOW_BG),
            )?;
            data.picture_title.set_font(Some(&data.section_font));
            nwg::Frame::builder()
                .parent(&data.control_window)
                .flags(nwg::FrameFlags::VISIBLE | nwg::FrameFlags::BORDER)
                .build(&mut data.picture_frame)?;
            build_surface(&mut data.picture_fill, &data.picture_frame, PANEL_BG)?;
            build_surface(
                &mut data.picture_separator,
                &data.picture_frame,
                SEPARATOR_BG,
            )?;
            build_tinted_image_frame(
                &mut data.brightness_icon,
                &data.picture_frame,
                &data.brightness_bitmap,
                PANEL_BG,
            )?;
            build_display_text(
                &mut data.brightness_label,
                &data.picture_fill,
                "Brightness",
                Some(PANEL_BG),
            )?;
            build_trackbar(&mut data.brightness_slider, &data.picture_frame)?;
            nwg::Frame::builder()
                .parent(&data.picture_frame)
                .flags(nwg::FrameFlags::VISIBLE | nwg::FrameFlags::BORDER)
                .build(&mut data.brightness_value_frame)?;
            build_value_input(&mut data.brightness_value, &data.brightness_value_frame)?;
            build_bitmap_button(
                &mut data.brightness_up,
                &data.picture_frame,
                &data.arrow_up_bitmap,
            )?;
            build_bitmap_button(
                &mut data.brightness_down,
                &data.picture_frame,
                &data.arrow_down_bitmap,
            )?;
            build_tinted_image_frame(
                &mut data.contrast_icon,
                &data.picture_frame,
                &data.contrast_bitmap,
                PANEL_BG,
            )?;
            build_display_text(
                &mut data.contrast_label,
                &data.picture_fill,
                "Contrast",
                Some(PANEL_BG),
            )?;
            build_trackbar(&mut data.contrast_slider, &data.picture_frame)?;
            nwg::Frame::builder()
                .parent(&data.picture_frame)
                .flags(nwg::FrameFlags::VISIBLE | nwg::FrameFlags::BORDER)
                .build(&mut data.contrast_value_frame)?;
            build_value_input(&mut data.contrast_value, &data.contrast_value_frame)?;
            build_bitmap_button(
                &mut data.contrast_up,
                &data.picture_frame,
                &data.arrow_up_bitmap,
            )?;
            build_bitmap_button(
                &mut data.contrast_down,
                &data.picture_frame,
                &data.arrow_down_bitmap,
            )?;

            build_display_text(
                &mut data.input_title,
                &data.window_background,
                "KVM / Input",
                Some(WINDOW_BG),
            )?;
            data.input_title.set_font(Some(&data.section_font));
            nwg::Frame::builder()
                .parent(&data.control_window)
                .flags(nwg::FrameFlags::VISIBLE | nwg::FrameFlags::BORDER)
                .build(&mut data.input_frame)?;
            build_surface(&mut data.input_fill, &data.input_frame, PANEL_BG)?;

            build_input_button(&mut data.input_tb_card, &data.input_frame)?;
            build_input_button(&mut data.input_dp_card, &data.input_frame)?;
            build_input_button(&mut data.input_hdmi_card, &data.input_frame)?;

            build_tinted_image_frame(
                &mut data.input_tb_radio,
                &data.input_tb_card,
                &data.indicator_unselected_bitmap,
                BUTTON_CONTENT_BG,
            )?;
            build_tinted_image_frame(
                &mut data.input_tb_icon,
                &data.input_tb_card,
                &data.thunderbolt_bitmap,
                BUTTON_CONTENT_BG,
            )?;
            build_label(
                &mut data.input_tb_label,
                &data.input_tb_card,
                "TB",
                Some(BUTTON_CONTENT_BG),
            )?;

            build_tinted_image_frame(
                &mut data.input_dp_radio,
                &data.input_dp_card,
                &data.indicator_unselected_bitmap,
                BUTTON_CONTENT_BG,
            )?;
            build_tinted_image_frame(
                &mut data.input_dp_icon,
                &data.input_dp_card,
                &data.displayport_bitmap,
                BUTTON_CONTENT_BG,
            )?;
            build_label(
                &mut data.input_dp_label,
                &data.input_dp_card,
                "DP",
                Some(BUTTON_CONTENT_BG),
            )?;

            build_tinted_image_frame(
                &mut data.input_hdmi_radio,
                &data.input_hdmi_card,
                &data.indicator_unselected_bitmap,
                BUTTON_CONTENT_BG,
            )?;
            build_tinted_image_frame(
                &mut data.input_hdmi_icon,
                &data.input_hdmi_card,
                &data.hdmi_bitmap,
                BUTTON_CONTENT_BG,
            )?;
            build_label(
                &mut data.input_hdmi_label,
                &data.input_hdmi_card,
                "HDMI",
                Some(BUTTON_CONTENT_BG),
            )?;

            build_display_text(
                &mut data.status_label,
                &data.window_background,
                "Ready",
                Some(WINDOW_BG),
            )?;

            data.brightness_throttle
                .replace(WriteThrottle::new(LIVE_WRITE_INTERVAL_MS));
            data.contrast_throttle
                .replace(WriteThrottle::new(LIVE_WRITE_INTERVAL_MS));

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
                            E::OnMousePress(nwg::MousePressEvent::MousePressLeftUp)
                                if handle == app.tray =>
                            {
                                app.handle_tray_left_up()
                            }
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
                move |evt, evt_data, handle| {
                    if let Some(app) = control_events.upgrade() {
                        match evt {
                            E::OnWindowClose if handle == app.control_window => {
                                app.hide_control_window()
                            }
                            E::OnButtonClick if handle == app.refresh_button => {
                                app.reload_monitors()
                            }
                            E::OnMousePress(nwg::MousePressEvent::MousePressLeftUp)
                                if handle == app.refresh_icon || handle == app.refresh_text =>
                            {
                                app.reload_monitors()
                            }
                            E::OnButtonClick if handle == app.brightness_down => app
                                .adjust_value_control(
                                    BRIGHTNESS_CODE,
                                    -1,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                ),
                            E::OnButtonClick if handle == app.brightness_up => app
                                .adjust_value_control(
                                    BRIGHTNESS_CODE,
                                    1,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                ),
                            E::OnButtonClick if handle == app.contrast_down => app
                                .adjust_value_control(
                                    CONTRAST_CODE,
                                    -1,
                                    &app.contrast_value,
                                    &app.contrast_slider,
                                ),
                            E::OnButtonClick if handle == app.contrast_up => app
                                .adjust_value_control(
                                    CONTRAST_CODE,
                                    1,
                                    &app.contrast_value,
                                    &app.contrast_slider,
                                ),
                            E::OnHorizontalScroll if handle == app.brightness_slider => app
                                .preview_slider_value(
                                    BRIGHTNESS_CODE,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                ),
                            E::OnHorizontalScroll if handle == app.contrast_slider => app
                                .preview_slider_value(
                                    CONTRAST_CODE,
                                    &app.contrast_value,
                                    &app.contrast_slider,
                                ),
                            E::TrackBarUpdated if handle == app.brightness_slider => app
                                .commit_slider_value(
                                    BRIGHTNESS_CODE,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                ),
                            E::TrackBarUpdated if handle == app.contrast_slider => app
                                .commit_slider_value(
                                    CONTRAST_CODE,
                                    &app.contrast_value,
                                    &app.contrast_slider,
                                ),
                            E::OnTextInput if handle == app.brightness_value => app
                                .preview_text_value(
                                    BRIGHTNESS_CODE,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                ),
                            E::OnTextInput if handle == app.contrast_value => app
                                .preview_text_value(
                                    CONTRAST_CODE,
                                    &app.contrast_value,
                                    &app.contrast_slider,
                                ),
                            E::OnKeyEnter if handle == app.brightness_value => app
                                .commit_text_value(
                                    BRIGHTNESS_CODE,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                ),
                            E::OnKeyEnter if handle == app.contrast_value => app.commit_text_value(
                                CONTRAST_CODE,
                                &app.contrast_value,
                                &app.contrast_slider,
                            ),
                            E::OnKeyPress if handle == app.brightness_value => app
                                .handle_value_key(
                                    BRIGHTNESS_CODE,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                    evt_data.on_key(),
                                ),
                            E::OnKeyPress if handle == app.contrast_value => app.handle_value_key(
                                CONTRAST_CODE,
                                &app.contrast_value,
                                &app.contrast_slider,
                                evt_data.on_key(),
                            ),
                            E::OnMouseWheel if handle == app.brightness_value => app
                                .handle_value_wheel(
                                    BRIGHTNESS_CODE,
                                    &app.brightness_value,
                                    &app.brightness_slider,
                                    mouse_wheel_delta(&evt_data),
                                ),
                            E::OnMouseWheel if handle == app.contrast_value => app
                                .handle_value_wheel(
                                    CONTRAST_CODE,
                                    &app.contrast_value,
                                    &app.contrast_slider,
                                    mouse_wheel_delta(&evt_data),
                                ),
                            E::OnMouseMove if handle == app.brightness_value => {
                                app.brightness_value.set_focus()
                            }
                            E::OnMouseMove if handle == app.contrast_value => {
                                app.contrast_value.set_focus()
                            }
                            E::OnButtonClick if handle == app.input_tb_card => {
                                app.set_input(INPUT_USB_C_VALUE)
                            }
                            E::OnButtonClick if handle == app.input_dp_card => {
                                app.set_input(INPUT_DP_VALUE)
                            }
                            E::OnButtonClick if handle == app.input_hdmi_card => {
                                app.set_input(INPUT_HDMI_VALUE)
                            }
                            E::OnMousePress(nwg::MousePressEvent::MousePressLeftUp)
                                if handle == app.input_tb_radio
                                    || handle == app.input_tb_icon
                                    || handle == app.input_tb_label =>
                            {
                                app.set_input(INPUT_USB_C_VALUE)
                            }
                            E::OnMousePress(nwg::MousePressEvent::MousePressLeftUp)
                                if handle == app.input_dp_radio
                                    || handle == app.input_dp_icon
                                    || handle == app.input_dp_label =>
                            {
                                app.set_input(INPUT_DP_VALUE)
                            }
                            E::OnMousePress(nwg::MousePressEvent::MousePressLeftUp)
                                if handle == app.input_hdmi_radio
                                    || handle == app.input_hdmi_icon
                                    || handle == app.input_hdmi_label =>
                            {
                                app.set_input(INPUT_HDMI_VALUE)
                            }
                            E::OnTimerTick if handle == app.live_write_timer => {
                                app.flush_pending_writes()
                            }
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
        background_color: Option<[u8; 3]>,
    ) -> Result<(), nwg::NwgError> {
        nwg::Label::builder()
            .text(text)
            .background_color(background_color)
            .parent(parent)
            .build(label)
    }

    fn build_display_text(
        label: &mut nwg::RichLabel,
        parent: impl Into<nwg::ControlHandle>,
        text: &str,
        background_color: Option<[u8; 3]>,
    ) -> Result<(), nwg::NwgError> {
        nwg::RichLabel::builder()
            .text(text)
            .background_color(background_color)
            .parent(parent)
            .build(label)?;
        apply_display_text_style(label, text);
        Ok(())
    }

    fn build_surface(
        frame: &mut nwg::ImageFrame,
        parent: impl Into<nwg::ControlHandle>,
        color: [u8; 3],
    ) -> Result<(), nwg::NwgError> {
        nwg::ImageFrame::builder()
            .background_color(Some(color))
            .parent(parent)
            .build(frame)
    }

    fn build_bitmap(
        bitmap: &mut nwg::Bitmap,
        bytes: &'static [u8],
        size: (u32, u32),
    ) -> Result<(), nwg::NwgError> {
        nwg::Bitmap::builder()
            .source_bin(Some(bytes))
            .size(Some(size))
            .build(bitmap)
    }

    fn build_tinted_image_frame(
        frame: &mut nwg::ImageFrame,
        parent: impl Into<nwg::ControlHandle>,
        bitmap: &nwg::Bitmap,
        color: [u8; 3],
    ) -> Result<(), nwg::NwgError> {
        nwg::ImageFrame::builder()
            .bitmap(Some(bitmap))
            .background_color(Some(color))
            .parent(parent)
            .build(frame)
    }

    fn build_button(
        button: &mut nwg::Button,
        parent: impl Into<nwg::ControlHandle>,
        text: &str,
    ) -> Result<(), nwg::NwgError> {
        nwg::Button::builder()
            .text(text)
            .parent(parent)
            .build(button)
    }

    fn build_input_button(
        button: &mut nwg::CheckBox,
        parent: impl Into<nwg::ControlHandle>,
    ) -> Result<(), nwg::NwgError> {
        nwg::CheckBox::builder()
            .text("")
            .background_color(Some(BUTTON_CONTENT_BG))
            .flags(
                nwg::CheckBoxFlags::VISIBLE
                    | nwg::CheckBoxFlags::PUSHLIKE
                    | nwg::CheckBoxFlags::TAB_STOP,
            )
            .parent(parent)
            .build(button)
    }

    fn build_bitmap_button(
        button: &mut nwg::Button,
        parent: impl Into<nwg::ControlHandle>,
        bitmap: &nwg::Bitmap,
    ) -> Result<(), nwg::NwgError> {
        nwg::Button::builder()
            .text("")
            .bitmap(Some(bitmap))
            .flags(
                nwg::ButtonFlags::VISIBLE | nwg::ButtonFlags::BITMAP | nwg::ButtonFlags::TAB_STOP,
            )
            .parent(parent)
            .build(button)
    }

    fn build_value_input(
        input: &mut nwg::TextInput,
        parent: impl Into<nwg::ControlHandle>,
    ) -> Result<(), nwg::NwgError> {
        nwg::TextInput::builder()
            .text("n/a")
            .limit(3)
            .align(nwg::HTextAlign::Center)
            .background_color(Some(BUTTON_CONTENT_BG))
            .flags(
                nwg::TextInputFlags::VISIBLE
                    | nwg::TextInputFlags::NUMBER
                    | nwg::TextInputFlags::TAB_STOP,
            )
            .parent(parent)
            .build(input)
    }

    fn build_trackbar(
        slider: &mut nwg::TrackBar,
        parent: impl Into<nwg::ControlHandle>,
    ) -> Result<(), nwg::NwgError> {
        nwg::TrackBar::builder()
            .range(Some(0..100))
            .pos(Some(0))
            .background_color(Some(PANEL_BG))
            .flags(
                nwg::TrackBarFlags::VISIBLE
                    | nwg::TrackBarFlags::HORIZONTAL
                    | nwg::TrackBarFlags::TAB_STOP,
            )
            .parent(parent)
            .build(slider)
    }

    fn mouse_wheel_delta(data: &nwg::EventData) -> i32 {
        match data {
            nwg::EventData::OnMouseWheel(delta) => *delta,
            _ => 0,
        }
    }
}

fn place_absolute<T>(control: &T, rect: Rect)
where
    T: ControlPlacement,
{
    control.set_position(rect.x, rect.y);
    control.set_size(rect.width, rect.height);
}

fn place_relative<T>(control: &T, rect: Rect, parent: Rect)
where
    T: ControlPlacement,
{
    control.set_position(rect.x - parent.x, rect.y - parent.y);
    control.set_size(rect.width, rect.height);
}

fn inset_rect(rect: Rect, inset: u32) -> Rect {
    Rect::new(
        rect.x + inset as i32,
        rect.y + inset as i32,
        rect.width.saturating_sub(inset * 2),
        rect.height.saturating_sub(inset * 2),
    )
}

fn set_display_text(label: &nwg::RichLabel, text: &str) {
    label.set_text(text);
    apply_display_text_style(label, text);
}

fn apply_display_text_style(label: &nwg::RichLabel, text: &str) {
    let text_len = text.encode_utf16().count() as u32;
    if text_len == 0 {
        return;
    }

    label.set_char_format(
        0..text_len,
        &nwg::CharFormat {
            text_color: Some(DISPLAY_TEXT_COLOR),
            ..Default::default()
        },
    );
}

trait ControlPlacement {
    fn set_position(&self, x: i32, y: i32);
    fn set_size(&self, width: u32, height: u32);
}

macro_rules! impl_control_placement {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ControlPlacement for $ty {
                fn set_position(&self, x: i32, y: i32) {
                    self.set_position(x, y);
                }

                fn set_size(&self, width: u32, height: u32) {
                    self.set_size(width, height);
                }
            }
        )+
    };
}

impl_control_placement!(
    nwg::Button,
    nwg::CheckBox,
    nwg::Frame,
    nwg::ImageFrame,
    nwg::Label,
    nwg::RichLabel,
    nwg::RadioButton,
    nwg::TextInput,
    nwg::TrackBar,
);

fn window_client_size(window: &nwg::Window) -> Option<(u32, u32)> {
    let hwnd = window.handle.hwnd()? as *mut core::ffi::c_void;
    let mut rect = WinRect::default();
    if unsafe { GetClientRect(windows::Win32::Foundation::HWND(hwnd), &mut rect) }.is_err() {
        return None;
    }

    Some((
        (rect.right - rect.left).max(0) as u32,
        (rect.bottom - rect.top).max(0) as u32,
    ))
}

fn center_window(window: &nwg::Window, width: u32, height: u32) {
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) }.max(0) as u32;
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) }.max(0) as u32;
    let x = screen_width.saturating_sub(width) / 2;
    let y = screen_height.saturating_sub(height) / 2;
    window.set_position(x as i32, y as i32);
}

fn main() {
    enable_dpi_awareness();
    nwg::init().expect("failed to initialize Native Windows GUI");

    let launch_mode = launch_mode_from_args(std::env::args().skip(1));
    let debug_output_path = match launch_mode {
        LaunchMode::DebugUi => Some(debug_ui_output_path(
            &std::env::current_dir().expect("failed to determine current working directory"),
            now_ms(),
        )),
        LaunchMode::Normal => None,
    };

    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("Segoe UI")
        .size_absolute(FONT_POINT_SIZE)
        .build(&mut font)
        .expect("failed to build default font");
    nwg::Font::set_global_default(Some(font));

    let app = TrayApp {
        launch_mode,
        debug_ui_output_path: debug_output_path,
        ..Default::default()
    };

    let ui = TrayApp::build_ui(app).expect("failed to build tray UI");
    if launch_mode == LaunchMode::DebugUi {
        ui.start_debug_ui_mode();
    }

    nwg::dispatch_thread_events();
}

fn enable_dpi_awareness() {
    #[allow(deprecated)]
    unsafe {
        nwg::set_dpi_awareness();
    }
}
