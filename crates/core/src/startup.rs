use crate::ddc::DdcError;

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AutostartEntryState {
    Enabled,
    Disabled,
    Stale,
}

fn autostart_entry_enabled(run_value: Option<&str>, current_exe: &Path) -> AutostartEntryState {
    let Some(run_value) = run_value else {
        return AutostartEntryState::Disabled;
    };
    let Some(run_path) = command_executable_path(run_value) else {
        return AutostartEntryState::Stale;
    };

    if normalized_path_text(&run_path) == normalized_path_text(current_exe) {
        AutostartEntryState::Enabled
    } else {
        AutostartEntryState::Stale
    }
}

fn command_executable_path(command: &str) -> Option<PathBuf> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(rest) = trimmed.strip_prefix('"') {
        let end = rest.find('"')?;
        return Some(PathBuf::from(&rest[..end]));
    }

    trimmed
        .split_whitespace()
        .next()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn normalized_path_text(path: &Path) -> String {
    path.as_os_str()
        .to_string_lossy()
        .replace('/', "\\")
        .trim_matches('"')
        .to_ascii_lowercase()
}

#[cfg(windows)]
pub fn is_autostart_enabled() -> Result<bool, DdcError> {
    let run_value = autostart_run_value()?;
    let exe = std::env::current_exe()
        .map_err(|error| DdcError::Permanent(format!("current_exe failed: {error}")))?;
    Ok(matches!(
        autostart_entry_enabled(run_value.as_deref(), &exe),
        AutostartEntryState::Enabled
    ))
}

#[cfg(windows)]
fn autostart_run_value() -> Result<Option<String>, DdcError> {
    use windows::{
        core::w,
        Win32::{
            Foundation::ERROR_SUCCESS,
            System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_SZ},
        },
    };

    unsafe {
        let mut size = 0;
        let result = RegGetValueW(
            HKEY_CURRENT_USER,
            w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
            w!("DellController"),
            RRF_RT_REG_SZ,
            None,
            None,
            Some(&mut size),
        );

        if result == ERROR_SUCCESS {
            let mut buffer = vec![0u16; (size as usize).div_ceil(std::mem::size_of::<u16>())];
            let result = RegGetValueW(
                HKEY_CURRENT_USER,
                w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
                w!("DellController"),
                RRF_RT_REG_SZ,
                None,
                Some(buffer.as_mut_ptr().cast()),
                Some(&mut size),
            );
            if result != ERROR_SUCCESS {
                return Err(DdcError::Permanent(format!(
                    "RegGetValueW autostart read failed: {}",
                    result.0
                )));
            }
            let len = buffer
                .iter()
                .position(|ch| *ch == 0)
                .unwrap_or(buffer.len());
            Ok(Some(String::from_utf16_lossy(&buffer[..len])))
        } else if result.0 == 2 {
            Ok(None)
        } else {
            Err(DdcError::Permanent(format!(
                "RegGetValueW autostart query failed: {}",
                result.0
            )))
        }
    }
}

#[cfg(windows)]
pub fn set_autostart_enabled(enabled: bool) -> Result<(), DdcError> {
    use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

    use windows::{
        core::{w, PCWSTR},
        Win32::{
            Foundation::ERROR_SUCCESS,
            System::Registry::{RegDeleteKeyValueW, RegSetKeyValueW, HKEY_CURRENT_USER, REG_SZ},
        },
    };

    unsafe {
        if enabled {
            let exe = std::env::current_exe()
                .map_err(|error| DdcError::Permanent(format!("current_exe failed: {error}")))?;
            let command = format!("\"{}\"", exe.display());
            let wide = OsStr::new(&command)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect::<Vec<u16>>();
            let result = RegSetKeyValueW(
                HKEY_CURRENT_USER,
                w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
                w!("DellController"),
                REG_SZ.0,
                Some(wide.as_ptr().cast()),
                (wide.len() * std::mem::size_of::<u16>()) as u32,
            );
            if result != ERROR_SUCCESS {
                return Err(DdcError::Permanent(format!(
                    "RegSetKeyValueW autostart failed: {}",
                    result.0
                )));
            }
        } else {
            let result = RegDeleteKeyValueW(
                HKEY_CURRENT_USER,
                w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
                w!("DellController"),
            );
            if result != ERROR_SUCCESS && result.0 != 2 {
                return Err(DdcError::Permanent(format!(
                    "RegDeleteKeyValueW autostart failed: {}",
                    result.0
                )));
            }
        }

        let _ = PCWSTR::null();
        Ok(())
    }
}

#[cfg(not(windows))]
pub fn is_autostart_enabled() -> Result<bool, DdcError> {
    Ok(false)
}

#[cfg(not(windows))]
pub fn set_autostart_enabled(_enabled: bool) -> Result<(), DdcError> {
    Err(DdcError::Permanent(
        "autostart is only available on Windows".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autostart_entry_matching_current_exe_is_enabled() {
        assert_eq!(
            autostart_entry_enabled(
                Some(r#""C:\Tools\Dell Controller\dell-controller-tray.exe""#),
                Path::new(r#"C:\Tools\Dell Controller\dell-controller-tray.exe"#),
            ),
            AutostartEntryState::Enabled
        );
    }

    #[test]
    fn stale_autostart_entry_is_not_enabled() {
        assert_eq!(
            autostart_entry_enabled(
                Some(r#""C:\Old\dell-controller-tray.exe""#),
                Path::new(r#"C:\Tools\Dell Controller\dell-controller-tray.exe"#),
            ),
            AutostartEntryState::Stale
        );
    }

    #[test]
    fn missing_autostart_entry_is_disabled() {
        assert_eq!(
            autostart_entry_enabled(
                None,
                Path::new(r#"C:\Tools\Dell Controller\dell-controller-tray.exe"#),
            ),
            AutostartEntryState::Disabled
        );
    }
}
