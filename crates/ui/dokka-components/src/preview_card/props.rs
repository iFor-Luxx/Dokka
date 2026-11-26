/// Ventana que se muestra en la tarjeta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewWindow {
    pub window: u64,
    pub title: String,
}

/// Reproducción en curso de la app de la tarjeta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NowPlaying {
    pub is_playing: bool,
}

/// Mini ventana bajo el cursor, con su estado de fijada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreviewHover {
    pub index: usize,
    pub is_pinned: bool,
}
