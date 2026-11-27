//! Seguimiento de las ventanas abiertas.

#![allow(unsafe_code)]

mod describe_window;
mod filter_window;
mod find_executable;
mod foreground_app;
mod list_windows;
mod occlusion;
mod settings_entry;
mod watch_changes;

pub use foreground_app::foreground_app;
pub use list_windows::SystemWindows;
pub use occlusion::foreground_occludes_primary;
pub use settings_entry::{SETTINGS_APP_ID, SETTINGS_APP_NAME};
pub use watch_changes::{ChangeWatcher, WINDOWS_CHANGED_MESSAGE};
