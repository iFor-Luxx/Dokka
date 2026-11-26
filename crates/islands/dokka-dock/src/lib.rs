//! Isla del dock.

mod logic;
mod motion;
mod state;
mod view;

pub use logic::{ClickAction, DockAction};
pub use state::Dock;
pub use view::dock_style::DOCK_BOTTOM_MARGIN;
