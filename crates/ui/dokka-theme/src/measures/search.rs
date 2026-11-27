//! Medidas del panel buscador tipo Spotlight.
//!
//! Cristal claro como `menu_panel` y `calendar_panel`, con la casilla grande
//! arriba y hasta siete filas debajo. Los radios son concéntricos con el
//! panel (interior = exterior - separación), como en Preferencias.

use dokka_scene::{DRAWING_SIDE, Rect};

/// Ancho fijo del panel.
pub const WIDTH: f32 = 560.0;
/// Esquinas del cristal, como los paneles flotantes.
pub const CORNER_RADIUS: f32 = 16.0;
pub const BORDER_WIDTH: f32 = 1.0;
/// Aire inferior bajo la última fila.
pub const PADDING: f32 = 12.0;

/// Separación entre el borde del panel y la casilla o las filas:
/// el radio interior queda en 10, concéntrico con el exterior de 16.
pub const SEARCH_MARGIN: f32 = 6.0;
/// Alto de la casilla grande, mayor que la del Launchpad.
pub const SEARCH_HEIGHT: f32 = 52.0;
/// Radio de la casilla: exterior menos su separación.
pub const SEARCH_RADIUS: f32 = CORNER_RADIUS - SEARCH_MARGIN;
pub const SEARCH_BORDER_WIDTH: f32 = 1.0;
/// Lado de la lupa dentro de la casilla.
pub const SEARCH_ICON_SIDE: f32 = 20.0;
/// Aire entre el borde de la casilla y la lupa.
pub const SEARCH_ICON_INSET: f32 = 14.0;
/// Aire entre la lupa y el texto.
pub const SEARCH_MAGNIFIER_GAP: f32 = 10.0;
/// Distancia del borde izquierdo al texto: lupa más aire a ambos lados.
pub const SEARCH_TEXT_INSET: f32 = SEARCH_ICON_INSET + SEARCH_ICON_SIDE + SEARCH_MAGNIFIER_GAP;
/// Texto grande de la consulta.
pub const SEARCH_TEXT_SIZE: f32 = 17.0;
/// Ancho aproximado de cada letra, para colocar el cursor sin medir la fuente.
pub const SEARCH_CHAR_ADVANCE: f32 = 9.0;
/// Aire entre la última letra y el cursor vertical.
pub const SEARCH_CURSOR_GAP: f32 = 2.0;
/// Ancho del cursor vertical.
pub const SEARCH_CURSOR_WIDTH: f32 = 2.0;
/// Alto del cursor vertical, acorde a la casilla grande.
pub const SEARCH_CURSOR_HEIGHT: f32 = 22.0;
/// Texto que se muestra con la consulta vacía.
pub const SEARCH_PLACEHOLDER: &str = "Buscar";

/// Separación entre la casilla y la primera fila.
pub const GAP: f32 = 8.0;
/// Borde superior de la primera fila, medido desde el borde del panel.
pub const ROWS_TOP: f32 = SEARCH_MARGIN + SEARCH_HEIGHT + GAP;
/// Alto de cada fila de resultado.
pub const ROW_HEIGHT: f32 = 44.0;
/// Margen lateral del resaltado dentro del panel.
pub const ROW_MARGIN: f32 = 6.0;
/// Radio del resaltado, concéntrico con el panel.
pub const ROW_RADIUS: f32 = CORNER_RADIUS - ROW_MARGIN;
/// Lado del icono de app en cada fila.
pub const ROW_ICON_SIDE: f32 = 28.0;
/// Radio del hueco del icono cuando no hay imagen.
pub const ROW_ICON_RADIUS: f32 = 7.0;
/// Aire entre el icono y el nombre.
pub const ROW_TEXT_GAP: f32 = 10.0;
/// Tamaño del nombre de cada resultado.
pub const ROW_TEXT_SIZE: f32 = 14.0;
/// Filas visibles como máximo, como Spotlight.
pub const MAX_RESULTS: usize = 7;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 32;
/// Mensaje cuando el filtro no encuentra nada.
pub const EMPTY_TEXT: &str = "Sin resultados";
/// Tamaño del mensaje de vacío.
pub const EMPTY_TEXT_SIZE: f32 = 14.0;
/// Grosor de la línea bajo la casilla.
pub const SEPARATOR_HEIGHT: f32 = 1.0;

/// Lado del icono de la barra superior.
pub const BAR_ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide de la barra: llena el lienzo de 100.
pub const BAR_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo Lucide dentro del panel: llena el lienzo de 100.
pub const FIELD_GLYPH_SIZE: f32 = DRAWING_SIDE;

/// Resultados que se muestran de `total`.
pub fn visible_results(total: usize) -> usize {
    total.min(MAX_RESULTS)
}

/// Alto del panel para `visible` filas; con cero se reserva una para el vacío.
pub fn panel_height(visible: usize) -> f32 {
    let rows = visible.max(1) as f32;
    ROWS_TOP + rows * ROW_HEIGHT + PADDING
}

/// Casilla de búsqueda dentro del panel.
pub fn search_box(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + SEARCH_MARGIN,
        y: bounds.y + SEARCH_MARGIN,
        width: bounds.width - SEARCH_MARGIN * 2.0,
        height: SEARCH_HEIGHT,
    }
}

/// Borde superior de la fila `index` dentro del panel.
pub fn row_top(bounds: Rect, index: usize) -> f32 {
    bounds.y + ROWS_TOP + index as f32 * ROW_HEIGHT
}
