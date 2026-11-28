//! Resumen del equipo para "Acerca de este PC", en un hilo propio.
#![allow(unsafe_code)]

mod display;
mod graphics;
mod machine;
mod monitor;
mod open;
mod placeholders;
mod registry;
mod smbios;
mod storage;
mod summary;

pub use monitor::SystemSystemInfo;
