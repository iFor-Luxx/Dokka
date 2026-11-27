//! Medidas del panel de apps en segundo plano que cuelga de la barra.
//!
//! Estilo Tahoe como `language_panel`: un solo cristal con título y una
//! fila compacta por app (círculo suave con su icono real y nombre).
//! El alto de la ventana (`menubar::WINDOW_HEIGHT`, 480 menos franja de
//! 30 y hueco de 6 = 444) deja sitio para diez filas como máximo.

use super::tray_glass;

/// Ancho fijo del panel (el de los menús Tahoe).
pub const PANEL_WIDTH: f32 = tray_glass::WIDTH;
/// Aire dentro del cristal único.
pub const PADDING: f32 = tray_glass::PADDING;
/// Esquinas y borde del cristal único.
pub const PANEL_RADIUS: f32 = tray_glass::PANEL_RADIUS;
pub const PANEL_BORDER_WIDTH: f32 = tray_glass::PANEL_BORDER_WIDTH;
/// Alto de la fila del título.
pub const TITLE_HEIGHT: f32 = tray_glass::TITLE_HEIGHT;
/// Separación entre el título y la primera fila.
pub const TITLE_GAP: f32 = 6.0;
/// Alto de cada fila compacta y separación entre filas.
pub const ROW_HEIGHT: f32 = tray_glass::ROW_HEIGHT;
pub const ROW_GAP: f32 = 4.0;
/// Círculo suave de cada fila y lado del icono real dentro.
pub const CIRCLE: f32 = 26.0;
pub const CIRCLE_ICON: f32 = 16.0;
/// Aire entre el círculo y el nombre.
pub const ICON_GAP: f32 = 10.0;
/// Radio del resaltado suave de cada fila, concéntrico.
pub const HIGHLIGHT_RADIUS: f32 = tray_glass::HIGHLIGHT_RADIUS;
/// Tamaños de texto y de título.
pub const TITLE_SIZE: f32 = tray_glass::TITLE_SIZE;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 28;
/// Filas visibles como máximo (10 × 34 + 9 × 4 + título y aires = 426).
pub const MAX_ROWS: usize = 10;
/// Título del panel y mensaje cuando no hay apps.
pub const TITLE: &str = "Segundo plano";
pub const EMPTY_TEXT: &str = "Sin apps en segundo plano";
/// Tamaño del mensaje de vacío.
pub const EMPTY_TEXT_SIZE: f32 = tray_glass::TITLE_SIZE;
/// Nombre cuando la app no trae título.
pub const FALLBACK_NAME: &str = "Aplicación";

/// Filas que se muestran de `total`.
pub fn visible_rows(total: usize) -> usize {
    total.min(MAX_ROWS)
}

/// Alto del panel para `visible` filas; con cero se reserva una para el vacío.
pub fn panel_height(visible: usize) -> f32 {
    let rows = visible.max(1) as f32;
    let gaps = (visible.max(1).saturating_sub(1)) as f32 * ROW_GAP;
    PADDING + TITLE_HEIGHT + TITLE_GAP + rows * ROW_HEIGHT + gaps + PADDING
}
