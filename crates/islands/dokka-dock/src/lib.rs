//! Isla del dock.

mod logic;
mod motion;
mod state;
mod view;

pub use logic::{ClickAction, DockAction, HoverKind, HoverTarget, MenuAction};
pub use state::Dock;
pub use view::dock_style::{DOCK_BOTTOM_MARGIN, DOCK_EDGE_MARGIN};
pub use view::system_icons::DockTime;
