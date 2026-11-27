//! Panel buscador estilo Spotlight de Tahoe.
//!
//! Dos piezas de cristal separadas con un hueco: cápsula grande de búsqueda
//! con lupa arriba y tarjeta de resultados debajo (hasta siete filas con
//! icono de app y nombre, o "Sin resultados"). La fila resaltada es una
//! cápsula translúcida. Radios concéntricos con cada pieza.

use dokka_scene::{DRAWING_SIDE, Rect};

use super::tray_glass as glass;

/// Ancho del buscador, mayor que los demás paneles de la barra.
pub const WIDTH: f32 = 560.0;
/// Hueco entre la cápsula de búsqueda y la tarjeta de resultados.
pub const GAP: f32 = glass::GAP;
/// Aire dentro de la tarjeta de resultados, arriba y abajo.
pub const PADDING: f32 = glass::PADDING;

/// Alto de la cápsula grande de búsqueda.
pub const SEARCH_HEIGHT: f32 = 52.0;
/// Radio de la cápsula: la mitad del alto.
pub const SEARCH_RADIUS: f32 = SEARCH_HEIGHT / 2.0;
/// Lado de la lupa dentro de la cápsula.
pub const SEARCH_ICON_SIDE: f32 = 20.0;
/// Aire entre el borde de la cápsula y la lupa.
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
/// Alto del cursor vertical, acorde a la cápsula grande.
pub const SEARCH_CURSOR_HEIGHT: f32 = 22.0;
/// Texto que se muestra con la consulta vacía.
pub const SEARCH_PLACEHOLDER: &str = "Buscar";

/// Esquinas de la tarjeta de resultados.
pub const RESULTS_RADIUS: f32 = glass::TILE_RADIUS;
pub const RESULTS_BORDER_WIDTH: f32 = glass::TILE_BORDER_WIDTH;
/// Alto de cada fila de resultado.
pub const ROW_HEIGHT: f32 = 44.0;
/// Margen lateral del resaltado dentro de la tarjeta.
pub const ROW_MARGIN: f32 = 6.0;
/// Radio del resaltado: cápsula (mitad del alto de la fila).
pub const ROW_RADIUS: f32 = ROW_HEIGHT / 2.0;
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

/// Lado del icono de la barra superior.
pub const BAR_ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide de la barra: llena el lienzo de 100.
pub const BAR_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo Lucide dentro de la cápsula: llena el lienzo de 100.
pub const FIELD_GLYPH_SIZE: f32 = DRAWING_SIDE;

/// Resultados que se muestran de `total`.
pub fn visible_results(total: usize) -> usize {
    total.min(MAX_RESULTS)
}

/// Alto del panel para `visible` filas; con cero se reserva una para el vacío.
pub fn panel_height(visible: usize) -> f32 {
    let rows = visible.max(1) as f32;
    SEARCH_HEIGHT + GAP + PADDING * 2.0 + rows * ROW_HEIGHT
}

/// Cápsula de búsqueda en lo alto del panel.
pub fn search_box(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y,
        width: bounds.width,
        height: SEARCH_HEIGHT,
    }
}

/// Tarjeta de resultados bajo la cápsula.
pub fn results_box(bounds: Rect) -> Rect {
    let top = bounds.y + SEARCH_HEIGHT + GAP;
    Rect {
        x: bounds.x,
        y: top,
        width: bounds.width,
        height: bounds.height - (top - bounds.y),
    }
}

/// Borde superior de la fila `index` dentro del panel.
pub fn row_top(bounds: Rect, index: usize) -> f32 {
    results_box(bounds).y + PADDING + index as f32 * ROW_HEIGHT
}
