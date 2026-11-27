//! Tarjeta de música sin saber qué es la barra: título y artista recortados
//! con "…" y botones de anterior, pausa/reproducir y siguiente.

mod icons;
mod props;
mod view;

pub use icons::MusicPanelIcons;
pub use props::{MusicPanelHit, MusicTrack};
pub use view::{music_panel_hit, music_panel_panel, music_panel_size};
