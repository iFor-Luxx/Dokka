//! Información y apertura de las aplicaciones instaladas.
#![allow(unsafe_code)]

mod icon_cache;
mod launch_app;
mod load_icon;
mod read_bitmap;

pub use icon_cache::IconCache;
pub use launch_app::SystemLauncher;
