#![cfg_attr(test, allow(dead_code))]

pub const FONT_POINT_SIZE: u32 = 13;
pub const SECTION_TITLE_FONT_POINT_SIZE: u32 = 18;
pub const HEADING_FONT_POINT_SIZE: u32 = 15;
pub const HDR_FONT_POINT_SIZE: u32 = 10;

#[cfg(windows)]
use windows::Win32::{
    Foundation::SIZE,
    Graphics::Gdi::{
        CreateFontW, DeleteObject, GetDC, GetTextExtentPoint32W, ReleaseDC, SelectObject,
        CLIP_DEFAULT_PRECIS, DEFAULT_CHARSET, DEFAULT_PITCH, DEFAULT_QUALITY, FF_DONTCARE,
        FW_NORMAL, HGDIOBJ, OUT_DEFAULT_PRECIS,
    },
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(self) -> i32 {
        self.x + self.width as i32
    }

    pub fn bottom(self) -> i32 {
        self.y + self.height as i32
    }

    #[allow(dead_code)]
    pub const fn size(self) -> Size {
        Size::new(self.width, self.height)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMetrics {
    pub window_padding: u32,
    pub section_gap: u32,
    pub status_gap: u32,
    pub title_gap: u32,
    pub card_padding: u32,
    pub gap_small: u32,
    pub gap_standard: u32,
    pub gap_large: u32,
    pub header_row_gap: u32,
    pub row_gap: u32,
    pub control_row_height: u32,
    pub slider_height: u32,
    pub slider_y_offset: u32,
    pub refresh_button_height: u32,
    pub refresh_button_padding_x: u32,
    pub refresh_icon_gap: u32,
    pub monitor_preview_box: Size,
    pub refresh_icon_box: Size,
    pub row_icon_box: Size,
    pub input_indicator_box: Size,
    pub input_tb_icon_box: Size,
    pub input_dp_icon_box: Size,
    pub input_hdmi_icon_box: Size,
    pub slider_min_width: u32,
    pub value_field_size: Size,
    pub stepper_button_size: Size,
    pub input_card_height: u32,
    pub input_card_padding_x: u32,
    pub input_card_gap: u32,
    pub separator_inset: u32,
    pub separator_thickness: u32,
}

impl Default for UiMetrics {
    fn default() -> Self {
        Self {
            window_padding: 16,
            section_gap: 18,
            status_gap: 10,
            title_gap: 8,
            card_padding: 16,
            gap_small: 8,
            gap_standard: 12,
            gap_large: 20,
            header_row_gap: 8,
            row_gap: 18,
            control_row_height: 44,
            slider_height: 36,
            slider_y_offset: 15,
            refresh_button_height: 32,
            refresh_button_padding_x: 10,
            refresh_icon_gap: 10,
            monitor_preview_box: Size::new(112, 72),
            refresh_icon_box: Size::new(22, 22),
            row_icon_box: Size::new(24, 24),
            input_indicator_box: Size::new(24, 24),
            input_tb_icon_box: Size::new(24, 24),
            input_dp_icon_box: Size::new(36, 36),
            input_hdmi_icon_box: Size::new(36, 36),
            slider_min_width: 220,
            value_field_size: Size::new(52, 38),
            stepper_button_size: Size::new(28, 19),
            input_card_height: 44,
            input_card_padding_x: 16,
            input_card_gap: 12,
            separator_inset: 18,
            separator_thickness: 1,
        }
    }
}

#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontSpec {
    pub height: u32,
    pub weight: u32,
}

#[cfg(windows)]
impl FontSpec {
    pub const fn new(height: u32, weight: u32) -> Self {
        Self { height, weight }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextMetrics {
    pub monitor_title: Size,
    pub input_summary: Size,
    pub hdr_status: Size,
    pub refresh_label: Size,
    pub picture_title: Size,
    pub input_title: Size,
    pub brightness_label: Size,
    pub contrast_label: Size,
    pub status: Size,
    pub input_tb_label: Size,
    pub input_dp_label: Size,
    pub input_hdmi_label: Size,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowLayout {
    pub client_size: Size,
    pub shared_panel_width: u32,
    pub header_min_size: Size,
    pub picture_min_size: Size,
    pub input_min_size: Size,
    pub header_frame: Rect,
    pub monitor_preview: Rect,
    pub monitor_title: Rect,
    pub header_separator: Rect,
    pub input_summary: Rect,
    pub hdr_status: Rect,
    pub refresh_button: Rect,
    pub refresh_icon: Rect,
    pub refresh_text: Rect,
    pub picture_title: Rect,
    pub picture_frame: Rect,
    pub brightness_icon: Rect,
    pub brightness_label: Rect,
    pub brightness_slider: Rect,
    pub brightness_value: Rect,
    pub brightness_up: Rect,
    pub brightness_down: Rect,
    pub contrast_icon: Rect,
    pub contrast_label: Rect,
    pub contrast_slider: Rect,
    pub contrast_value: Rect,
    pub contrast_up: Rect,
    pub contrast_down: Rect,
    pub picture_separator: Rect,
    pub input_title: Rect,
    pub input_frame: Rect,
    pub input_tb_card: Rect,
    pub input_tb_radio: Rect,
    pub input_tb_icon: Rect,
    pub input_tb_label: Rect,
    pub input_dp_card: Rect,
    pub input_dp_radio: Rect,
    pub input_dp_icon: Rect,
    pub input_dp_label: Rect,
    pub input_hdmi_card: Rect,
    pub input_hdmi_radio: Rect,
    pub input_hdmi_icon: Rect,
    pub input_hdmi_label: Rect,
    pub status_label: Rect,
}

pub fn compute_layout(metrics: &UiMetrics, text: &TextMetrics) -> WindowLayout {
    let header_min_size = header_min_size(metrics, text);
    let picture_min_size = picture_min_size(metrics, text);
    let input_min_size = input_min_size(metrics, text);
    let shared_panel_width = header_min_size
        .width
        .max(picture_min_size.width)
        .max(input_min_size.width);

    let picture_title_width = shared_panel_width.max(text.picture_title.width);
    let input_title_width = shared_panel_width.max(text.input_title.width);
    let status_width = shared_panel_width.max(text.status.width);

    let client_width = shared_panel_width + metrics.window_padding * 2;

    let header_frame = Rect::new(
        metrics.window_padding as i32,
        metrics.window_padding as i32,
        shared_panel_width,
        header_min_size.height,
    );

    let picture_title_y = header_frame.bottom() as u32 + metrics.section_gap;
    let picture_title = Rect::new(
        metrics.window_padding as i32,
        picture_title_y as i32,
        picture_title_width,
        text.picture_title.height,
    );

    let picture_frame_y = picture_title.bottom() as u32 + metrics.title_gap;
    let picture_frame = Rect::new(
        metrics.window_padding as i32,
        picture_frame_y as i32,
        shared_panel_width,
        picture_min_size.height,
    );

    let input_title_y = picture_frame.bottom() as u32 + metrics.section_gap;
    let input_title = Rect::new(
        metrics.window_padding as i32,
        input_title_y as i32,
        input_title_width,
        text.input_title.height,
    );

    let input_frame_y = input_title.bottom() as u32 + metrics.title_gap;
    let input_frame = Rect::new(
        metrics.window_padding as i32,
        input_frame_y as i32,
        shared_panel_width,
        input_min_size.height,
    );

    let status_label_y = input_frame.bottom() as u32 + metrics.status_gap;
    let status_height = (text.status.height + 4).clamp(20, 24);
    let status_label = Rect::new(
        metrics.window_padding as i32,
        status_label_y as i32,
        status_width,
        status_height,
    );

    let client_height = status_label.bottom() as u32 + metrics.window_padding;

    let monitor_preview = Rect::new(
        header_frame.x + metrics.card_padding as i32,
        header_frame.y
            + vertical_center_offset(header_frame.height, metrics.monitor_preview_box.height),
        metrics.monitor_preview_box.width,
        metrics.monitor_preview_box.height,
    );

    let refresh_button_width = refresh_button_width(metrics, text);
    let refresh_button = Rect::new(
        header_frame.right() - metrics.card_padding as i32 - refresh_button_width as i32,
        header_frame.y + vertical_center_offset(header_frame.height, metrics.refresh_button_height),
        refresh_button_width,
        metrics.refresh_button_height,
    );

    let text_block_x = monitor_preview.right() + metrics.gap_large as i32;
    let top_row_y = header_frame.y + metrics.card_padding as i32;
    let second_row_y = top_row_y + text.monitor_title.height as i32 + metrics.header_row_gap as i32;

    let monitor_title = Rect::new(
        text_block_x,
        top_row_y,
        text.monitor_title.width,
        text.monitor_title.height,
    );
    let header_separator = Rect::new(
        monitor_title.right() + metrics.gap_standard as i32,
        top_row_y + vertical_center_offset(text.monitor_title.height, 24),
        metrics.separator_thickness,
        24,
    );
    let input_summary_x = header_separator.right() + metrics.gap_standard as i32;
    let input_summary = Rect::new(
        input_summary_x,
        top_row_y + vertical_center_offset(text.monitor_title.height, text.input_summary.height),
        text.input_summary.width,
        text.input_summary.height,
    );
    let hdr_status = Rect::new(
        text_block_x,
        second_row_y,
        text.hdr_status.width,
        text.hdr_status.height,
    );
    let refresh_icon = Rect::new(
        refresh_button.x + metrics.refresh_button_padding_x as i32,
        refresh_button.y
            + vertical_center_offset(refresh_button.height, metrics.refresh_icon_box.height),
        metrics.refresh_icon_box.width,
        metrics.refresh_icon_box.height,
    );
    let refresh_text = Rect::new(
        refresh_icon.right() + metrics.refresh_icon_gap as i32,
        refresh_button.y + vertical_center_offset(refresh_button.height, text.refresh_label.height),
        text.refresh_label.width,
        text.refresh_label.height,
    );

    let picture_left = picture_frame.x + metrics.card_padding as i32;
    let picture_top = picture_frame.y + metrics.card_padding as i32;
    let icon_x = picture_left;
    let label_x = icon_x + metrics.row_icon_box.width as i32 + metrics.gap_standard as i32;
    let label_width = text.brightness_label.width.max(text.contrast_label.width);
    let brightness_label_height = body_label_rect_height(text.brightness_label.height);
    let contrast_label_height = body_label_rect_height(text.contrast_label.height);
    let value_x = picture_frame.right()
        - metrics.card_padding as i32
        - (metrics.value_field_size.width + metrics.stepper_button_size.width) as i32;
    let slider_x = label_x + label_width as i32 + metrics.gap_large as i32;
    let slider_gap_right = metrics.gap_standard as i32;
    let slider_width = (value_x - slider_gap_right - slider_x).max(0) as u32;

    let brightness_row_y = picture_top;
    let contrast_row_y =
        brightness_row_y + metrics.control_row_height as i32 + metrics.row_gap as i32;
    let icon_y_offset =
        vertical_center_offset(metrics.control_row_height, metrics.row_icon_box.height);
    let slider_height = metrics.slider_height;
    let value_y_offset =
        vertical_center_offset(metrics.control_row_height, metrics.value_field_size.height);

    let brightness_icon = Rect::new(
        icon_x,
        brightness_row_y + icon_y_offset,
        metrics.row_icon_box.width,
        metrics.row_icon_box.height,
    );
    let brightness_label = Rect::new(
        label_x,
        brightness_row_y
            + vertical_center_offset(metrics.control_row_height, brightness_label_height),
        label_width,
        brightness_label_height,
    );
    let brightness_slider = Rect::new(
        slider_x,
        brightness_row_y + metrics.slider_y_offset as i32,
        slider_width.max(metrics.slider_min_width),
        slider_height,
    );
    let brightness_value = Rect::new(
        value_x,
        brightness_row_y + value_y_offset,
        metrics.value_field_size.width,
        metrics.value_field_size.height,
    );
    let brightness_up = Rect::new(
        brightness_value.right(),
        brightness_value.y,
        metrics.stepper_button_size.width,
        metrics.stepper_button_size.height,
    );
    let brightness_down = Rect::new(
        brightness_value.right(),
        brightness_up.bottom(),
        metrics.stepper_button_size.width,
        metrics.stepper_button_size.height,
    );

    let contrast_icon = Rect::new(
        icon_x,
        contrast_row_y + icon_y_offset,
        metrics.row_icon_box.width,
        metrics.row_icon_box.height,
    );
    let contrast_label = Rect::new(
        label_x,
        contrast_row_y + vertical_center_offset(metrics.control_row_height, contrast_label_height),
        label_width,
        contrast_label_height,
    );
    let contrast_slider = Rect::new(
        slider_x,
        contrast_row_y + metrics.slider_y_offset as i32,
        slider_width.max(metrics.slider_min_width),
        slider_height,
    );
    let contrast_value = Rect::new(
        value_x,
        contrast_row_y + value_y_offset,
        metrics.value_field_size.width,
        metrics.value_field_size.height,
    );
    let contrast_up = Rect::new(
        contrast_value.right(),
        contrast_value.y,
        metrics.stepper_button_size.width,
        metrics.stepper_button_size.height,
    );
    let contrast_down = Rect::new(
        contrast_value.right(),
        contrast_up.bottom(),
        metrics.stepper_button_size.width,
        metrics.stepper_button_size.height,
    );

    let picture_separator = Rect::new(
        picture_frame.x + metrics.separator_inset as i32,
        brightness_row_y + metrics.control_row_height as i32 + (metrics.row_gap / 2) as i32,
        picture_frame
            .width
            .saturating_sub(metrics.separator_inset * 2),
        metrics.separator_thickness,
    );

    let input_card_width =
        (shared_panel_width - metrics.card_padding * 2 - metrics.input_card_gap * 2) / 3;
    let input_cards_y =
        input_frame.y + (input_frame.height.saturating_sub(metrics.input_card_height) / 2) as i32;
    let input_tb_card = Rect::new(
        input_frame.x + metrics.card_padding as i32,
        input_cards_y,
        input_card_width,
        metrics.input_card_height,
    );
    let input_dp_card = Rect::new(
        input_tb_card.right() + metrics.input_card_gap as i32,
        input_cards_y,
        input_card_width,
        metrics.input_card_height,
    );
    let input_hdmi_card = Rect::new(
        input_dp_card.right() + metrics.input_card_gap as i32,
        input_cards_y,
        input_card_width,
        metrics.input_card_height,
    );

    let (input_tb_radio, input_tb_icon, input_tb_label) = layout_input_card(
        input_tb_card,
        metrics,
        metrics.input_tb_icon_box,
        text.input_tb_label,
    );
    let (input_dp_radio, input_dp_icon, input_dp_label) = layout_input_card(
        input_dp_card,
        metrics,
        metrics.input_dp_icon_box,
        text.input_dp_label,
    );
    let (input_hdmi_radio, input_hdmi_icon, input_hdmi_label) = layout_input_card(
        input_hdmi_card,
        metrics,
        metrics.input_hdmi_icon_box,
        text.input_hdmi_label,
    );

    WindowLayout {
        client_size: Size::new(client_width, client_height),
        shared_panel_width,
        header_min_size,
        picture_min_size,
        input_min_size,
        header_frame,
        monitor_preview,
        monitor_title,
        header_separator,
        input_summary,
        hdr_status,
        refresh_button,
        refresh_icon,
        refresh_text,
        picture_title,
        picture_frame,
        brightness_icon,
        brightness_label,
        brightness_slider,
        brightness_value,
        brightness_up,
        brightness_down,
        contrast_icon,
        contrast_label,
        contrast_slider,
        contrast_value,
        contrast_up,
        contrast_down,
        picture_separator,
        input_title,
        input_frame,
        input_tb_card,
        input_tb_radio,
        input_tb_icon,
        input_tb_label,
        input_dp_card,
        input_dp_radio,
        input_dp_icon,
        input_dp_label,
        input_hdmi_card,
        input_hdmi_radio,
        input_hdmi_icon,
        input_hdmi_label,
        status_label,
    }
}

fn header_min_size(metrics: &UiMetrics, text: &TextMetrics) -> Size {
    let refresh_button_width = refresh_button_width(metrics, text);
    let top_row_width = metrics.monitor_preview_box.width
        + metrics.gap_large
        + text.monitor_title.width
        + metrics.gap_standard
        + metrics.separator_thickness
        + metrics.gap_standard
        + text.input_summary.width
        + metrics.gap_large
        + refresh_button_width;
    let bottom_row_width = metrics.monitor_preview_box.width
        + metrics.gap_large
        + text.hdr_status.width
        + metrics.gap_large
        + refresh_button_width;
    let width = metrics.card_padding * 2 + top_row_width.max(bottom_row_width);

    let text_stack_height =
        text.monitor_title.height + metrics.header_row_gap + text.hdr_status.height;
    let content_height = metrics
        .monitor_preview_box
        .height
        .max(text_stack_height)
        .max(metrics.refresh_button_height);
    let height = metrics.card_padding * 2 + content_height;

    Size::new(width, height)
}

fn picture_min_size(metrics: &UiMetrics, text: &TextMetrics) -> Size {
    let label_width = text.brightness_label.width.max(text.contrast_label.width);
    let compound_width = metrics.value_field_size.width + metrics.stepper_button_size.width;
    let row_width = metrics.row_icon_box.width
        + metrics.gap_standard
        + label_width
        + metrics.gap_large
        + metrics.slider_min_width
        + metrics.gap_standard
        + compound_width;
    let width = metrics.card_padding * 2 + row_width;
    let height = metrics.card_padding * 2 + metrics.control_row_height * 2 + metrics.row_gap;
    Size::new(width, height)
}

fn input_min_size(metrics: &UiMetrics, text: &TextMetrics) -> Size {
    let tb_width = input_option_min_width(metrics, metrics.input_tb_icon_box, text.input_tb_label);
    let dp_width = input_option_min_width(metrics, metrics.input_dp_icon_box, text.input_dp_label);
    let hdmi_width =
        input_option_min_width(metrics, metrics.input_hdmi_icon_box, text.input_hdmi_label);
    let option_width = tb_width.max(dp_width).max(hdmi_width);
    let width = metrics.card_padding * 2 + option_width * 3 + metrics.input_card_gap * 2;
    let height = metrics.card_padding * 2 + metrics.input_card_height;
    Size::new(width, height)
}

fn refresh_button_width(metrics: &UiMetrics, text: &TextMetrics) -> u32 {
    metrics.refresh_icon_box.width
        + text.refresh_label.width
        + metrics.refresh_button_padding_x * 2
        + metrics.refresh_icon_gap
}

fn input_option_min_width(metrics: &UiMetrics, icon: Size, label: Size) -> u32 {
    metrics.input_card_padding_x * 2
        + metrics.input_indicator_box.width
        + metrics.gap_standard
        + icon.width
        + metrics.gap_small
        + kvm_label_rect_width(label.width)
}

fn layout_input_card(
    card: Rect,
    metrics: &UiMetrics,
    icon_size: Size,
    label: Size,
) -> (Rect, Rect, Rect) {
    let group_width = metrics.input_indicator_box.width
        + metrics.gap_standard
        + icon_size.width
        + metrics.gap_small
        + kvm_label_rect_width(label.width);
    let spacer = card.width.saturating_sub(group_width) / 2;
    let radio_x = card.x + spacer as i32;
    let radio = Rect::new(
        radio_x,
        card.y + vertical_center_offset(card.height, metrics.input_indicator_box.height),
        metrics.input_indicator_box.width,
        metrics.input_indicator_box.height,
    );
    let icon = Rect::new(
        radio.right() + metrics.gap_standard as i32,
        card.y + vertical_center_offset(card.height, icon_size.height),
        icon_size.width,
        icon_size.height,
    );
    let label = Rect::new(
        icon.right() + metrics.gap_small as i32,
        card.y + vertical_center_offset(card.height, label.height),
        kvm_label_rect_width(label.width),
        label.height,
    );
    (radio, icon, label)
}

fn body_label_rect_height(text_height: u32) -> u32 {
    (text_height + 4).max(21)
}

fn kvm_label_rect_width(text_width: u32) -> u32 {
    text_width + 14
}

fn vertical_center_offset(container_height: u32, content_height: u32) -> i32 {
    (container_height.saturating_sub(content_height) / 2) as i32
}

#[cfg(windows)]
pub fn body_font_spec() -> FontSpec {
    FontSpec::new(FONT_POINT_SIZE, FW_NORMAL.0)
}

#[cfg(windows)]
pub fn hdr_font_spec() -> FontSpec {
    FontSpec::new(HDR_FONT_POINT_SIZE, FW_NORMAL.0)
}

#[cfg(windows)]
pub fn section_font_spec() -> FontSpec {
    FontSpec::new(SECTION_TITLE_FONT_POINT_SIZE, 600)
}

#[cfg(windows)]
pub fn heading_font_spec() -> FontSpec {
    FontSpec::new(HEADING_FONT_POINT_SIZE, 700)
}

#[cfg(windows)]
pub fn measure_text(text: &str, font: FontSpec) -> Size {
    let wide: Vec<u16> = text.encode_utf16().collect();
    if wide.is_empty() {
        return Size::new(0, font.height);
    }

    unsafe {
        let dc = GetDC(None);
        let font_handle = CreateFontW(
            -(font.height as i32),
            0,
            0,
            0,
            font.weight as i32,
            0,
            0,
            0,
            DEFAULT_CHARSET,
            OUT_DEFAULT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            DEFAULT_QUALITY,
            u32::from(DEFAULT_PITCH.0 | FF_DONTCARE.0),
            windows::core::w!("Segoe UI"),
        );
        let previous = SelectObject(dc, HGDIOBJ(font_handle.0));
        let mut size = SIZE::default();
        let _ = GetTextExtentPoint32W(dc, &wide, &mut size);
        let _ = SelectObject(dc, previous);
        let _ = DeleteObject(font_handle.into());
        let _ = ReleaseDC(None, dc);
        Size::new(size.cx.max(0) as u32, size.cy.max(0) as u32)
    }
}
