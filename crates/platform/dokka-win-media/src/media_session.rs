use std::path::Path;

use dokka_model::NowPlayingInfo;

/// Lo que una app está reproduciendo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaSession {
    /// Identificador que la app declara a Windows.
    pub source: String,
    pub title: String,
    pub artist: String,
    pub is_playing: bool,
}

impl MediaSession {
    /// Canción o vídeo en el tipo que espera la barra (`dokka_model`).
    pub fn now_playing(&self) -> NowPlayingInfo {
        NowPlayingInfo {
            title: self.title.clone(),
            artist: self.artist.clone(),
            is_playing: self.is_playing,
        }
    }

    /// Compara el identificador declarado con el nombre del ejecutable.
    pub fn belongs_to(&self, executable: &Path) -> bool {
        let Some(name) = executable.file_stem() else {
            return false;
        };
        let name = name.to_string_lossy().to_lowercase();
        let source = self.source.to_lowercase();
        let source = source.strip_suffix(".exe").unwrap_or(&source);
        !source.is_empty() && (source.contains(&name) || name.contains(source))
    }
}
