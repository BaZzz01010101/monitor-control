use dell_controller_core::profiles::u4025qw_profile;

#[test]
fn u4025qw_profile_marks_known_safe_controls_and_vendor_controls() {
    let profile = u4025qw_profile();

    assert_eq!(profile.identity.model, "U4025QW");
    assert!(profile.can_write_control("brightness"));
    assert!(profile.can_write_control("contrast"));
    assert!(profile.can_write_control("input"));
    assert!(!profile.can_write_control("smart-hdr"));
    assert!(profile.control("hardware-kvm").is_some());
}
