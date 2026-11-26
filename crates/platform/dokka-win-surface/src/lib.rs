//! Ventanas propias de Dokka.
#![allow(unsafe_code)]

mod create_window;
mod local_time;
mod place_window;
mod read_pointer;
mod run_loop;
mod set_active_area;
mod show_above;
mod show_menu;
mod surface;
mod surface_error;
mod surface_event;
mod window_procedure;

pub use local_time::{LocalTime, local_time};
pub use surface::{Surface, SurfaceHandle, SurfaceOptions};
pub use surface_error::SurfaceError;
pub use surface_event::SurfaceEvent;
