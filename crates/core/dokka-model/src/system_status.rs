/// Volumen general del equipo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeState {
    /// De 0 (silencio) a 1 (máximo).
    pub level: f32,
    pub is_muted: bool,
};

/// Salida de audio activa (punto de render).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioOutput {
    /// Identificador del punto (`IMMDevice::GetId`).
    pub id: String,
    /// Nombre descriptivo (`PKEY_Device_FriendlyName`).
    pub name: String,
    /// `true` si es la predeterminada multimedia.
    pub is_default: bool,
}

/// Cómo está conectado el equipo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkKind {
    Offline,
    Ethernet,
    Wifi,
    Other,
}

/// Conexión de red en uso.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkState {
    pub kind: NetworkKind,
    /// Nombre de la red; vacío si no hay conexión.
    pub name: String,
    pub has_internet: bool,
}

/// Idioma de teclado instalado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardLanguage {
    /// Identificador opaco de la distribución.
    pub id: u64,
    /// Código corto para la barra, como "ES".
    pub code: String,
    /// Nombre completo, como "Español (Perú)".
    pub name: String,
    pub is_active: bool,
}

/// Canción o vídeo que informa una app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NowPlayingInfo {
    pub title: String,
    pub artist: String,
    pub is_playing: bool,
}
