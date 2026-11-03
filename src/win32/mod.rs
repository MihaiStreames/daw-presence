#![allow(clippy::indexing_slicing)]
pub mod autostart;
pub mod handle;
pub mod process;
pub mod registry;
pub mod single_instance;
pub mod version;
pub mod watcher;
pub mod window;

/// Encode a string as null-terminated UTF-16 for Win32 APIs
pub(super) fn to_wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
