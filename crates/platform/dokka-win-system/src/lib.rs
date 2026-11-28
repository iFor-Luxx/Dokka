//! Resumen del equipo para "Acerca de este PC", en un hilo propio.
#![allow(unsafe_code)]

mod apps_size;
mod breakdown;
mod datetime;
mod disks;
mod display;
mod folder_size;
mod graphics;
mod known_folders;
mod locale;
mod machine;
mod monitor;
mod open;
mod placeholders;
mod registry;
mod smbios;
mod storage;
mod summary;

pub use monitor::SystemSystemInfo;
