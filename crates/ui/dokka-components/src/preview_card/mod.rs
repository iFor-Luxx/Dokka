//! Vistas en vivo de las ventanas de una app, con su título y controles de música.

mod media_controls;
mod props;
mod view;

pub use media_controls::media_buttons;
pub use props::{NowPlaying, PreviewWindow};
pub use view::{preview_card, preview_size, preview_slot};
