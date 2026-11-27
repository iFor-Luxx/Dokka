//! Propiedades del panel de música, sin significado.

/// Canción en curso: título, artista y si suena.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicTrack {
    pub title: String,
    pub artist: String,
    pub is_playing: bool,
}

/// Botón pulsado del panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicPanelHit {
    Previous,
    Toggle,
    Next,
}
