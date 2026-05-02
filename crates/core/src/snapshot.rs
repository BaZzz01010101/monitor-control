use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ddc::VcpCode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub monitor: String,
    pub captured_at: DateTime<Utc>,
    pub values: BTreeMap<VcpCode, u32>,
}

impl Snapshot {
    pub fn from_pairs(
        monitor: impl Into<String>,
        pairs: impl IntoIterator<Item = (u8, u32)>,
    ) -> Self {
        Self {
            monitor: monitor.into(),
            captured_at: Utc::now(),
            values: pairs
                .into_iter()
                .map(|(code, value)| (VcpCode::new(code), value))
                .collect(),
        }
    }

    pub fn value(&self, code: u8) -> Option<u32> {
        self.values.get(&VcpCode::new(code)).copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotDiff {
    pub changed: Vec<ChangedValue>,
    pub added: Vec<SnapshotValue>,
    pub removed: Vec<SnapshotValue>,
}

impl SnapshotDiff {
    pub fn between(before: &Snapshot, after: &Snapshot) -> Self {
        let mut changed = Vec::new();
        let mut added = Vec::new();
        let mut removed = Vec::new();

        for (code, before_value) in &before.values {
            match after.values.get(code) {
                Some(after_value) if after_value != before_value => changed.push(ChangedValue {
                    code: *code,
                    before: *before_value,
                    after: *after_value,
                }),
                Some(_) => {}
                None => removed.push(SnapshotValue {
                    code: *code,
                    value: *before_value,
                }),
            }
        }

        for (code, value) in &after.values {
            if !before.values.contains_key(code) {
                added.push(SnapshotValue {
                    code: *code,
                    value: *value,
                });
            }
        }

        Self {
            changed,
            added,
            removed,
        }
    }

    pub fn changed_value(&self, code: u8) -> Option<(u32, u32)> {
        self.changed
            .iter()
            .find(|value| value.code == VcpCode::new(code))
            .map(|value| (value.before, value.after))
    }

    pub fn added_value(&self, code: u8) -> Option<u32> {
        self.added
            .iter()
            .find(|value| value.code == VcpCode::new(code))
            .map(|value| value.value)
    }

    pub fn removed_value(&self, code: u8) -> Option<u32> {
        self.removed
            .iter()
            .find(|value| value.code == VcpCode::new(code))
            .map(|value| value.value)
    }

    pub fn has_change(&self, code: u8) -> bool {
        self.changed_value(code).is_some()
            || self.added_value(code).is_some()
            || self.removed_value(code).is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedValue {
    pub code: VcpCode,
    pub before: u32,
    pub after: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotValue {
    pub code: VcpCode,
    pub value: u32,
}
