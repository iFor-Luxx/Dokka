//! Isla del Launchpad a pantalla completa.

mod events;
mod logic;
mod state;
mod view;

pub use events::LaunchpadAction;
pub use logic::{filter_apps, merge_apps};
pub use state::Launchpad;
pub use view::launchpad_scene;
