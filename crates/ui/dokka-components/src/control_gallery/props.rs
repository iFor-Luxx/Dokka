//! Lo que muestra la galería, sin saber de dónde sale.

use dokka_model::ControlItem;

use super::super::control_center_panel::ControlCenterProps;

/// Estado que pinta la galería: los datos del centro para dibujar cada
/// control igual que allí, más la categoría elegida y lo escrito.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlGalleryProps {
    pub center: ControlCenterProps,
    /// Índice de la categoría elegida (0: todas).
    pub selected: usize,
    /// Lo escrito en "Buscar controles".
    pub query: String,
}

/// Zona pulsable de la galería.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GalleryHit {
    /// Fila de esta categoría en la columna.
    Category(usize),
    /// Control para añadir al centro.
    Control(ControlItem),
    /// Clic en la casilla: no hace nada y no cierra.
    Search,
    /// Botón azul "Aceptar": cierra la galería y el modo editar.
    Accept,
}
