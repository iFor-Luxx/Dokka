//! Preferencias guardadas del usuario.

mod pinned_apps_file;
mod preferences_dock;
mod preferences_file;
mod preferences_general;
mod preferences_menubar;
mod taskbar_mode;

pub use pinned_apps_file::PinnedAppsFile;
pub use preferences_file::PreferencesFile;
pub use taskbar_mode::{TaskbarMode, TaskbarModeFile};
