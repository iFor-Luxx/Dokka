use std::sync::Arc;

use dokka_scene::Bitmap;

/// Un resultado: nombre e icono de app ya resuelto, si lo hay.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchPanelItem {
    pub name: String,
    pub image: Option<Arc<Bitmap>>,
}

impl SearchPanelItem {
    /// Resultado con nombre e icono opcional.
    pub fn new(name: impl Into<String>, image: Option<Arc<Bitmap>>) -> Self {
        Self {
            name: name.into(),
            image,
        }
    }
}

/// Lo que necesita el panel, sin saber qué es cada app.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchPanelProps {
    /// Lo escrito en la casilla grande.
    pub query: String,
    /// Resultados ya filtrados en su orden; se muestran los 7 primeros.
    pub results: Vec<SearchPanelItem>,
    /// Fila resaltada con flechas u hover, si es visible.
    pub highlighted: Option<usize>,
    /// Fase del parpadeo del cursor; solo se pinta si es `true`.
    pub cursor_visible: bool,
}

/// Fila bajo el punto, sin significado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchHit {
    Result(usize),
}
