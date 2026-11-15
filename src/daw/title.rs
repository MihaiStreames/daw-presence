use std::cell::OnceCell;

use fancy_regex::Regex;
use tracing::warn;

use super::status::Project;

const UNTITLED_NAME: &str = "Untitled";

/// A DAW's window title regex, compiled the first time it's needed
pub(super) struct TitlePattern {
    pattern: String,
    regex: OnceCell<Option<Regex>>,
}

impl TitlePattern {
    pub(super) const fn new(pattern: String) -> Self {
        Self {
            pattern,
            regex: OnceCell::new(),
        }
    }

    fn regex(&self) -> Option<&Regex> {
        self.regex
            .get_or_init(|| {
                Regex::new(&self.pattern)
                    .map_err(|error| warn!("Invalid regex pattern: {}: {error}", self.pattern))
                    .ok()
            })
            .as_ref()
    }

    /// Check if a title matches the pattern at all (used to filter false-positive windows)
    pub(super) fn matches(&self, title: &str) -> bool {
        self.regex()
            .and_then(|regex| regex.is_match(title).ok())
            .unwrap_or(false)
    }

    /// Read the open project out of a window title
    pub(super) fn project(&self, title: &str) -> Project {
        if title.is_empty() {
            return Project::Unknown;
        }

        let Some(Ok(Some(captures))) = self.regex().map(|regex| regex.captures(title)) else {
            return Project::Unknown;
        };

        // prefer capture group 1 (named match) over group 0 (full match)
        // for patterns without groups

        captures
            .get(1)
            .or_else(|| captures.get(0))
            .map(|matched| matched.as_str().trim().trim_end_matches('*').trim())
            .map_or(Project::Unknown, |name| {
                if name.is_empty() || name == UNTITLED_NAME {
                    Project::Untitled
                } else {
                    Project::Named(name.to_owned())
                }
            })
    }
}

#[cfg(test)]
#[path = "../../tests/rust/daw/title.rs"]
mod tests;
