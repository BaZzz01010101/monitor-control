use crate::{
    capabilities::Capabilities,
    ddc::{CommandQueue, DdcBackend, DdcError, RetryPolicy, VcpCode, VcpFeature},
    profile::{MonitorId, PhysicalMonitor},
    snapshot::Snapshot,
};

#[cfg(windows)]
mod imp {
    use std::sync::Arc;

    use windows::core::BOOL;
    use windows::Win32::{
        Devices::Display::{
            CapabilitiesRequestAndCapabilitiesReply, DestroyPhysicalMonitor,
            GetCapabilitiesStringLength, GetNumberOfPhysicalMonitorsFromHMONITOR,
            GetPhysicalMonitorsFromHMONITOR, GetVCPFeatureAndVCPFeatureReply, SetVCPFeature,
            MC_VCP_CODE_TYPE, PHYSICAL_MONITOR,
        },
        Foundation::{HANDLE, LPARAM, RECT},
        Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
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
    }

    unsafe impl Send for WindowsDdcBackend {}
    unsafe impl Sync for WindowsDdcBackend {}

    impl WindowsDdcBackend {
        fn new(handle: HANDLE) -> Self {
            Self {
                handle: PhysicalMonitorHandle::new(handle),
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
                Ok(())
            }
        }
    }

    #[derive(Clone)]
    pub struct WindowsMonitor {
        pub info: PhysicalMonitor,
        pub raw_capabilities: Option<String>,
        pub capabilities: Option<Capabilities>,
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

            let mut monitors = Vec::new();
            for hmonitor in display_monitors {
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
                    let raw_capabilities = backend.capabilities_string().ok();
                    let capabilities = raw_capabilities
                        .as_ref()
                        .and_then(|raw| Capabilities::parse(raw).ok());
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
                        backend,
                    });
                }
            }

            Ok(monitors)
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

pub use imp::{enumerate_monitors, WindowsDdcBackend, WindowsMonitor};
