//! Preferencias guardadas del usuario.

mod pinned_apps_file;
mod preferences_file;
mod taskbar_mode;

pub use pinned_apps_file::PinnedAppsFile;
pub use preferences_file::PreferencesFile;
pub use taskbar_mode::{TaskbarMode, TaskbarModeFile};
