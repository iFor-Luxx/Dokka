//! Vistas en vivo de las ventanas de una app, con su título y controles de música.

mod media_controls;
mod props;
mod view;
mod window_buttons;

pub use media_controls::{MediaIcons, media_buttons};
pub use props::{NowPlaying, PreviewHover, PreviewWindow};
pub use view::{preview_card, preview_size, preview_slot};
pub use window_buttons::{WindowButtonIcons, window_button_areas};
