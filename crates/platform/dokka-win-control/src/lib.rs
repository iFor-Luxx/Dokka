//! Acciones sobre ventanas ya abiertas y sobre el escritorio.
#![allow(unsafe_code)]

mod control_window;
mod start_menu;

pub use control_window::{SystemWindowControl, foreground_window};
pub use start_menu::open_start_menu;
