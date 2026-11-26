//! Información y apertura de las aplicaciones instaladas.
#![allow(unsafe_code)]

mod icon_cache;
mod launch_app;
mod load_icon;
mod read_bitmap;
mod system_places;

pub use icon_cache::IconCache;
pub use launch_app::SystemLauncher;
pub use system_places::{explorer_app, recycle_bin_has_items, system_place};
