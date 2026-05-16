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
fn invalid_toml_falls_back_to_defaults() {
    let dir = tempdir().expect("temp dir should exist");
    fs::write(dir.path().join("settings.toml"), "this = { is = invalid")
        .expect("write invalid settings");
    fs::write(dir.path().join("state.toml"), "not = [valid").expect("write invalid state");
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
