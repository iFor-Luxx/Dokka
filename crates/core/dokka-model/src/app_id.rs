use std::path::Path;

/// Identidad estable de una aplicación.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppId(String);

impl AppId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Identidad a partir del ejecutable; las rutas de Windows no distinguen mayúsculas.
    pub fn from_executable(path: &Path) -> Self {
        Self(path.to_string_lossy().to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
