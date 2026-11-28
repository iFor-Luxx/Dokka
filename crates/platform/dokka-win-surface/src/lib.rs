//! Ventanas propias de Dokka.
#![allow(unsafe_code)]

mod create_window;
mod foreground;
mod installer;
mod installer_event;
mod installer_loop;
mod installer_waker;
mod installer_window;
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
mod work_area;

pub use installer_event::InstallerEvent;
pub use installer_waker::InstallerWaker;
pub use installer_window::InstallerWindow;
pub use local_time::{LocalTime, local_time};
pub use surface::{Surface, SurfaceHandle, SurfaceOptions};
pub use surface_error::SurfaceError;
pub use surface_event::SurfaceEvent;
