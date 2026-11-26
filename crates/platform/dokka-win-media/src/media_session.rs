use std::path::Path;

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
