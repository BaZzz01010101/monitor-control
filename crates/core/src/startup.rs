use crate::ddc::DdcError;

#[cfg(windows)]
pub fn is_autostart_enabled() -> Result<bool, DdcError> {
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
            Ok(true)
        } else if result.0 == 2 {
            Ok(false)
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
