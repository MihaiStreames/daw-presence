use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use serde::Deserialize;
use serde::Serialize;
use tracing::debug;

use super::title::TitlePattern;
use crate::error::ConfigError;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DawConfigFile {
    version: u32,
    daws: Vec<DawConfig>,
}

/// DAW configuration from `daws.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DawConfig {
    #[serde(rename = "ProcessName")]
    process_name: String,
    #[serde(rename = "DisplayText")]
    display_text: String,
    #[serde(rename = "TitleRegex")]
    title_regex: String,
    #[serde(rename = "ClientID")]
    client_id: String,
    #[serde(rename = "HideVersion")]
    #[serde(default)]
    hide_version: bool,
    #[serde(rename = "AdditionalProcessNames")]
    #[serde(default)]
    additional_process_names: Vec<String>,
}

/// Load DAW configs from a JSON file
pub fn load_configs(path: &Path) -> Result<Vec<DawConfig>, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    let file: DawConfigFile = serde_json::from_str(&content)?;
    Ok(file.daws)
}

/// Ensure a bundled `daws.json` exists in the config directory or
/// overwrite the local copy only when the bundled version is newer
pub fn ensure_daw_config() -> Result<PathBuf, ConfigError> {
    let config_path =
        confy::get_configuration_file_path("dawpresence", None).map_err(|e| ConfigError::InitFailed(e.to_string()))?;
    let config_dir = config_path.parent().ok_or(ConfigError::NoConfigDir)?;
    let daws_path = config_dir.join("daws.json");

    let bundled = include_bytes!("../../daws.json");
    let bundled_version = serde_json::from_slice::<DawConfigFile>(bundled).map_or(0, |f| f.version);

    let local_version = std::fs::read_to_string(&daws_path)
        .ok()
        .and_then(|s| serde_json::from_str::<DawConfigFile>(&s).ok())
        .map_or(0, |f| f.version);

    if local_version < bundled_version {
        std::fs::create_dir_all(config_dir)?;
        std::fs::write(&daws_path, bundled)?;

        debug!(
            "Updated daws.json v{local_version} -> v{bundled_version} at {}",
            daws_path.display()
        );
    }

    Ok(daws_path)
}

/// Pre-normalized DAW config (for fast matching during scanning)
pub(super) struct NormalizedConfig {
    display_text: String,
    client_id: String,
    hide_version: bool,
    title: TitlePattern,
    normalized_name: String,
    additional_prefixes: Vec<String>,
}

impl NormalizedConfig {
    /// Shared so an attached DAW can hold on to its config
    pub(super) fn from_configs(configs: Vec<DawConfig>) -> Vec<Rc<Self>> {
        configs
            .into_iter()
            .map(|config| {
                Rc::new(Self {
                    normalized_name: normalize_process_name(&config.process_name),
                    additional_prefixes: config
                        .additional_process_names
                        .iter()
                        .map(|name| normalize_process_name(name))
                        .collect(),
                    display_text: config.display_text,
                    client_id: config.client_id,
                    hide_version: config.hide_version,
                    title: TitlePattern::new(config.title_regex),
                })
            })
            .collect()
    }

    /// Check if a normalized process name matches this config
    pub(super) fn matches(&self, process_name: &str) -> bool {
        // exact match instead of starts_with (#46)
        process_name == self.normalized_name
            || self
                .additional_prefixes
                .iter()
                .any(|prefix| process_name.starts_with(prefix))
    }

    pub(super) fn display_text(&self) -> &str {
        &self.display_text
    }

    pub(super) const fn title(&self) -> &TitlePattern {
        &self.title
    }

    pub(super) fn client_id(&self) -> &str {
        &self.client_id
    }

    pub(super) const fn hide_version(&self) -> bool {
        self.hide_version
    }
}

/// Normalize a process name for comparison (lowercase, strip .exe)
pub(super) fn normalize_process_name(name: &str) -> String {
    let lower = name.trim().to_lowercase();
    lower.strip_suffix(".exe").unwrap_or(&lower).to_owned()
}

#[cfg(test)]
#[path = "../../tests/rust/daw/config.rs"]
mod tests;
