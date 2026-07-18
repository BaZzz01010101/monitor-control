use log::{debug, info};

use crate::ddc::DdcError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdrState {
    pub supported: bool,
    pub user_enabled: bool,
    pub active: bool,
    pub limited_by_policy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisplayTargetId {
    monitor_device_path: String,
}

impl DisplayTargetId {
    pub(crate) fn new(monitor_device_path: impl Into<String>) -> Self {
        Self {
            monitor_device_path: monitor_device_path.into(),
        }
    }

    pub(crate) fn monitor_device_path(&self) -> &str {
        &self.monitor_device_path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DisplayPathIdentity {
    pub source_name: String,
    pub monitor_device_path: String,
}

const TOPOLOGY_QUERY_ATTEMPTS: usize = 3;
#[cfg(test)]
const ADVANCED_COLOR_MODE_SDR: i32 = 0;
#[cfg(test)]
const ADVANCED_COLOR_MODE_WCG: i32 = 1;
const ADVANCED_COLOR_MODE_HDR: i32 = 2;

#[derive(Debug)]
pub(crate) enum TopologyQueryError {
    InsufficientBuffer,
    Failed(String),
}

pub(crate) fn retry_topology_query<T, F>(mut query: F) -> Result<T, DdcError>
where
    F: FnMut() -> Result<T, TopologyQueryError>,
{
    for attempt in 1..=TOPOLOGY_QUERY_ATTEMPTS {
        match query() {
            Ok(value) => return Ok(value),
            Err(TopologyQueryError::InsufficientBuffer) if attempt < TOPOLOGY_QUERY_ATTEMPTS => {}
            Err(TopologyQueryError::InsufficientBuffer) => {
                return Err(DdcError::Transient(format!(
                    "display topology changed during {TOPOLOGY_QUERY_ATTEMPTS} query attempts"
                )));
            }
            Err(TopologyQueryError::Failed(message)) => {
                return Err(DdcError::Permanent(message));
            }
        }
    }

    unreachable!("the bounded topology query loop always returns")
}

pub(crate) fn map_display_target(
    source_name: &str,
    physical_sources: &[String],
    paths: &[DisplayPathIdentity],
) -> Result<DisplayTargetId, DdcError> {
    if source_name.trim().is_empty() {
        return Err(DdcError::Permanent(
            "DDC monitor has an empty Windows display source".into(),
        ));
    }
    let physical_matches = physical_sources
        .iter()
        .filter(|candidate| candidate.eq_ignore_ascii_case(source_name))
        .count();
    if physical_matches != 1 {
        return Err(DdcError::Permanent(if physical_matches == 0 {
            format!("DDC monitor source {source_name} is unavailable")
        } else {
            format!("multiple DDC monitors share Windows source {source_name}")
        }));
    }

    let matching_paths = paths
        .iter()
        .filter(|path| path.source_name.eq_ignore_ascii_case(source_name))
        .collect::<Vec<_>>();
    let path = match matching_paths.as_slice() {
        [] => {
            return Err(DdcError::Permanent(format!(
                "Windows source {source_name} has no active target"
            )));
        }
        [path] => *path,
        _ => {
            return Err(DdcError::Permanent(format!(
                "Windows source {source_name} maps to multiple Windows targets"
            )));
        }
    };

    if path.monitor_device_path.trim().is_empty() {
        return Err(DdcError::Permanent(format!(
            "Windows source {source_name} has no monitor device path"
        )));
    }

    let target_matches = paths
        .iter()
        .filter(|candidate| {
            candidate
                .monitor_device_path
                .eq_ignore_ascii_case(&path.monitor_device_path)
        })
        .count();
    if target_matches != 1 {
        return Err(DdcError::Permanent(format!(
            "Windows target device path {} is not unique",
            path.monitor_device_path
        )));
    }

    Ok(DisplayTargetId::new(path.monitor_device_path.clone()))
}

pub(crate) fn resolve_target_path<'a>(
    target: &DisplayTargetId,
    paths: &'a [DisplayPathIdentity],
) -> Result<&'a DisplayPathIdentity, DdcError> {
    let matches = paths
        .iter()
        .filter(|path| {
            path.monitor_device_path
                .eq_ignore_ascii_case(target.monitor_device_path())
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Err(DdcError::Permanent(format!(
            "Windows target device path {} is not active",
            target.monitor_device_path()
        ))),
        [path] => Ok(*path),
        _ => Err(DdcError::Permanent(format!(
            "Windows target device path {} is not unique",
            target.monitor_device_path()
        ))),
    }
}

pub(crate) fn same_display_topology(
    left: &[DisplayPathIdentity],
    right: &[DisplayPathIdentity],
) -> bool {
    fn normalized(paths: &[DisplayPathIdentity]) -> Vec<(String, String)> {
        let mut paths = paths
            .iter()
            .map(|path| {
                (
                    path.source_name.to_ascii_lowercase(),
                    path.monitor_device_path.to_ascii_lowercase(),
                )
            })
            .collect::<Vec<_>>();
        paths.sort_unstable();
        paths
    }

    normalized(left) == normalized(right)
}

fn hdr_state_from_raw(flags: u32, active_color_mode: i32) -> HdrState {
    HdrState {
        supported: flags & (1 << 4) != 0,
        user_enabled: flags & (1 << 5) != 0,
        active: flags & (1 << 1) != 0 && active_color_mode == ADVANCED_COLOR_MODE_HDR,
        limited_by_policy: flags & (1 << 3) != 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DisplayRoute {
    adapter_id_low: u32,
    adapter_id_high: i32,
    target_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ActiveDisplayPath {
    identity: DisplayPathIdentity,
    route: DisplayRoute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawHdrState {
    flags: u32,
    active_color_mode: i32,
}

trait DisplayConfigApi {
    fn active_paths(&mut self) -> Result<Vec<ActiveDisplayPath>, DdcError>;
    fn hdr_state(&mut self, route: DisplayRoute) -> Result<RawHdrState, DdcError>;
    fn set_hdr_enabled(&mut self, route: DisplayRoute, enabled: bool) -> Result<(), DdcError>;
}

fn resolve_active_path(
    target: &DisplayTargetId,
    paths: &[ActiveDisplayPath],
) -> Result<DisplayRoute, DdcError> {
    let identities = paths
        .iter()
        .map(|path| path.identity.clone())
        .collect::<Vec<_>>();
    let identity = resolve_target_path(target, &identities)?;
    let path = paths
        .iter()
        .find(|path| path.identity == *identity)
        .expect("resolved identity came from the active path list");
    Ok(path.route)
}

fn hdr_state_with_api(
    api: &mut impl DisplayConfigApi,
    target: &DisplayTargetId,
) -> Result<HdrState, DdcError> {
    let paths = api.active_paths()?;
    let route = resolve_active_path(target, &paths)?;
    let raw = api.hdr_state(route)?;
    Ok(hdr_state_from_raw(raw.flags, raw.active_color_mode))
}

fn set_hdr_enabled_with_api(
    api: &mut impl DisplayConfigApi,
    target: &DisplayTargetId,
    enabled: bool,
) -> Result<HdrState, DdcError> {
    let paths = api.active_paths()?;
    let route = resolve_active_path(target, &paths)?;
    let current_raw = api.hdr_state(route)?;
    let current = hdr_state_from_raw(current_raw.flags, current_raw.active_color_mode);
    if !current.supported {
        return Err(DdcError::Permanent(
            "selected Windows display does not support HDR".into(),
        ));
    }
    if current.limited_by_policy {
        return Err(DdcError::Permanent(
            "HDR changes are blocked by Windows policy".into(),
        ));
    }

    if current.user_enabled != enabled {
        api.set_hdr_enabled(route, enabled)?;
    }

    let confirmed = hdr_state_with_api(api, target)?;
    if confirmed.user_enabled != enabled {
        let requested = if enabled { "enabled" } else { "disabled" };
        return Err(DdcError::Permanent(format!(
            "Windows did not confirm HDR {requested}"
        )));
    }
    Ok(confirmed)
}

#[cfg(windows)]
mod windows_api {
    use std::mem::size_of;

    use windows::Win32::{
        Devices::Display::{
            DisplayConfigGetDeviceInfo, DisplayConfigSetDeviceInfo, GetDisplayConfigBufferSizes,
            QueryDisplayConfig, DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_DEVICE_INFO_TYPE, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME,
            QDC_ONLY_ACTIVE_PATHS,
        },
        Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, LUID},
        Graphics::Gdi::DISPLAYCONFIG_COLOR_ENCODING,
    };

    use super::*;

    const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO_2: DISPLAYCONFIG_DEVICE_INFO_TYPE =
        DISPLAYCONFIG_DEVICE_INFO_TYPE(15);
    const DISPLAYCONFIG_DEVICE_INFO_SET_HDR_STATE: DISPLAYCONFIG_DEVICE_INFO_TYPE =
        DISPLAYCONFIG_DEVICE_INFO_TYPE(16);

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(super) struct DisplayConfigGetAdvancedColorInfo2 {
        pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
        flags: u32,
        color_encoding: DISPLAYCONFIG_COLOR_ENCODING,
        bits_per_color_channel: u32,
        active_color_mode: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(super) struct DisplayConfigSetHdrState {
        pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
        pub flags: u32,
    }

    pub(super) struct SystemDisplayConfigApi;

    impl DisplayConfigApi for SystemDisplayConfigApi {
        fn active_paths(&mut self) -> Result<Vec<ActiveDisplayPath>, DdcError> {
            query_active_paths()?
                .into_iter()
                .map(|path| unsafe {
                    let source_route =
                        DisplayRoute::from_luid(path.sourceInfo.adapterId, path.sourceInfo.id);
                    let target_route =
                        DisplayRoute::from_luid(path.targetInfo.adapterId, path.targetInfo.id);
                    Ok(ActiveDisplayPath {
                        identity: DisplayPathIdentity {
                            source_name: source_name(source_route)?,
                            monitor_device_path: target_name(target_route)?,
                        },
                        route: target_route,
                    })
                })
                .collect()
        }

        fn hdr_state(&mut self, route: DisplayRoute) -> Result<RawHdrState, DdcError> {
            unsafe {
                let mut packet = advanced_color_info_packet(route);
                let result = DisplayConfigGetDeviceInfo(
                    &mut packet.header as *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
                );
                if result != 0 {
                    return Err(DdcError::Permanent(format!(
                        "DisplayConfigGetDeviceInfo(GET_ADVANCED_COLOR_INFO_2) failed: {result}"
                    )));
                }
                Ok(RawHdrState {
                    flags: packet.flags,
                    active_color_mode: packet.active_color_mode,
                })
            }
        }

        fn set_hdr_enabled(&mut self, route: DisplayRoute, enabled: bool) -> Result<(), DdcError> {
            unsafe {
                let packet = hdr_state_packet(route, enabled);
                let result = DisplayConfigSetDeviceInfo(
                    &packet.header as *const DISPLAYCONFIG_DEVICE_INFO_HEADER,
                );
                if result != 0 {
                    return Err(DdcError::Permanent(format!(
                        "DisplayConfigSetDeviceInfo(SET_HDR_STATE) failed: {result}"
                    )));
                }
                Ok(())
            }
        }
    }

    impl DisplayRoute {
        fn from_luid(adapter_id: LUID, target_id: u32) -> Self {
            Self {
                adapter_id_low: adapter_id.LowPart,
                adapter_id_high: adapter_id.HighPart,
                target_id,
            }
        }

        fn adapter_id(self) -> LUID {
            LUID {
                LowPart: self.adapter_id_low,
                HighPart: self.adapter_id_high,
            }
        }
    }

    fn header(
        request_type: DISPLAYCONFIG_DEVICE_INFO_TYPE,
        size: usize,
        route: DisplayRoute,
    ) -> DISPLAYCONFIG_DEVICE_INFO_HEADER {
        DISPLAYCONFIG_DEVICE_INFO_HEADER {
            r#type: request_type,
            size: size as u32,
            adapterId: route.adapter_id(),
            id: route.target_id,
        }
    }

    pub(super) fn advanced_color_info_packet(
        route: DisplayRoute,
    ) -> DisplayConfigGetAdvancedColorInfo2 {
        DisplayConfigGetAdvancedColorInfo2 {
            header: header(
                DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO_2,
                size_of::<DisplayConfigGetAdvancedColorInfo2>(),
                route,
            ),
            flags: 0,
            color_encoding: DISPLAYCONFIG_COLOR_ENCODING::default(),
            bits_per_color_channel: 0,
            active_color_mode: 0,
        }
    }

    pub(super) fn hdr_state_packet(route: DisplayRoute, enabled: bool) -> DisplayConfigSetHdrState {
        DisplayConfigSetHdrState {
            header: header(
                DISPLAYCONFIG_DEVICE_INFO_SET_HDR_STATE,
                size_of::<DisplayConfigSetHdrState>(),
                route,
            ),
            flags: u32::from(enabled),
        }
    }

    fn query_active_paths() -> Result<Vec<DISPLAYCONFIG_PATH_INFO>, DdcError> {
        retry_topology_query(|| unsafe {
            let mut path_count = 0;
            let mut mode_count = 0;
            let size_result = GetDisplayConfigBufferSizes(
                QDC_ONLY_ACTIVE_PATHS,
                &mut path_count,
                &mut mode_count,
            );
            if size_result != ERROR_SUCCESS {
                return Err(TopologyQueryError::Failed(format!(
                    "GetDisplayConfigBufferSizes failed: {}",
                    size_result.0
                )));
            }

            let mut paths = vec![DISPLAYCONFIG_PATH_INFO::default(); path_count as usize];
            let mut modes = vec![DISPLAYCONFIG_MODE_INFO::default(); mode_count as usize];
            let query_result = QueryDisplayConfig(
                QDC_ONLY_ACTIVE_PATHS,
                &mut path_count,
                paths.as_mut_ptr(),
                &mut mode_count,
                modes.as_mut_ptr(),
                None,
            );
            if query_result == ERROR_INSUFFICIENT_BUFFER {
                return Err(TopologyQueryError::InsufficientBuffer);
            }
            if query_result != ERROR_SUCCESS {
                return Err(TopologyQueryError::Failed(format!(
                    "QueryDisplayConfig failed: {}",
                    query_result.0
                )));
            }
            paths.truncate(path_count as usize);
            Ok(paths)
        })
    }

    unsafe fn source_name(route: DisplayRoute) -> Result<String, DdcError> {
        let mut packet = DISPLAYCONFIG_SOURCE_DEVICE_NAME {
            header: header(
                DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
                size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>(),
                route,
            ),
            ..Default::default()
        };
        let result = DisplayConfigGetDeviceInfo(&mut packet.header);
        if result != 0 {
            return Err(DdcError::Permanent(format!(
                "DisplayConfigGetDeviceInfo(GET_SOURCE_NAME) failed: {result}"
            )));
        }
        non_empty_wide_string(&packet.viewGdiDeviceName, "Windows display source name")
    }

    unsafe fn target_name(route: DisplayRoute) -> Result<String, DdcError> {
        let mut packet = DISPLAYCONFIG_TARGET_DEVICE_NAME {
            header: header(
                DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>(),
                route,
            ),
            ..Default::default()
        };
        let result = DisplayConfigGetDeviceInfo(&mut packet.header);
        if result != 0 {
            return Err(DdcError::Permanent(format!(
                "DisplayConfigGetDeviceInfo(GET_TARGET_NAME) failed: {result}"
            )));
        }
        non_empty_wide_string(&packet.monitorDevicePath, "monitor device path")
    }

    fn non_empty_wide_string(value: &[u16], field: &str) -> Result<String, DdcError> {
        let len = value.iter().position(|ch| *ch == 0).unwrap_or(value.len());
        let value = String::from_utf16_lossy(&value[..len]);
        if value.is_empty() {
            Err(DdcError::Permanent(format!("active path has no {field}")))
        } else {
            Ok(value)
        }
    }
}

#[cfg(windows)]
use windows_api::SystemDisplayConfigApi;
#[cfg(all(windows, test))]
use windows_api::{
    advanced_color_info_packet, hdr_state_packet, DisplayConfigGetAdvancedColorInfo2,
    DisplayConfigSetHdrState,
};

#[cfg(windows)]
pub(crate) fn active_display_identities() -> Result<Vec<DisplayPathIdentity>, DdcError> {
    let mut api = SystemDisplayConfigApi;
    let paths = api.active_paths()?;
    debug!("queried {} active Windows display path(s)", paths.len());
    Ok(paths.into_iter().map(|path| path.identity).collect())
}

#[cfg(windows)]
pub fn hdr_state(target: &DisplayTargetId) -> Result<HdrState, DdcError> {
    hdr_state_with_api(&mut SystemDisplayConfigApi, target)
}

#[cfg(not(windows))]
pub fn hdr_state(_target: &DisplayTargetId) -> Result<HdrState, DdcError> {
    Err(DdcError::Permanent(
        "Windows HDR control is only available on Windows".into(),
    ))
}

#[cfg(windows)]
pub fn set_hdr_enabled(target: &DisplayTargetId, enabled: bool) -> Result<HdrState, DdcError> {
    let confirmed = set_hdr_enabled_with_api(&mut SystemDisplayConfigApi, target, enabled)?;
    info!(
        "set HDR target {} to {enabled}",
        target.monitor_device_path()
    );
    Ok(confirmed)
}

#[cfg(not(windows))]
pub fn set_hdr_enabled(_target: &DisplayTargetId, _enabled: bool) -> Result<HdrState, DdcError> {
    Err(DdcError::Permanent(
        "Windows HDR control is only available on Windows".into(),
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn topology_path(source_name: &str, monitor_device_path: &str) -> DisplayPathIdentity {
        DisplayPathIdentity {
            source_name: source_name.into(),
            monitor_device_path: monitor_device_path.into(),
        }
    }

    #[test]
    fn unique_source_maps_to_its_target_device_path() {
        let physical_sources = vec![r"\\.\DISPLAY1".to_string()];
        let paths = vec![topology_path(r"\\.\DISPLAY1", "MONITOR#DELA227")];

        let target = map_display_target(r"\\.\DISPLAY1", &physical_sources, &paths).unwrap();

        assert_eq!(target.monitor_device_path(), "MONITOR#DELA227");
    }

    #[test]
    fn cloned_source_mapping_is_rejected() {
        let physical_sources = vec![r"\\.\DISPLAY1".to_string()];
        let paths = vec![
            topology_path(r"\\.\DISPLAY1", "MONITOR#DELA227"),
            topology_path(r"\\.\DISPLAY1", "MONITOR#ACME0001"),
        ];

        let error = map_display_target(r"\\.\DISPLAY1", &physical_sources, &paths)
            .unwrap_err()
            .to_string();

        assert!(error.contains("multiple Windows targets"));
    }

    #[test]
    fn multiple_physical_monitors_for_one_source_are_rejected() {
        let physical_sources = vec![r"\\.\DISPLAY1".to_string(), r"\\.\display1".to_string()];
        let paths = vec![topology_path(r"\\.\DISPLAY1", "MONITOR#DELA227")];

        let error = map_display_target(r"\\.\DISPLAY1", &physical_sources, &paths)
            .unwrap_err()
            .to_string();

        assert!(error.contains("multiple DDC monitors"));
    }

    #[test]
    fn duplicate_target_path_is_rejected() {
        let physical_sources = vec![r"\\.\DISPLAY1".to_string()];
        let paths = vec![
            topology_path(r"\\.\DISPLAY1", "MONITOR#DELA227"),
            topology_path(r"\\.\DISPLAY2", "monitor#dela227"),
        ];

        let error = map_display_target(r"\\.\DISPLAY1", &physical_sources, &paths)
            .unwrap_err()
            .to_string();

        assert!(error.contains("not unique"));
    }

    #[test]
    fn empty_display_source_identity_is_rejected() {
        let physical_sources = vec![String::new()];
        let paths = vec![topology_path("", "MONITOR#DELA227")];

        let error = map_display_target("", &physical_sources, &paths)
            .unwrap_err()
            .to_string();

        assert!(error.contains("empty Windows display source"));
    }

    #[test]
    fn target_resolution_uses_device_path_after_topology_reordering() {
        let target = DisplayTargetId::new("MONITOR#DELA227");
        let reordered = vec![
            topology_path(r"\\.\DISPLAY2", "MONITOR#ACME0001"),
            topology_path(r"\\.\DISPLAY7", "monitor#dela227"),
        ];

        let resolved = resolve_target_path(&target, &reordered).unwrap();

        assert_eq!(resolved.source_name, r"\\.\DISPLAY7");
    }

    #[test]
    fn topology_stability_ignores_path_order_but_detects_reassignment() {
        let before = vec![
            topology_path(r"\\.\DISPLAY1", "MONITOR#DELA227"),
            topology_path(r"\\.\DISPLAY2", "MONITOR#ACME0001"),
        ];
        let reordered = vec![before[1].clone(), before[0].clone()];
        let reassigned = vec![
            topology_path(r"\\.\DISPLAY1", "MONITOR#ACME0001"),
            topology_path(r"\\.\DISPLAY2", "MONITOR#DELA227"),
        ];

        assert!(same_display_topology(&before, &reordered));
        assert!(!same_display_topology(&before, &reassigned));
    }

    #[test]
    fn stale_target_path_is_rejected() {
        let target = DisplayTargetId::new("MONITOR#DELA227");
        let paths = vec![topology_path(r"\\.\DISPLAY2", "MONITOR#ACME0001")];

        let error = resolve_target_path(&target, &paths)
            .unwrap_err()
            .to_string();

        assert!(error.contains("is not active"));
    }

    #[test]
    fn topology_query_retries_after_insufficient_buffer() {
        let mut attempts = 0;

        let value = retry_topology_query(|| {
            attempts += 1;
            if attempts == 1 {
                Err(TopologyQueryError::InsufficientBuffer)
            } else {
                Ok(42)
            }
        })
        .unwrap();

        assert_eq!(value, 42);
        assert_eq!(attempts, 2);
    }

    #[test]
    fn topology_query_stops_after_three_insufficient_buffers() {
        let mut attempts = 0;

        let error = retry_topology_query::<(), _>(|| {
            attempts += 1;
            Err(TopologyQueryError::InsufficientBuffer)
        })
        .unwrap_err()
        .to_string();

        assert_eq!(attempts, 3);
        assert!(error.contains("changed during 3 query attempts"));
    }

    #[test]
    fn windows_11_flags_distinguish_hdr_from_other_advanced_color() {
        let wcg_only = hdr_state_from_raw(0b1100_0011, ADVANCED_COLOR_MODE_WCG);
        let hdr = hdr_state_from_raw(0b0011_0011, ADVANCED_COLOR_MODE_HDR);

        assert!(!wcg_only.supported);
        assert!(!wcg_only.user_enabled);
        assert!(!wcg_only.active);
        assert!(hdr.supported);
        assert!(hdr.user_enabled);
        assert!(hdr.active);
    }

    #[test]
    fn windows_11_policy_flag_is_exposed() {
        let state = hdr_state_from_raw(0b0001_1001, ADVANCED_COLOR_MODE_SDR);

        assert!(state.supported);
        assert!(!state.user_enabled);
        assert!(!state.active);
        assert!(state.limited_by_policy);
    }

    fn active_path(target_id: u32) -> ActiveDisplayPath {
        ActiveDisplayPath {
            identity: topology_path(r"\\.\DISPLAY1", "MONITOR#DELA227"),
            route: DisplayRoute {
                adapter_id_low: 7,
                adapter_id_high: 0,
                target_id,
            },
        }
    }

    struct FakeDisplayConfigApi {
        topologies: VecDeque<Vec<ActiveDisplayPath>>,
        raw_states: VecDeque<RawHdrState>,
        read_routes: Vec<DisplayRoute>,
        writes: Vec<(DisplayRoute, bool)>,
    }

    impl FakeDisplayConfigApi {
        fn new(topologies: Vec<Vec<ActiveDisplayPath>>, raw_states: Vec<RawHdrState>) -> Self {
            Self {
                topologies: topologies.into(),
                raw_states: raw_states.into(),
                read_routes: Vec::new(),
                writes: Vec::new(),
            }
        }
    }

    impl DisplayConfigApi for FakeDisplayConfigApi {
        fn active_paths(&mut self) -> Result<Vec<ActiveDisplayPath>, DdcError> {
            if self.topologies.len() > 1 {
                Ok(self.topologies.pop_front().unwrap())
            } else {
                Ok(self.topologies.front().cloned().unwrap_or_default())
            }
        }

        fn hdr_state(&mut self, route: DisplayRoute) -> Result<RawHdrState, DdcError> {
            self.read_routes.push(route);
            self.raw_states
                .pop_front()
                .ok_or_else(|| DdcError::Permanent("missing fake HDR state".into()))
        }

        fn set_hdr_enabled(&mut self, route: DisplayRoute, enabled: bool) -> Result<(), DdcError> {
            self.writes.push((route, enabled));
            Ok(())
        }
    }

    fn raw_hdr_state(supported: bool, enabled: bool, limited_by_policy: bool) -> RawHdrState {
        let mut flags = 0;
        if supported {
            flags |= 1 << 4;
        }
        if enabled {
            flags |= (1 << 1) | (1 << 5);
        }
        if limited_by_policy {
            flags |= 1 << 3;
        }
        RawHdrState {
            flags,
            active_color_mode: if enabled {
                ADVANCED_COLOR_MODE_HDR
            } else {
                ADVANCED_COLOR_MODE_SDR
            },
        }
    }

    #[test]
    fn hdr_write_re_resolves_target_and_confirms_the_result() {
        let target = DisplayTargetId::new("MONITOR#DELA227");
        let mut api = FakeDisplayConfigApi::new(
            vec![vec![active_path(3)], vec![active_path(9)]],
            vec![
                raw_hdr_state(true, false, false),
                raw_hdr_state(true, true, false),
            ],
        );

        let confirmed = set_hdr_enabled_with_api(&mut api, &target, true).unwrap();

        assert!(confirmed.user_enabled);
        assert_eq!(api.writes, vec![(active_path(3).route, true)]);
        assert_eq!(
            api.read_routes,
            vec![active_path(3).route, active_path(9).route]
        );
    }

    #[test]
    fn hdr_write_rejects_unsupported_targets() {
        let target = DisplayTargetId::new("MONITOR#DELA227");
        let mut api = FakeDisplayConfigApi::new(
            vec![vec![active_path(3)]],
            vec![raw_hdr_state(false, false, false)],
        );

        let error = set_hdr_enabled_with_api(&mut api, &target, true)
            .unwrap_err()
            .to_string();

        assert!(error.contains("does not support HDR"));
        assert!(api.writes.is_empty());
    }

    #[test]
    fn hdr_write_rejects_policy_limited_targets() {
        let target = DisplayTargetId::new("MONITOR#DELA227");
        let mut api = FakeDisplayConfigApi::new(
            vec![vec![active_path(3)]],
            vec![raw_hdr_state(true, false, true)],
        );

        let error = set_hdr_enabled_with_api(&mut api, &target, true)
            .unwrap_err()
            .to_string();

        assert!(error.contains("blocked by Windows policy"));
        assert!(api.writes.is_empty());
    }

    #[test]
    fn hdr_write_fails_when_windows_does_not_confirm_requested_state() {
        let target = DisplayTargetId::new("MONITOR#DELA227");
        let mut api = FakeDisplayConfigApi::new(
            vec![vec![active_path(3)]],
            vec![
                raw_hdr_state(true, false, false),
                raw_hdr_state(true, false, false),
            ],
        );

        let error = set_hdr_enabled_with_api(&mut api, &target, true)
            .unwrap_err()
            .to_string();

        assert!(error.contains("did not confirm HDR enabled"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_11_hdr_packets_use_hdr_specific_request_types() {
        use std::mem::size_of;

        let route = active_path(3).route;
        let get = advanced_color_info_packet(route);
        let set = hdr_state_packet(route, true);

        assert_eq!(get.header.r#type.0, 15);
        assert_eq!(get.header.size as usize, size_of_val(&get));
        assert_eq!(set.header.r#type.0, 16);
        assert_eq!(set.header.size as usize, size_of_val(&set));
        assert_eq!(set.flags, 1);
        assert_eq!(size_of::<DisplayConfigGetAdvancedColorInfo2>(), 36);
        assert_eq!(size_of::<DisplayConfigSetHdrState>(), 24);
    }
}
