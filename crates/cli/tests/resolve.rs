use dell_controller_core::profiles::u4025qw_profile;
use dellctl::ops::resolve_write;

#[test]
fn resolves_safe_named_control_writes() {
    let profile = u4025qw_profile();

    let write = resolve_write(Some(&profile), "brightness", "70", false).expect("safe write");

    assert_eq!(write.code.get(), 0x10);
    assert_eq!(write.value, 70);
}

#[test]
fn blocks_unknown_raw_vcp_writes_unless_advanced_is_set() {
    let profile = u4025qw_profile();

    assert!(resolve_write(Some(&profile), "0xE5", "1", false).is_err());

    let write = resolve_write(Some(&profile), "0xE5", "1", true).expect("advanced write");
    assert_eq!(write.code.get(), 0xE5);
    assert_eq!(write.value, 1);
}
