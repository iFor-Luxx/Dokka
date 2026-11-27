//! Información y apertura de las aplicaciones instaladas.
#![allow(unsafe_code)]

mod app_actions;
mod app_name;
mod desktop_catalog;
mod desktop_entry;
mod desktop_paths;
mod file_icon;
mod filter_target;
mod hicon_bitmap;
mod icon_cache;
mod launch_app;
mod load_icon;
mod pick_app;
mod read_bitmap;
mod read_url;
mod recycle_bin;
mod resolve_lnk;
mod run_as_administrator;
mod show_in_folder;
mod system_places;
mod tray_icon;
mod url_icon;

pub use app_actions::SystemAppActions;
pub use app_name::NameCache;
pub use desktop_catalog::DesktopCatalog;
pub use icon_cache::IconCache;
pub use launch_app::SystemLauncher;
pub use pick_app::AppPicker;
pub use recycle_bin::SystemRecycleBin;
pub use system_places::{explorer_app, recycle_bin_size, system_place};
pub use tray_icon::tray_icon_bitmap;
