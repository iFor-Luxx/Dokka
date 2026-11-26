//! Información y apertura de las aplicaciones instaladas.
#![allow(unsafe_code)]

mod app_name;
mod desktop_catalog;
mod desktop_entry;
mod desktop_paths;
mod filter_target;
mod icon_cache;
mod launch_app;
mod load_icon;
mod pick_app;
mod read_bitmap;
mod read_url;
mod resolve_lnk;
mod system_places;

pub use app_name::NameCache;
pub use desktop_catalog::DesktopCatalog;
pub use icon_cache::IconCache;
pub use launch_app::SystemLauncher;
pub use pick_app::AppPicker;
pub use system_places::{explorer_app, recycle_bin_size, system_place};
