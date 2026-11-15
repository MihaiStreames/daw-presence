/// What a DAW's window title says about the open project
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Project {
    /// No usable window title, e.g. the DAW is still loading
    #[default]
    Unknown,
    /// A project is open but was never saved under a name
    Untitled,
    Named(String),
}

/// Current state of a detected DAW
#[derive(Debug, Clone, Default)]
pub struct DawStatus {
    pub display_name: String,
    pub project: Project,
    pub cpu_usage: f32,
    pub memory_mb: u64,
    /// `None` when it couldn't be read or the DAW's config hides it
    pub version: Option<String>,
    pub client_id: String,
}

impl DawStatus {
    pub fn cpu_usage_str(&self) -> String {
        format!("{:.2}%", self.cpu_usage)
    }

    pub fn ram_usage_str(&self) -> String {
        if self.memory_mb >= 1024 {
            format!("{:.2}GB", self.memory_mb as f64 / 1024.0)
        } else {
            format!("{}MB", self.memory_mb)
        }
    }
}

#[cfg(test)]
#[path = "../../tests/rust/daw/status.rs"]
mod tests;
