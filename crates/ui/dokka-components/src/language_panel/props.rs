//! Idiomas instalados, sin saber de dónde vienen.

/// Idioma instalado (como `KeyboardLanguage` del modelo).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageEntry {
    /// Identificador opaco de la distribución.
    pub id: u64,
    /// Nombre completo, como "Español (Perú)".
    pub name: String,
    pub is_active: bool,
}

/// Fila elegible del panel de idioma.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageHit {
    /// Idioma con este identificador.
    Language(u64),
    Settings,
}
