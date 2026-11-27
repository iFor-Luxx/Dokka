//! Ventanas propias de Dokka.
#![allow(unsafe_code)]

mod create_window;
mod foreground;
mod launchpad_window;
mod local_time;
mod menu_focus;
mod menubar_appbar;
mod menubar_area;
mod menubar_focus;
mod menubar_loop;
mod menubar_window;
mod place_window;
mod preview_cursor;
mod preview_window;
mod read_pointer;
mod run_loop;
mod set_active_area;
mod settings_window;
mod show_above;
mod surface;
mod surface_error;
mod surface_event;
mod window_procedure;

pub use local_time::{LocalTime, local_time};
pub use surface::{Surface, SurfaceHandle, SurfaceOptions};
pub use surface_error::SurfaceError;
pub use surface_event::SurfaceEvent;
