//! Acciones sobre ventanas ya abiertas y sobre el escritorio.
#![allow(unsafe_code)]

mod control_window;
mod front_window;
mod start_menu;
mod taskbar;

pub use control_window::{SystemWindowControl, foreground_window, window_rect};
pub use start_menu::show_desktop;
pub use taskbar::set_taskbar_visible;
