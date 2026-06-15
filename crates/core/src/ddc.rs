use std::{fmt, str::FromStr, sync::Arc, thread, time::Duration};

use parking_lot::Mutex;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VcpCode(u8);

impl VcpCode {
    pub fn new(code: u8) -> Self {
        Self(code)
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl fmt::Display for VcpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

impl FromStr for VcpCode {
    type Err = DdcError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = parse_u32(value).map_err(|_| DdcError::InvalidVcpCode(value.to_string()))?;
        let code = u8::try_from(parsed).map_err(|_| DdcError::InvalidVcpCode(value.to_string()))?;
        Ok(Self(code))
    }
}

impl Serialize for VcpCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VcpCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = VcpCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a VCP code as an integer or hex string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let code = u8::try_from(value).map_err(|_| E::custom("VCP code exceeds 0xFF"))?;
                Ok(VcpCode(code))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    return Err(E::custom("VCP code cannot be negative"));
                }
                self.visit_u64(value as u64)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                value.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VcpValue(u32);

impl VcpValue {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for VcpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

impl FromStr for VcpValue {
    type Err = DdcError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_u32(value)
            .map(Self)
            .map_err(|_| DdcError::InvalidVcpValue(value.to_string()))
    }
}

impl Serialize for VcpValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VcpValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = VcpValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a VCP value as an integer or hex string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = u32::try_from(value).map_err(|_| E::custom("VCP value exceeds u32"))?;
                Ok(VcpValue(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    return Err(E::custom("VCP value cannot be negative"));
                }
                self.visit_u64(value as u64)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                value.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcpFeature {
    pub code: VcpCode,
    pub current: u32,
    pub maximum: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub attempts: usize,
    pub delay_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            attempts: 3,
            delay_ms: 40,
        }
    }
}

pub trait DdcBackend: Send + Sync + 'static {
    fn get_vcp_feature(&self, code: VcpCode) -> Result<VcpFeature, DdcError>;
    fn set_vcp_feature(&self, code: VcpCode, value: u32) -> Result<(), DdcError>;

    fn synchronization_lock(&self) -> Option<&Mutex<()>> {
        None
    }
}

impl<T> DdcBackend for Arc<T>
where
    T: DdcBackend + ?Sized,
{
    fn get_vcp_feature(&self, code: VcpCode) -> Result<VcpFeature, DdcError> {
        (**self).get_vcp_feature(code)
    }

    fn set_vcp_feature(&self, code: VcpCode, value: u32) -> Result<(), DdcError> {
        (**self).set_vcp_feature(code, value)
    }

    fn synchronization_lock(&self) -> Option<&Mutex<()>> {
        (**self).synchronization_lock()
    }
}

pub struct CommandQueue<B> {
    backend: B,
    retry_policy: RetryPolicy,
    gate: Mutex<()>,
}

impl<B> CommandQueue<B>
where
    B: DdcBackend,
{
    pub fn new(backend: B, retry_policy: RetryPolicy) -> Self {
        Self {
            backend,
            retry_policy,
            gate: Mutex::new(()),
        }
    }

    pub fn get(&self, code: VcpCode) -> Result<VcpFeature, DdcError> {
        if let Some(gate) = self.backend.synchronization_lock() {
            let _guard = gate.lock();
            self.with_retries(|| self.backend.get_vcp_feature(code))
        } else {
            let _guard = self.gate.lock();
            self.with_retries(|| self.backend.get_vcp_feature(code))
        }
    }

    pub fn set(&self, code: VcpCode, value: u32) -> Result<(), DdcError> {
        if let Some(gate) = self.backend.synchronization_lock() {
            let _guard = gate.lock();
            self.with_retries(|| self.backend.set_vcp_feature(code, value))
        } else {
            let _guard = self.gate.lock();
            self.with_retries(|| self.backend.set_vcp_feature(code, value))
        }
    }

    fn with_retries<T>(
        &self,
        mut operation: impl FnMut() -> Result<T, DdcError>,
    ) -> Result<T, DdcError> {
        let attempts = self.retry_policy.attempts.max(1);
        let mut last_error = None;

        for attempt in 0..attempts {
            match operation() {
                Ok(value) => return Ok(value),
                Err(DdcError::Transient(message)) => {
                    last_error = Some(DdcError::Transient(message));
                    if attempt + 1 < attempts && self.retry_policy.delay_ms > 0 {
                        thread::sleep(Duration::from_millis(self.retry_policy.delay_ms));
                    }
                }
                Err(error) => return Err(error),
            }
        }

        Err(last_error.unwrap_or_else(|| DdcError::Transient("operation failed".to_string())))
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DdcError {
    #[error("transient DDC error: {0}")]
    Transient(String),
    #[error("permanent DDC error: {0}")]
    Permanent(String),
    #[error("invalid VCP code: {0}")]
    InvalidVcpCode(String),
    #[error("invalid VCP value: {0}")]
    InvalidVcpValue(String),
}

pub fn parse_u32(value: &str) -> Result<u32, std::num::ParseIntError> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16)
    } else {
        trimmed.parse()
    }
}
