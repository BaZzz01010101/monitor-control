use dell_controller_core::capabilities::Capabilities;

const U4025QW_CAPS: &str = "(prot(monitor)type(LCD)model(U4025QW)cmds(01 02 03 07 0C E3 F3)vcp(02 04 05 08 10 12 14(01 04 05 06 08 09 0B 0C) 16 18 1A 52 60(19 0F 11 ) 62 66(00F2) 67 68 87 AC AE B2 B6 C6 C8 C9 CA CC(02 0A 03 04 08 09 0D 06 ) D6(01 04 05) DC(00 03 05 ) DF E0 E1 E2(00 02 04 0C 0D 0F 10 11 13 0B 1A 1B 3D 14 27 23 24 3A ) E4(00 01) E5 E7(02 03) E8 E9(00 01 02 21 22 24 27 28 29 2A ) EA(FE FC ) F0(09 0A A1 31 32 34 36 ) EE EF(00 01 0F) F1 F2 FD)mswhql(1)asset_eep(40)mccs_ver(2.1))";

#[test]
fn parses_model_commands_mccs_version_and_vcp_values() {
    let caps = Capabilities::parse(U4025QW_CAPS).expect("valid capabilities string");

    assert_eq!(caps.model.as_deref(), Some("U4025QW"));
    assert_eq!(caps.mccs_version.as_deref(), Some("2.1"));
    assert!(caps.commands.contains(&0xE3));
    assert!(caps.supports_vcp(0x10));
    assert_eq!(caps.vcp_values(0x60), Some(&[0x19, 0x0F, 0x11][..]));
    assert_eq!(
        caps.vcp_values(0xE2),
        Some(
            &[
                0x00, 0x02, 0x04, 0x0C, 0x0D, 0x0F, 0x10, 0x11, 0x13, 0x0B, 0x1A, 0x1B, 0x3D, 0x14,
                0x27, 0x23, 0x24, 0x3A
            ][..]
        )
    );
}

#[test]
fn rejects_unbalanced_capabilities_strings() {
    let err = Capabilities::parse("(model(U4025QW)vcp(10 12)").unwrap_err();

    assert!(err.to_string().contains("unbalanced"));
}
