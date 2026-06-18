use log::{debug, info};

use crate::ddc::DdcError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdrState {
    pub display_index: usize,
    pub supported: bool,
    pub enabled: bool,
    pub wide_color_enforced: bool,
    pub force_disabled: bool,
    pub bits_per_color_channel: u32,
    pub color_encoding: String,
}

#[cfg(windows)]
pub fn hdr_states() -> Result<Vec<HdrState>, DdcError> {
    use std::mem::size_of;

    use windows::Win32::{
        Devices::Display::{
            DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
            DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO, DISPLAYCONFIG_MODE_INFO,
            DISPLAYCONFIG_PATH_INFO, QDC_ONLY_ACTIVE_PATHS,
        },
        Foundation::ERROR_SUCCESS,
    };

    unsafe {
        let mut path_count = 0;
        let mut mode_count = 0;
        let result =
            GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_count, &mut mode_count);
        if result != ERROR_SUCCESS {
            return Err(DdcError::Permanent(format!(
                "GetDisplayConfigBufferSizes failed: {}",
                result.0
            )));
        }

        let mut paths = vec![DISPLAYCONFIG_PATH_INFO::default(); path_count as usize];
        let mut modes = vec![DISPLAYCONFIG_MODE_INFO::default(); mode_count as usize];
        let result = QueryDisplayConfig(
            QDC_ONLY_ACTIVE_PATHS,
            &mut path_count,
            paths.as_mut_ptr(),
            &mut mode_count,
            modes.as_mut_ptr(),
            None,
        );
        if result != ERROR_SUCCESS {
            return Err(DdcError::Permanent(format!(
                "QueryDisplayConfig failed: {}",
                result.0
            )));
        }
        paths.truncate(path_count as usize);

        let mut states = Vec::new();
        for (index, path) in paths.iter().enumerate() {
            let mut info = DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
                header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO,
                    size: size_of::<DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO>() as u32,
                    adapterId: path.targetInfo.adapterId,
                    id: path.targetInfo.id,
                },
                ..Default::default()
            };

            let result = DisplayConfigGetDeviceInfo(
                &mut info as *mut DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO
                    as *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
            );
            if result != 0 {
                continue;
            }

            let flags = info.Anonymous.value;
            states.push(HdrState {
                display_index: index,
                supported: flags & 0x1 != 0,
                enabled: flags & 0x2 != 0,
                wide_color_enforced: flags & 0x4 != 0,
                force_disabled: flags & 0x8 != 0,
                bits_per_color_channel: info.bitsPerColorChannel,
                color_encoding: format!("{:?}", info.colorEncoding),
            });
        }

        debug!("queried {} display(s) for hdr state", states.len());
        Ok(states)
    }
}

#[cfg(not(windows))]
pub fn hdr_states() -> Result<Vec<HdrState>, DdcError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub fn set_hdr_enabled(display_index: usize, enabled: bool) -> Result<(), DdcError> {
    use std::mem::size_of;

    use windows::Win32::{
        Devices::Display::{
            DisplayConfigSetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
            DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE, DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0,
            QDC_ONLY_ACTIVE_PATHS,
        },
        Foundation::ERROR_SUCCESS,
    };

    unsafe {
        let mut path_count = 0;
        let mut mode_count = 0;
        let result =
            GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_count, &mut mode_count);
        if result != ERROR_SUCCESS {
            return Err(DdcError::Permanent(format!(
                "GetDisplayConfigBufferSizes failed: {}",
                result.0
            )));
        }

        let mut paths = vec![DISPLAYCONFIG_PATH_INFO::default(); path_count as usize];
        let mut modes = vec![DISPLAYCONFIG_MODE_INFO::default(); mode_count as usize];
        let result = QueryDisplayConfig(
            QDC_ONLY_ACTIVE_PATHS,
            &mut path_count,
            paths.as_mut_ptr(),
            &mut mode_count,
            modes.as_mut_ptr(),
            None,
        );
        if result != ERROR_SUCCESS {
            return Err(DdcError::Permanent(format!(
                "QueryDisplayConfig failed: {}",
                result.0
            )));
        }
        paths.truncate(path_count as usize);

        let path = paths.get(display_index).ok_or_else(|| {
            DdcError::Permanent(format!("display index {display_index} not found"))
        })?;
        let state = DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE,
                size: size_of::<DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE>() as u32,
                adapterId: path.targetInfo.adapterId,
                id: path.targetInfo.id,
            },
            Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
                value: u32::from(enabled),
            },
        };

        let result = DisplayConfigSetDeviceInfo(
            &state as *const DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE
                as *const DISPLAYCONFIG_DEVICE_INFO_HEADER,
        );
        if result != 0 {
            return Err(DdcError::Permanent(format!(
                "DisplayConfigSetDeviceInfo failed: {result}"
            )));
        }
        info!("set hdr display {display_index} to {enabled}");
        Ok(())
    }
}

#[cfg(not(windows))]
pub fn set_hdr_enabled(_display_index: usize, _enabled: bool) -> Result<(), DdcError> {
    Err(DdcError::Permanent(
        "Windows HDR control is only available on Windows".into(),
    ))
}
