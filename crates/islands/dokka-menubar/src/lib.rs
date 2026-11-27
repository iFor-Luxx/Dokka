//! Isla de la barra superior estilo macOS.

mod events;
mod logic;
mod motion;
mod painted;
mod state;
mod view;

pub use events::MenubarAction;
pub use logic::{TrayKind, clock_text};
pub use painted::MenubarPainted;
pub use state::Menubar;
