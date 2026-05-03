#[path = "../src/monitor_text.rs"]
mod monitor_text;

use monitor_text::{compact_input_label, monitor_heading, monitor_title};

#[test]
fn does_not_append_model_when_description_already_contains_it() {
    assert_eq!(
        monitor_title("Dell U4025QW (DP HDR)", Some("U4025QW")),
        "Dell U4025QW (DP HDR)"
    );
}

#[test]
fn appends_model_when_description_does_not_contain_it() {
    assert_eq!(
        monitor_title("Dell Display (DP HDR)", Some("U4025QW")),
        "Dell Display (DP HDR) (U4025QW)"
    );
}

#[test]
fn keeps_description_when_model_is_missing_or_blank() {
    assert_eq!(monitor_title("Dell Display", None), "Dell Display");
    assert_eq!(monitor_title("Dell Display", Some("   ")), "Dell Display");
}

#[test]
fn heading_uses_clean_dell_model_when_model_is_known() {
    assert_eq!(
        monitor_heading("Dell U4025QW (DP HDR)", Some("U4025QW")),
        "Dell U4025QW"
    );
}

#[test]
fn heading_falls_back_to_description_without_parenthetical_suffix() {
    assert_eq!(
        monitor_heading("Dell U4025QW (DP HDR)", None),
        "Dell U4025QW"
    );
}

#[test]
fn compact_input_label_matches_header_labels() {
    assert_eq!(compact_input_label("DisplayPort 1"), "DP");
    assert_eq!(compact_input_label("USB-C"), "TB");
    assert_eq!(compact_input_label("HDMI"), "HDMI");
    assert_eq!(compact_input_label("unknown"), "Input");
}
