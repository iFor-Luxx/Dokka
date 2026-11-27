//! Isla de la barra superior estilo macOS.

mod events;
mod logic;
mod motion;
mod state;
mod view;

pub use events::MenubarAction;
pub use logic::{TrayKind, clock_text};
pub use state::{Menubar, MenubarPainted};
