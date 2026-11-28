//! Resumen del equipo para "Acerca de este PC", en un hilo propio.
#![allow(unsafe_code)]

mod datetime;
mod display;
mod graphics;
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
