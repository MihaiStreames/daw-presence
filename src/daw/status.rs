pub const UNKNOWN_VERSION: &str = "0.0.0";
pub const UNKNOWN_PROJECT: &str = "None";
pub const UNTITLED_PROJECT: &str = "Untitled";

/// Current state of a detected DAW
#[derive(Debug, Clone, Default)]
pub struct DawStatus {
    pub is_running: bool,
    pub display_name: String,
    pub project_name: String,
    pub cpu_usage: f32,
    pub memory_mb: u64,
    pub version: String,
    pub client_id: String,
    pub hide_version: bool,
}

impl DawStatus {
    pub fn cpu_usage_str(&self) -> String {
        if self.is_running {
            format!("{:.2}%", self.cpu_usage)
        } else {
            "Undefined".to_owned()
        }
    }

    pub fn ram_usage_str(&self) -> String {
        if self.is_running {
            if self.memory_mb >= 1024 {
                format!("{:.2}GB", self.memory_mb as f64 / 1024.0)
            } else {
                format!("{}MB", self.memory_mb)
            }
        } else {
            "Undefined".to_owned()
        }
    }
}

#[cfg(test)]
#[path = "../../tests/rust/daw/status.rs"]
mod tests;
