//! Core monitor-control library for Dell Controller.

pub mod capabilities;
pub mod ddc;
pub mod hdr;
pub mod profile;
pub mod profiles;
pub mod snapshot;
pub mod startup;
pub mod windows_backend;

pub use capabilities::Capabilities;
pub use ddc::{CommandQueue, DdcBackend, DdcError, RetryPolicy, VcpCode, VcpFeature, VcpValue};
pub use hdr::HdrState;
pub use profile::{
    ControlDefinition, ControlKind, KvmRoute, MonitorId, MonitorProfile, PhysicalMonitor,
};
pub use snapshot::{Snapshot, SnapshotDiff};
pub use windows_backend::{enumerate_monitors, WindowsDdcBackend, WindowsMonitor};
