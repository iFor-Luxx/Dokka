//! Tarjeta de música Tahoe sin saber qué es la barra: carátula cuadrada,
//! título y artista recortados con "…" y controles grandes de anterior,
//! pausa/reproducir y siguiente; sin canción, atenuados.

mod icons;
mod props;
mod view;

pub use icons::MusicPanelIcons;
pub use props::{MusicPanelHit, MusicTrack};
pub use view::{music_panel_hit, music_panel_panel, music_panel_size};
