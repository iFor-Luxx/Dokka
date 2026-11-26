//! Seguimiento de las ventanas abiertas.
#![allow(unsafe_code)]

mod describe_window;
mod filter_window;
mod find_executable;
mod list_windows;
mod watch_changes;

pub use list_windows::SystemWindows;
pub use watch_changes::{ChangeWatcher, WINDOWS_CHANGED_MESSAGE};
