#[path = "../src/ui_layout.rs"]
mod ui_layout;

use ui_layout::{
    compute_layout, heading_font_spec, Rect, Size, TextMetrics, UiMetrics, FONT_POINT_SIZE,
    HDR_FONT_POINT_SIZE, HEADING_FONT_POINT_SIZE, SECTION_TITLE_FONT_POINT_SIZE,
};

fn sample_text_metrics() -> TextMetrics {
    TextMetrics {
        monitor_title: Size::new(252, 32),
        input_summary: Size::new(24, 24),
        hdr_status: Size::new(170, 24),
        refresh_label: Size::new(66, 24),
        picture_title: Size::new(78, 28),
        input_title: Size::new(118, 28),
        brightness_label: Size::new(84, 24),
        contrast_label: Size::new(70, 24),
        status: Size::new(132, 20),
        input_tb_label: Size::new(20, 20),
        input_dp_label: Size::new(20, 20),
        input_hdmi_label: Size::new(42, 20),
    }
}

fn assert_inside(child: Rect, parent: Rect, name: &str) {
    assert!(
        child.x >= parent.x
            && child.y >= parent.y
            && child.right() <= parent.right()
            && child.bottom() <= parent.bottom(),
        "{name} should stay inside its parent"
    );
}

fn assert_no_overlap(a: Rect, b: Rect, a_name: &str, b_name: &str) {
    let overlaps = a.x < b.right() && a.right() > b.x && a.y < b.bottom() && a.bottom() > b.y;
    assert!(!overlaps, "{a_name} should not overlap {b_name}");
}

#[test]
fn modern_native_font_metrics_match_the_layout_spec() {
    assert_eq!(FONT_POINT_SIZE, 13);
    assert_eq!(SECTION_TITLE_FONT_POINT_SIZE, 18);
    assert_eq!(HEADING_FONT_POINT_SIZE, 15);
    assert_eq!(HDR_FONT_POINT_SIZE, 10);
    assert_eq!(heading_font_spec().weight, 700);
}

#[test]
fn refined_ui_metrics_match_the_current_spec() {
    let metrics = UiMetrics::default();

    assert_eq!(metrics.refresh_button_height, 32);
    assert_eq!(metrics.refresh_icon_box, Size::new(22, 22));
    assert_eq!(metrics.row_icon_box, Size::new(24, 24));
    assert_eq!(metrics.input_indicator_box, Size::new(24, 24));
    assert_eq!(metrics.slider_y_offset, 15);
    assert_eq!(metrics.value_field_size.height, 38);
    assert_eq!(metrics.stepper_button_size, Size::new(28, 19));
}

#[test]
fn three_primary_panels_share_the_same_resolved_width() {
    let layout = compute_layout(&UiMetrics::default(), &sample_text_metrics());

    assert_eq!(layout.header_frame.width, layout.shared_panel_width);
    assert_eq!(layout.picture_frame.width, layout.shared_panel_width);
    assert_eq!(layout.input_frame.width, layout.shared_panel_width);
    assert_eq!(layout.header_frame.x, layout.picture_frame.x);
    assert_eq!(layout.picture_frame.x, layout.input_frame.x);
    assert_eq!(
        layout.client_size.width as i32 - layout.header_frame.right(),
        layout.header_frame.x
    );
}

#[test]
fn root_client_size_is_derived_from_the_resolved_layout_stack() {
    let metrics = UiMetrics::default();
    let layout = compute_layout(&metrics, &sample_text_metrics());

    assert_eq!(
        layout.client_size.width,
        layout.shared_panel_width + metrics.window_padding * 2
    );

    let expected_height = metrics.window_padding
        + layout.header_frame.height
        + metrics.section_gap
        + layout.picture_title.height
        + metrics.title_gap
        + layout.picture_frame.height
        + metrics.section_gap
        + layout.input_title.height
        + metrics.title_gap
        + layout.input_frame.height
        + metrics.status_gap
        + layout.status_label.height
        + metrics.window_padding;

    assert_eq!(layout.client_size.height, expected_height);
    assert!(layout.status_label.height <= 24);
}

#[test]
fn picture_rows_share_columns_and_connect_value_fields_to_steppers() {
    let text = sample_text_metrics();
    let layout = compute_layout(&UiMetrics::default(), &text);

    assert_eq!(layout.brightness_icon.x, layout.contrast_icon.x);
    assert_eq!(layout.brightness_label.x, layout.contrast_label.x);
    assert_eq!(layout.brightness_slider.x, layout.contrast_slider.x);
    assert_eq!(layout.brightness_slider.width, layout.contrast_slider.width);
    assert_eq!(layout.brightness_value.x, layout.contrast_value.x);
    assert_eq!(layout.brightness_value.width, layout.contrast_value.width);

    assert_eq!(layout.brightness_value.right(), layout.brightness_up.x);
    assert_eq!(layout.brightness_value.right(), layout.brightness_down.x);
    assert_eq!(layout.contrast_value.right(), layout.contrast_up.x);
    assert_eq!(layout.contrast_value.right(), layout.contrast_down.x);

    assert_eq!(
        layout.brightness_value.height,
        layout.brightness_up.height + layout.brightness_down.height
    );
    assert_eq!(
        layout.contrast_value.height,
        layout.contrast_up.height + layout.contrast_down.height
    );
    assert_eq!(layout.brightness_up.bottom(), layout.brightness_down.y);
    assert_eq!(layout.contrast_up.bottom(), layout.contrast_down.y);
    assert_eq!(layout.brightness_up.height, layout.brightness_down.height);
    assert_eq!(layout.contrast_up.height, layout.contrast_down.height);
    assert_eq!(layout.brightness_value.height, 38);
    assert_eq!(layout.contrast_value.height, 38);
    assert_eq!(layout.brightness_up.height, 19);
    assert_eq!(layout.contrast_up.height, 19);
    assert!(layout.brightness_label.height >= text.brightness_label.height + 4);
    assert!(layout.contrast_label.height >= text.contrast_label.height + 4);
    assert!(layout.brightness_slider.y > layout.brightness_label.y);
    assert!(layout.contrast_slider.y > layout.contrast_label.y);
}

#[test]
fn separators_have_native_insets_and_align_between_content_groups() {
    let metrics = UiMetrics::default();
    let layout = compute_layout(&metrics, &sample_text_metrics());

    assert_eq!(layout.header_separator.width, 1);
    assert!(layout.header_separator.x > layout.monitor_title.right());
    assert!(layout.input_summary.x > layout.header_separator.right());

    assert!(layout.picture_separator.width < layout.picture_frame.width);
    assert!(layout.picture_separator.x >= layout.picture_frame.x + 16);
    assert!(layout.picture_separator.right() <= layout.picture_frame.right() - 16);
    assert!(layout.picture_separator.y > layout.brightness_value.bottom());
    assert!(layout.picture_separator.y < layout.contrast_value.y);
}

#[test]
fn refresh_icon_is_halfway_between_button_edge_and_label() {
    let layout = compute_layout(&UiMetrics::default(), &sample_text_metrics());

    assert_eq!(layout.refresh_button.height, 32);
    assert_eq!(layout.refresh_icon.size(), Size::new(22, 22));
    let icon_center = layout.refresh_icon.x + (layout.refresh_icon.width / 2) as i32;
    let halfway_to_text =
        layout.refresh_button.x + ((layout.refresh_text.x - layout.refresh_button.x) / 2);

    assert_eq!(icon_center, halfway_to_text);
    assert_eq!(
        layout.refresh_icon.y + (layout.refresh_icon.height / 2) as i32,
        layout.refresh_button.y + (layout.refresh_button.height / 2) as i32
    );
    assert_eq!(
        layout.refresh_text.y + (layout.refresh_text.height / 2) as i32,
        layout.refresh_button.y + (layout.refresh_button.height / 2) as i32
    );
}

#[test]
fn input_option_buttons_stretch_evenly_and_keep_icon_targets_fixed() {
    let text = sample_text_metrics();
    let layout = compute_layout(&UiMetrics::default(), &text);

    assert_eq!(layout.input_tb_card.width, layout.input_dp_card.width);
    assert_eq!(layout.input_tb_card.width, layout.input_hdmi_card.width);

    assert_eq!(
        layout.monitor_preview.size(),
        UiMetrics::default().monitor_preview_box
    );
    assert_eq!(
        layout.refresh_icon.size(),
        UiMetrics::default().refresh_icon_box
    );
    assert_eq!(
        layout.brightness_icon.size(),
        UiMetrics::default().row_icon_box
    );
    assert_eq!(
        layout.contrast_icon.size(),
        UiMetrics::default().row_icon_box
    );
    assert_eq!(
        layout.input_tb_icon.size(),
        UiMetrics::default().input_tb_icon_box
    );
    assert_eq!(
        layout.input_dp_icon.size(),
        UiMetrics::default().input_dp_icon_box
    );
    assert_eq!(
        layout.input_hdmi_icon.size(),
        UiMetrics::default().input_hdmi_icon_box
    );
    assert_eq!(
        layout.input_tb_radio.size(),
        UiMetrics::default().input_indicator_box
    );
    assert!(layout.input_dp_label.width >= text.input_dp_label.width + 14);
    assert!(layout.input_hdmi_label.width >= text.input_hdmi_label.width + 14);
}

#[test]
fn panels_and_children_stay_inside_the_computed_client_area_without_overlap() {
    let layout = compute_layout(&UiMetrics::default(), &sample_text_metrics());
    let root = Rect::new(0, 0, layout.client_size.width, layout.client_size.height);

    assert_inside(layout.header_frame, root, "header_frame");
    assert_inside(layout.picture_frame, root, "picture_frame");
    assert_inside(layout.input_frame, root, "input_frame");
    assert_inside(layout.status_label, root, "status_label");

    assert_no_overlap(
        layout.header_frame,
        layout.picture_title,
        "header_frame",
        "picture_title",
    );
    assert_no_overlap(
        layout.picture_frame,
        layout.input_title,
        "picture_frame",
        "input_title",
    );
    assert_no_overlap(
        layout.input_frame,
        layout.status_label,
        "input_frame",
        "status_label",
    );

    assert_inside(
        layout.monitor_preview,
        layout.header_frame,
        "monitor_preview",
    );
    assert_inside(layout.monitor_title, layout.header_frame, "monitor_title");
    assert_inside(
        layout.header_separator,
        layout.header_frame,
        "header_separator",
    );
    assert_inside(layout.input_summary, layout.header_frame, "input_summary");
    assert_inside(layout.hdr_status, layout.header_frame, "hdr_status");
    assert_inside(layout.refresh_button, layout.header_frame, "refresh_button");
    assert_inside(layout.refresh_icon, layout.refresh_button, "refresh_icon");
    assert_inside(layout.refresh_text, layout.refresh_button, "refresh_text");

    assert_inside(
        layout.brightness_slider,
        layout.picture_frame,
        "brightness_slider",
    );
    assert_inside(
        layout.brightness_value,
        layout.picture_frame,
        "brightness_value",
    );
    assert_inside(layout.brightness_up, layout.picture_frame, "brightness_up");
    assert_inside(
        layout.brightness_down,
        layout.picture_frame,
        "brightness_down",
    );
    assert_inside(
        layout.contrast_slider,
        layout.picture_frame,
        "contrast_slider",
    );
    assert_inside(
        layout.contrast_value,
        layout.picture_frame,
        "contrast_value",
    );
    assert_inside(layout.contrast_up, layout.picture_frame, "contrast_up");
    assert_inside(layout.contrast_down, layout.picture_frame, "contrast_down");
    assert_inside(
        layout.picture_separator,
        layout.picture_frame,
        "picture_separator",
    );

    assert_inside(layout.input_tb_card, layout.input_frame, "input_tb_card");
    assert_inside(layout.input_dp_card, layout.input_frame, "input_dp_card");
    assert_inside(
        layout.input_hdmi_card,
        layout.input_frame,
        "input_hdmi_card",
    );
}
