use crate::{
    capabilities::Capabilities,
    ddc::{CommandQueue, DdcBackend, DdcError, RetryPolicy, VcpCode, VcpFeature},
    hdr::{self, DisplayTargetId},
    profile::{MonitorId, PhysicalMonitor},
    snapshot::Snapshot,
};
use log::debug;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayMonitorHandle(isize);

#[cfg(windows)]
impl DisplayMonitorHandle {
    fn from_hmonitor(hmonitor: windows::Win32::Graphics::Gdi::HMONITOR) -> Self {
        Self(hmonitor.0 as isize)
    }

    pub fn as_hmonitor(self) -> windows::Win32::Graphics::Gdi::HMONITOR {
        windows::Win32::Graphics::Gdi::HMONITOR(self.0 as *mut std::ffi::c_void)
    }
}

fn map_display_target_and_monitor_handle(
    source_name: &str,
    physical_sources: &[String],
    paths: &[hdr::DisplayPathIdentity],
    display_monitor: DisplayMonitorHandle,
) -> Result<(DisplayTargetId, DisplayMonitorHandle), DdcError> {
    let display_target = hdr::map_display_target(source_name, physical_sources, paths)?;
    Ok((display_target, display_monitor))
}

#[cfg(windows)]
mod imp {
    use std::{mem::size_of, sync::Arc};

    use parking_lot::Mutex;
    use windows::core::BOOL;
    use windows::Win32::{
        Devices::Display::{
            CapabilitiesRequestAndCapabilitiesReply, DestroyPhysicalMonitor,
            GetCapabilitiesStringLength, GetNumberOfPhysicalMonitorsFromHMONITOR,
            GetPhysicalMonitorsFromHMONITOR, GetVCPFeatureAndVCPFeatureReply, SetVCPFeature,
            MC_VCP_CODE_TYPE, PHYSICAL_MONITOR,
        },
        Foundation::{HANDLE, LPARAM, RECT},
        Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW},
    };

    use super::*;

    pub struct PhysicalMonitorHandle {
        handle: HANDLE,
    }

    impl PhysicalMonitorHandle {
        pub fn new(handle: HANDLE) -> Self {
            Self { handle }
        }

        pub fn raw(&self) -> HANDLE {
            self.handle
        }
    }

    impl Drop for PhysicalMonitorHandle {
        fn drop(&mut self) {
            unsafe {
                let _ = DestroyPhysicalMonitor(self.handle);
            }
        }
    }

    pub struct WindowsDdcBackend {
        handle: PhysicalMonitorHandle,
        gate: Mutex<()>,
    }

    unsafe impl Send for WindowsDdcBackend {}
    unsafe impl Sync for WindowsDdcBackend {}

    impl WindowsDdcBackend {
        fn new(handle: HANDLE) -> Self {
            Self {
                handle: PhysicalMonitorHandle::new(handle),
                gate: Mutex::new(()),
            }
        }

        pub fn capabilities_string(&self) -> Result<String, DdcError> {
            unsafe {
                let mut len = 0;
                if GetCapabilitiesStringLength(self.handle.raw(), &mut len) == 0 {
                    return Err(DdcError::Transient(
                        "GetCapabilitiesStringLength failed".into(),
                    ));
                }
                let mut buffer = vec![0u8; len as usize];
                if CapabilitiesRequestAndCapabilitiesReply(self.handle.raw(), &mut buffer) == 0 {
                    return Err(DdcError::Transient(
                        "CapabilitiesRequestAndCapabilitiesReply failed".into(),
                    ));
                }
                let nul = buffer
                    .iter()
                    .position(|byte| *byte == 0)
                    .unwrap_or(buffer.len());
                Ok(String::from_utf8_lossy(&buffer[..nul]).to_string())
            }
        }
    }

    impl DdcBackend for WindowsDdcBackend {
        fn get_vcp_feature(&self, code: VcpCode) -> Result<VcpFeature, DdcError> {
            unsafe {
                let mut code_type = MC_VCP_CODE_TYPE::default();
                let mut current = 0;
                let mut maximum = 0;
                let ok = GetVCPFeatureAndVCPFeatureReply(
                    self.handle.raw(),
                    code.get(),
                    Some(&mut code_type),
                    &mut current,
                    Some(&mut maximum),
                );
                if ok == 0 {
                    return Err(DdcError::Transient(format!(
                        "GetVCPFeatureAndVCPFeatureReply failed for {code}"
                    )));
                }
                debug!(
                    "get_vcp_feature code={}: current={}, max={}",
                    code, current, maximum
                );
                Ok(VcpFeature {
                    code,
                    current,
                    maximum,
                })
            }
        }

        fn set_vcp_feature(&self, code: VcpCode, value: u32) -> Result<(), DdcError> {
            unsafe {
                if SetVCPFeature(self.handle.raw(), code.get(), value) == 0 {
                    return Err(DdcError::Transient(format!(
                        "SetVCPFeature failed for {code}"
                    )));
                }
                debug!("set_vcp_feature code={}: value={}", code, value);
                Ok(())
            }
        }

        fn synchronization_lock(&self) -> Option<&Mutex<()>> {
            Some(&self.gate)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct MonitorDiagnostics {
        pub capability_error: Option<String>,
        pub parse_error: Option<String>,
        pub display_mapping_error: Option<String>,
    }

    #[derive(Clone)]
    pub struct WindowsMonitor {
        pub info: PhysicalMonitor,
        pub raw_capabilities: Option<String>,
        pub capabilities: Option<Capabilities>,
        pub diagnostics: MonitorDiagnostics,
        pub display_target: Option<DisplayTargetId>,
        pub display_monitor: Option<DisplayMonitorHandle>,
        pub backend: Arc<WindowsDdcBackend>,
    }

    impl WindowsMonitor {
        pub fn queue(&self) -> CommandQueue<Arc<WindowsDdcBackend>> {
            CommandQueue::new(self.backend.clone(), RetryPolicy::default())
        }

        pub fn snapshot(&self) -> Snapshot {
            let queue = self.queue();
            let pairs = self
                .capabilities
                .as_ref()
                .into_iter()
                .flat_map(|caps| caps.vcp_codes())
                .filter_map(|code| {
                    queue
                        .get(VcpCode::new(code))
                        .ok()
                        .map(|feature| (code, feature.current))
                });
            Snapshot::from_pairs(&self.info.description, pairs)
        }
    }

    pub fn enumerate_monitors() -> Result<Vec<WindowsMonitor>, DdcError> {
        struct PendingMonitor {
            source_name: Result<String, String>,
            description: String,
            display_monitor: DisplayMonitorHandle,
            backend: Arc<WindowsDdcBackend>,
        }

        unsafe extern "system" fn enum_proc(
            monitor: HMONITOR,
            _hdc: HDC,
            _rect: *mut RECT,
            data: LPARAM,
        ) -> BOOL {
            let monitors = &mut *(data.0 as *mut Vec<HMONITOR>);
            monitors.push(monitor);
            BOOL(1)
        }

        unsafe {
            let topology_before = hdr::active_display_identities();
            let mut display_monitors = Vec::new();
            let ok = EnumDisplayMonitors(
                None,
                None,
                Some(enum_proc),
                LPARAM(&mut display_monitors as *mut Vec<HMONITOR> as isize),
            );
            if !ok.as_bool() {
                return Err(DdcError::Permanent("EnumDisplayMonitors failed".into()));
            }

            let mut pending_monitors = Vec::new();
            for hmonitor in display_monitors {
                let display_monitor = DisplayMonitorHandle::from_hmonitor(hmonitor);
                let source_name = display_source_name(hmonitor).map_err(|error| error.to_string());
                let mut count = 0;
                GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut count)
                    .map_err(|error| DdcError::Transient(error.message().to_string()))?;
                let mut physical = vec![PHYSICAL_MONITOR::default(); count as usize];
                GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut physical)
                    .map_err(|error| DdcError::Transient(error.message().to_string()))?;

                for physical_monitor in physical {
                    let description_field =
                        std::ptr::addr_of!(physical_monitor.szPhysicalMonitorDescription)
                            .read_unaligned();
                    let handle =
                        std::ptr::addr_of!(physical_monitor.hPhysicalMonitor).read_unaligned();
                    let description = wide_to_string(&description_field);
                    let backend = Arc::new(WindowsDdcBackend::new(handle));

                    pending_monitors.push(PendingMonitor {
                        source_name: source_name.clone(),
                        description,
                        display_monitor,
                        backend,
                    });
                }
            }

            let physical_sources = pending_monitors
                .iter()
                .filter_map(|monitor| monitor.source_name.as_ref().ok().cloned())
                .collect::<Vec<_>>();
            let topology_after = hdr::active_display_identities();
            let active_display_paths = match (topology_before, topology_after) {
                (Ok(before), Ok(after)) if hdr::same_display_topology(&before, &after) => Ok(after),
                (Ok(_), Ok(_)) => Err(DdcError::Transient(
                    "display topology changed during DDC monitor enumeration".into(),
                )),
                (Err(error), _) | (_, Err(error)) => Err(error),
            };
            let mut monitors = Vec::with_capacity(pending_monitors.len());

            for pending in pending_monitors {
                let PendingMonitor {
                    source_name,
                    description,
                    display_monitor,
                    backend,
                } = pending;
                let display_mapping = match (&source_name, &active_display_paths) {
                    (Ok(source_name), Ok(paths)) => map_display_target_and_monitor_handle(
                        source_name,
                        &physical_sources,
                        paths,
                        display_monitor,
                    )
                    .map_err(|error| error.to_string()),
                    (Err(error), _) => Err(error.clone()),
                    (_, Err(error)) => Err(error.to_string()),
                };
                let (display_target, display_monitor, display_mapping_error) = match display_mapping
                {
                    Ok((target, handle)) => (Some(target), Some(handle), None),
                    Err(error) => {
                        debug!("HDR display mapping unavailable for {description}: {error}");
                        (None, None, Some(error))
                    }
                };
                let capabilities_result = backend.capabilities_string();
                let (raw_capabilities, capability_error) = match capabilities_result {
                    Ok(raw) => (Some(raw), None),
                    Err(error) => (None, Some(error.to_string())),
                };
                let (capabilities, parse_error) = match raw_capabilities.as_ref() {
                    Some(raw) => match Capabilities::parse(raw) {
                        Ok(capabilities) => (Some(capabilities), None),
                        Err(error) => (None, Some(error.to_string())),
                    },
                    None => (None, None),
                };
                let model = capabilities.as_ref().and_then(|caps| caps.model.clone());
                let index = monitors.len();

                monitors.push(WindowsMonitor {
                    info: PhysicalMonitor {
                        id: MonitorId(index.to_string()),
                        description,
                        model,
                    },
                    raw_capabilities,
                    capabilities,
                    diagnostics: MonitorDiagnostics {
                        capability_error,
                        parse_error,
                        display_mapping_error,
                    },
                    display_target,
                    display_monitor,
                    backend,
                });
            }

            debug!("enumerated {} monitor(s)", monitors.len());
            Ok(monitors)
        }
    }

    unsafe fn display_source_name(hmonitor: HMONITOR) -> Result<String, DdcError> {
        let mut info = MONITORINFOEXW::default();
        info.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as u32;
        if !GetMonitorInfoW(hmonitor, &mut info.monitorInfo).as_bool() {
            return Err(DdcError::Permanent("GetMonitorInfoW failed".into()));
        }
        let source_name = wide_to_string(&info.szDevice);
        if source_name.is_empty() {
            Err(DdcError::Permanent(
                "GetMonitorInfoW returned an empty display source name".into(),
            ))
        } else {
            Ok(source_name)
        }
    }

    fn wide_to_string(value: &[u16]) -> String {
        let len = value.iter().position(|ch| *ch == 0).unwrap_or(value.len());
        String::from_utf16_lossy(&value[..len])
    }
}

#[cfg(not(windows))]
mod imp {
    use std::sync::Arc;

    use super::*;

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct MonitorDiagnostics {
        pub capability_error: Option<String>,
        pub parse_error: Option<String>,
        pub display_mapping_error: Option<String>,
    }

    pub struct WindowsDdcBackend;

    impl DdcBackend for WindowsDdcBackend {
        fn get_vcp_feature(&self, _code: VcpCode) -> Result<VcpFeature, DdcError> {
            Err(DdcError::Permanent(
                "Windows DDC backend is only available on Windows".into(),
            ))
        }

        fn set_vcp_feature(&self, _code: VcpCode, _value: u32) -> Result<(), DdcError> {
            Err(DdcError::Permanent(
                "Windows DDC backend is only available on Windows".into(),
            ))
        }
    }

    #[derive(Clone)]
    pub struct WindowsMonitor {
        pub info: PhysicalMonitor,
        pub raw_capabilities: Option<String>,
        pub capabilities: Option<Capabilities>,
        pub diagnostics: MonitorDiagnostics,
        pub display_target: Option<DisplayTargetId>,
        pub display_monitor: Option<DisplayMonitorHandle>,
        pub backend: Arc<WindowsDdcBackend>,
    }

    impl WindowsMonitor {
        pub fn queue(&self) -> CommandQueue<Arc<WindowsDdcBackend>> {
            CommandQueue::new(self.backend.clone(), RetryPolicy::default())
        }

        pub fn snapshot(&self) -> Snapshot {
            Snapshot::from_pairs(&self.info.description, [])
        }
    }

    pub fn enumerate_monitors() -> Result<Vec<WindowsMonitor>, DdcError> {
        Ok(Vec::new())
    }
}

pub use imp::{enumerate_monitors, MonitorDiagnostics, WindowsDdcBackend, WindowsMonitor};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdr::DisplayPathIdentity;

    fn path(source_name: &str, monitor_device_path: &str) -> DisplayPathIdentity {
        DisplayPathIdentity {
            source_name: source_name.into(),
            monitor_device_path: monitor_device_path.into(),
        }
    }

    #[test]
    fn unique_display_mapping_retains_the_enumerated_monitor_handle() {
        let handle = DisplayMonitorHandle(0x1234);
        let physical_sources = vec![r"\\.\DISPLAY1".to_string()];
        let paths = vec![path(r"\\.\DISPLAY1", "MONITOR#DELA227")];

        let (target, retained_handle) = map_display_target_and_monitor_handle(
            r"\\.\DISPLAY1",
            &physical_sources,
            &paths,
            handle,
        )
        .unwrap();

        assert_eq!(target.monitor_device_path(), "MONITOR#DELA227");
        assert_eq!(retained_handle, handle);
    }

    #[test]
    fn failed_display_mapping_does_not_produce_a_monitor_handle() {
        let handle = DisplayMonitorHandle(0x1234);
        let physical_sources = vec![r"\\.\DISPLAY1".to_string()];
        let paths = vec![
            path(r"\\.\DISPLAY1", "MONITOR#DELA227"),
            path(r"\\.\DISPLAY1", "MONITOR#ACME0001"),
        ];

        let error = map_display_target_and_monitor_handle(
            r"\\.\DISPLAY1",
            &physical_sources,
            &paths,
            handle,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("multiple Windows targets"));
    }
}
