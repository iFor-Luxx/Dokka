//! Acciones sobre ventanas ya abiertas y sobre el escritorio.
#![allow(unsafe_code)]

mod brightness;
mod control_window;
mod front_window;
mod keyboard;
mod lock_screen;
mod network;
mod shutdown_dialog;
mod start_menu;
mod taskbar;

pub use brightness::SystemBrightness;

pub use control_window::{SystemWindowControl, foreground_window, window_rect};
pub use keyboard::SystemKeyboard;
pub use lock_screen::lock_screen;
pub use network::SystemNetwork;
pub use shutdown_dialog::show_shutdown_dialog;
pub use start_menu::show_desktop;
pub use taskbar::set_taskbar_visible;
