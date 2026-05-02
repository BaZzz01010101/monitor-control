use tempfile::tempdir;

use dell_controller_core::profiles::ProfileCatalog;

#[test]
fn loads_builtin_profiles_and_user_overrides_from_toml_files() {
    let dir = tempdir().expect("temp dir");
    std::fs::write(
        dir.path().join("u4025qw-override.toml"),
        r#"
[identity]
manufacturer = "DEL"
model = "U4025QW"

[[controls]]
name = "brightness"
label = "Brightness Override"
vcp = "0x10"
kind = "continuous"
safe_write = false
"#,
    )
    .expect("write profile");

    let catalog = ProfileCatalog::load(Some(dir.path())).expect("catalog loads");
    let profile = catalog.for_model("U4025QW").expect("profile exists");

    assert_eq!(
        profile.control("brightness").unwrap().label,
        "Brightness Override"
    );
    assert!(!profile.can_write_control("brightness"));
}
