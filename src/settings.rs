use serde::Deserialize;
use serde::Serialize;

use crate::error::ConfigError;

const DEFAULT_UPDATE_INTERVAL: u64 = 2500;
const MIN_UPDATE_INTERVAL: u64 = 1000;
const MAX_UPDATE_INTERVAL: u64 = 100_000_000;

/// User preferences persisted via confy
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub hide_project_name: bool,
    #[serde(default)]
    pub hide_system_usage: bool,
    #[serde(default = "default_true")]
    pub close_to_tray: bool,
    #[serde(default = "default_true")]
    pub was_hidden: bool,
    #[serde(default = "default_update_interval")]
    pub update_interval: u64,
}

const fn default_true() -> bool {
    true
}

const fn default_update_interval() -> u64 {
    DEFAULT_UPDATE_INTERVAL
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hide_project_name: false,
            hide_system_usage: false,
            close_to_tray: true,
            was_hidden: true,
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}

impl AppSettings {
    pub const fn should_start_hidden(&self, is_autostart_launch: bool) -> bool {
        is_autostart_launch && self.was_hidden
    }

    pub fn load() -> Self {
        confy::load("dawpresence", None).unwrap_or_else(|error| {
            tracing::warn!("Couldn't load settings, using defaults: {error}");
            Self::default()
        })
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        confy::store("dawpresence", None, self).map_err(|e| ConfigError::SaveFailed(e.to_string()))
    }

    pub fn set_update_interval(&mut self, interval: u64) -> Result<(), ConfigError> {
        Self::validate_update_interval(interval)?;
        self.update_interval = interval;
        Ok(())
    }

    pub fn validate_update_interval(interval: u64) -> Result<(), ConfigError> {
        if !(MIN_UPDATE_INTERVAL..=MAX_UPDATE_INTERVAL).contains(&interval) {
            return Err(ConfigError::InvalidInterval {
                min: MIN_UPDATE_INTERVAL,
                max: MAX_UPDATE_INTERVAL,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "../tests/rust/settings.rs"]
mod tests;
