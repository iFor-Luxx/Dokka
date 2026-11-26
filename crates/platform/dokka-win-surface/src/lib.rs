//! Ventanas propias de Dokka.
#![allow(unsafe_code)]

mod create_window;
mod place_window;
mod surface;
mod surface_error;
mod window_procedure;

pub use surface::{Surface, SurfaceHandle, SurfaceOptions};
pub use surface_error::SurfaceError;
