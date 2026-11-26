/// Ventana que se muestra en la tarjeta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewWindow {
    pub window: u64,
    pub title: String,
}

/// Canción en curso de la app de la tarjeta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NowPlaying {
    pub title: String,
    pub is_playing: bool,
}
