use std::fs;

use dell_controller_tray::persistence::{
    PersistedAppState, PersistedSettings, PersistedWindowPosition, PersistenceStore,
};
use tempfile::tempdir;

#[test]
fn missing_files_load_as_defaults() {
    let dir = tempdir().expect("temp dir should exist");
    let store = PersistenceStore::new(dir.path());

    assert_eq!(
        store.load_settings().expect("settings load"),
        PersistedSettings::default()
    );
    assert_eq!(
        store.load_state().expect("state load"),
        PersistedAppState::default()
    );
}

#[test]
fn settings_round_trip_through_toml_files() {
    let dir = tempdir().expect("temp dir should exist");
    let store = PersistenceStore::new(dir.path());
    let settings = PersistedSettings {
        autostart_enabled: true,
        selected_monitor_key: "dell-u4025qw|u4025qw|0".into(),
        tb_shortcut: "Ctrl+Alt+T".into(),
        dp_shortcut: "Ctrl+Alt+D".into(),
        hdmi_shortcut: "Ctrl+Alt+H".into(),
    };

    store.save_settings(&settings).expect("settings save");

    let path = dir.path().join("settings.toml");
    let file = fs::read_to_string(&path).expect("settings file should exist");
    assert!(file.contains("autostart_enabled = true"));
    assert!(file.contains("tb_shortcut = \"Ctrl+Alt+T\""));
    assert_eq!(store.load_settings().expect("settings load"), settings);
}

#[test]
fn app_state_round_trip_through_toml_files() {
    let dir = tempdir().expect("temp dir should exist");
    let store = PersistenceStore::new(dir.path());
    let state = PersistedAppState {
        window_position: Some(PersistedWindowPosition { x: 120, y: 340 }),
    };

    store.save_state(&state).expect("state save");

    let path = dir.path().join("state.toml");
    let file = fs::read_to_string(&path).expect("state file should exist");
    assert!(file.contains("[window_position]"));
    assert!(file.contains("x = 120"));
    assert_eq!(store.load_state().expect("state load"), state);
}

#[test]
fn invalid_toml_is_preserved_and_loads_defaults_with_a_warning() {
    let dir = tempdir().expect("temp dir should exist");
    let invalid_settings = "this = { is = invalid";
    fs::write(dir.path().join("settings.toml"), invalid_settings).expect("write invalid settings");
    fs::write(dir.path().join("state.toml"), "not = [valid").expect("write invalid state");
    let store = PersistenceStore::new(dir.path());

    let settings = store.load_settings_with_warning().expect("settings load");
    let state = store.load_state_with_warning().expect("state load");

    assert_eq!(settings.value, PersistedSettings::default());
    assert_eq!(state.value, PersistedAppState::default());
    assert!(settings
        .warning
        .as_deref()
        .is_some_and(|warning| warning.contains("settings.toml")));
    assert!(state
        .warning
        .as_deref()
        .is_some_and(|warning| warning.contains("state.toml")));

    let backup = fs::read_dir(dir.path())
        .expect("dir can be read")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.starts_with("settings.corrupt.") && name.ends_with(".toml")
                })
        })
        .expect("corrupt settings backup should exist");
    assert_eq!(
        fs::read_to_string(backup).expect("backup should be readable"),
        invalid_settings
    );
}

#[test]
fn missing_selected_monitor_key_defaults_to_empty_string() {
    let dir = tempdir().expect("temp dir should exist");
    fs::write(
        dir.path().join("settings.toml"),
        "autostart_enabled = true\ntb_shortcut = \"Ctrl+Alt+T\"\n",
    )
    .expect("write old settings");
    let store = PersistenceStore::new(dir.path());

    let settings = store.load_settings().expect("settings load");

    assert_eq!(settings.selected_monitor_key, "");
    assert!(settings.autostart_enabled);
    assert_eq!(settings.tb_shortcut, "Ctrl+Alt+T");
}
