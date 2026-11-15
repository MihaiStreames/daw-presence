use crate::daw::DawStatus;
use crate::daw::Project;
use crate::settings::AppSettings;

/// Presence update for the Discord worker, with privacy settings already applied
#[derive(Debug)]
pub struct PresenceRequest {
    pub(super) client_id: String,
    pub(super) presence: DiscordPresence,
}

impl PresenceRequest {
    /// Build a request from current DAW status
    pub fn from_daw_status(daw_status: &DawStatus, settings: &AppSettings) -> Self {
        Self {
            client_id: daw_status.client_id.clone(),
            presence: DiscordPresence::from_daw_status(daw_status, settings),
        }
    }
}

/// Rich Presence data to display on Discord
#[derive(Debug)]
pub(super) struct DiscordPresence {
    pub(super) details: String,
    pub(super) state: String,
    pub(super) large_image: String,
    pub(super) large_text: String,
}

impl DiscordPresence {
    /// Build presence from current DAW status
    pub(super) fn from_daw_status(daw_status: &DawStatus, settings: &AppSettings) -> Self {
        let details = match &daw_status.project {
            _ if settings.hide_project_name => "Opening project: (hidden)".to_owned(),
            Project::Named(name) => format!("Opening project: {name}"),
            Project::Unknown | Project::Untitled => "Opening an untitled project".to_owned(),
        };

        let state = if settings.hide_system_usage {
            format!("Using {}", daw_status.display_name)
        } else {
            let mut parts = Vec::new();

            if let Some(version) = &daw_status.version {
                parts.push(format!("v{version}"));
            }

            parts.push(format!("{} CPU", daw_status.cpu_usage_str()));
            parts.push(format!("{} RAM", daw_status.ram_usage_str()));
            parts.join(", ")
        };

        Self {
            details,
            state,
            large_image: "icon".to_owned(),
            large_text: format!("DAWPresence v{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

#[cfg(test)]
#[path = "../../tests/rust/discord/presence.rs"]
mod tests;
