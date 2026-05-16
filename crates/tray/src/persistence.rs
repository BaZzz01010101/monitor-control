use std::{
    env, fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use windows::{
    core::PCWSTR,
    Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH},
};

const SETTINGS_FILE_NAME: &str = "settings.toml";
const STATE_FILE_NAME: &str = "state.toml";
const APP_DIR_NAME: &str = "Dell Controller";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PersistedSettings {
    #[serde(default)]
    pub autostart_enabled: bool,
    #[serde(default = "default_shortcut_value")]
    pub tb_shortcut: String,
    #[serde(default = "default_shortcut_value")]
    pub dp_shortcut: String,
    #[serde(default = "default_shortcut_value")]
    pub hdmi_shortcut: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PersistedWindowPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PersistedAppState {
    #[serde(default)]
    pub window_position: Option<PersistedWindowPosition>,
}

#[derive(Clone, Debug)]
pub struct PersistenceStore {
    base_dir: PathBuf,
}

impl PersistenceStore {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    pub fn for_current_user() -> Result<Self> {
        let app_data = env::var_os("APPDATA").context("APPDATA is not set")?;
        Ok(Self::new(PathBuf::from(app_data).join(APP_DIR_NAME)))
    }

    pub fn load_settings(&self) -> Result<PersistedSettings> {
        self.load_toml_or_default(&self.base_dir.join(SETTINGS_FILE_NAME))
    }

    pub fn save_settings(&self, settings: &PersistedSettings) -> Result<()> {
        self.save_toml(&self.base_dir.join(SETTINGS_FILE_NAME), settings)
    }

    pub fn load_state(&self) -> Result<PersistedAppState> {
        self.load_toml_or_default(&self.base_dir.join(STATE_FILE_NAME))
    }

    pub fn save_state(&self, state: &PersistedAppState) -> Result<()> {
        self.save_toml(&self.base_dir.join(STATE_FILE_NAME), state)
    }

    fn load_toml_or_default<T>(&self, path: &Path) -> Result<T>
    where
        T: Default + for<'de> Deserialize<'de>,
    {
        match fs::read_to_string(path) {
            Ok(contents) => Ok(toml::from_str(&contents).unwrap_or_default()),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(T::default()),
            Err(error) => Err(error)
                .with_context(|| format!("failed to read persisted file {}", path.display())),
        }
    }

    fn save_toml<T>(&self, path: &Path, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("failed to create {}", self.base_dir.display()))?;
        let contents = toml::to_string_pretty(value).context("failed to serialize TOML")?;
        let temp_path = path.with_extension("toml.tmp");
        fs::write(&temp_path, contents)
            .with_context(|| format!("failed to write {}", temp_path.display()))?;
        replace_file_atomically(&temp_path, path)?;
        Ok(())
    }
}

fn default_shortcut_value() -> String {
    "None".into()
}

fn replace_file_atomically(from: &Path, to: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let from_w = encode_wide(from.as_os_str());
        let to_w = encode_wide(to.as_os_str());
        unsafe {
            MoveFileExW(
                PCWSTR(from_w.as_ptr()),
                PCWSTR(to_w.as_ptr()),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        }
        .with_context(|| {
            format!(
                "failed to move persisted file {} into place at {}",
                from.display(),
                to.display()
            )
        })?;
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    {
        fs::rename(from, to).with_context(|| {
            format!(
                "failed to move persisted file {} into place at {}",
                from.display(),
                to.display()
            )
        })?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn encode_wide(value: &std::ffi::OsStr) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    value.encode_wide().chain(std::iter::once(0)).collect()
}
