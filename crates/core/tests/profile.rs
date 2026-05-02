use dell_controller_core::profile::{ControlKind, MonitorProfile};

const PROFILE: &str = r#"
[identity]
manufacturer = "DEL"
model = "U4025QW"

[[controls]]
name = "brightness"
label = "Brightness"
vcp = "0x10"
kind = "continuous"
safe_write = true

[[controls]]
name = "input"
label = "Input Source"
vcp = "0x60"
kind = "enum"
safe_write = true

[controls.values]
"0x0F" = "DisplayPort"
"0x11" = "HDMI"
"0x19" = "USB-C"

[[controls]]
name = "smart-hdr"
label = "Smart HDR"
vcp = "0xE2"
kind = "enum"
safe_write = false

[[quick_actions]]
name = "usb-c"
label = "Switch to USB-C"
control = "input"
value = "0x19"
"#;

#[test]
fn loads_controls_and_quick_actions_from_toml() {
    let profile = MonitorProfile::from_toml_str(PROFILE).expect("profile loads");

    assert_eq!(profile.identity.model, "U4025QW");
    let brightness = profile.control("brightness").expect("brightness control");
    assert_eq!(brightness.vcp.get(), 0x10);
    assert_eq!(brightness.kind, ControlKind::Continuous);
    assert!(brightness.safe_write);

    let input = profile.control("input").expect("input control");
    assert_eq!(input.value_label(0x19), Some("USB-C"));

    let action = profile.quick_action("usb-c").expect("quick action");
    assert_eq!(action.control, "input");
    assert_eq!(action.value.get(), 0x19);
}

#[test]
fn refuses_unknown_writes_without_an_explicit_safe_control() {
    let profile = MonitorProfile::from_toml_str(PROFILE).expect("profile loads");

    assert!(profile.can_write_control("brightness"));
    assert!(!profile.can_write_control("smart-hdr"));
    assert!(!profile.can_write_vcp(0xE5));
}

#[test]
fn resolves_packed_mccs_input_values_by_low_byte() {
    let profile = MonitorProfile::from_toml_str(PROFILE).expect("profile loads");
    let input = profile.control("input").expect("input control");

    assert_eq!(input.value_label(0x0F0F), Some("DisplayPort"));
    assert_eq!(input.value_label(0x1919), Some("USB-C"));
}
