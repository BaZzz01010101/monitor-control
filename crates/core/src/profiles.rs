use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::profile::MonitorProfile;

const U4025QW_PROFILE: &str = include_str!("profiles/u4025qw.toml");

pub fn u4025qw_profile() -> MonitorProfile {
    MonitorProfile::from_toml_str(U4025QW_PROFILE).expect("built-in U4025QW profile must parse")
}

pub fn profile_for_model(model: Option<&str>) -> Option<MonitorProfile> {
    match model {
        Some("U4025QW") => Some(u4025qw_profile()),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct ProfileCatalog {
    profiles: BTreeMap<String, MonitorProfile>,
}

impl ProfileCatalog {
    pub fn load(user_profile_dir: Option<&Path>) -> Result<Self, ProfileCatalogError> {
        let mut profiles = BTreeMap::new();
        let builtin = u4025qw_profile();
        profiles.insert(builtin.identity.model.clone(), builtin);

        if let Some(dir) = user_profile_dir {
            if dir.exists() {
                for entry in fs::read_dir(dir).map_err(ProfileCatalogError::ReadDir)? {
                    let entry = entry.map_err(ProfileCatalogError::ReadDir)?;
                    let path = entry.path();
                    if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                        continue;
                    }

                    let input = fs::read_to_string(&path).map_err(|source| {
                        ProfileCatalogError::ReadProfile {
                            path: path.clone(),
                            source,
                        }
                    })?;
                    let profile = MonitorProfile::from_toml_str(&input).map_err(|source| {
                        ProfileCatalogError::ParseProfile {
                            path: path.clone(),
                            source,
                        }
                    })?;
                    profiles.insert(profile.identity.model.clone(), profile);
                }
            }
        }

        Ok(Self { profiles })
    }

    pub fn for_model(&self, model: &str) -> Option<&MonitorProfile> {
        self.profiles.get(model)
    }
}

#[derive(Debug, Error)]
pub enum ProfileCatalogError {
    #[error("failed to read profile directory: {0}")]
    ReadDir(std::io::Error),
    #[error("failed to read profile `{path}`: {source}")]
    ReadProfile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse profile `{path}`: {source}")]
    ParseProfile {
        path: PathBuf,
        source: toml::de::Error,
    },
}
