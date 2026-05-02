use std::collections::BTreeMap;

use serde::Deserialize;

use crate::ddc::{VcpCode, VcpValue};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub struct MonitorId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMonitor {
    pub id: MonitorId,
    pub description: String,
    pub model: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MonitorProfile {
    pub identity: ProfileIdentity,
    #[serde(default)]
    pub controls: Vec<ControlDefinition>,
    #[serde(default)]
    pub quick_actions: Vec<QuickAction>,
}

impl MonitorProfile {
    pub fn from_toml_str(input: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(input)
    }

    pub fn control(&self, name: &str) -> Option<&ControlDefinition> {
        self.controls.iter().find(|control| control.name == name)
    }

    pub fn control_by_vcp(&self, code: VcpCode) -> Option<&ControlDefinition> {
        self.controls.iter().find(|control| control.vcp == code)
    }

    pub fn quick_action(&self, name: &str) -> Option<&QuickAction> {
        self.quick_actions.iter().find(|action| action.name == name)
    }

    pub fn can_write_control(&self, name: &str) -> bool {
        self.control(name)
            .map(|control| control.safe_write)
            .unwrap_or(false)
    }

    pub fn can_write_vcp(&self, code: u8) -> bool {
        self.control_by_vcp(VcpCode::new(code))
            .map(|control| control.safe_write)
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ProfileIdentity {
    pub manufacturer: String,
    pub model: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ControlDefinition {
    pub name: String,
    pub label: String,
    pub vcp: VcpCode,
    pub kind: ControlKind,
    #[serde(default)]
    pub safe_write: bool,
    #[serde(default)]
    pub values: BTreeMap<VcpValue, String>,
}

impl ControlDefinition {
    pub fn value_label(&self, value: u32) -> Option<&str> {
        self.values
            .get(&VcpValue::new(value))
            .or_else(|| self.values.get(&VcpValue::new(value & 0xFF)))
            .map(String::as_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ControlKind {
    Continuous,
    #[serde(rename = "enum")]
    Enum,
    Momentary,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct QuickAction {
    pub name: String,
    pub label: String,
    pub control: String,
    pub value: VcpValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvmRoute {
    pub input: VcpValue,
    pub usb_upstream: Option<VcpValue>,
}
