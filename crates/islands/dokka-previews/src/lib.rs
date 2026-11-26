//! Isla de paneles flotantes.

mod events;
mod logic;
mod motion;
mod state;
mod view;

pub use dokka_components::preview_card::{NowPlaying, PreviewWindow};
pub use events::{MediaCommand, PreviewClick, PreviewContent, PreviewRequest};
pub use state::Previews;
