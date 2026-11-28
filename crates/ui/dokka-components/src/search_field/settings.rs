use dokka_scene::{Color, Rect, Text, TextAlign, TextCursor};
use dokka_theme::settings as measures;

use super::cursor::text_cursor;

/// Texto de la casilla fija de Preferencias: lo escrito a la izquierda tras
/// la lupa; con la casilla vacía, el marcador "Buscar" en el mismo sitio.
/// No cambia el aspecto de la casilla del Launchpad.
pub fn settings_search_text(text_color: Color, bounds: Rect, query: &str) -> Text {
    let content = if query.is_empty() {
        measures::SEARCH_PLACEHOLDER.to_owned()
    } else {
        query.to_owned()
    };
    Text {
        x: bounds.x + measures::SEARCH_TEXT_INSET,
        center_y: bounds.y + bounds.height / 2.0,
        content,
        size: measures::SEARCH_TEXT_SIZE,
        is_bold: false,
        color: text_color,
        align: TextAlign::Left,
    }
}

/// Cursor vertical tras el texto medido (o tras el borde si está vacío).
/// Siempre hay uno: `focused` lo oculta sin destruir la capa al enfocar.
pub fn settings_search_cursor(
    cursor_color: Color,
    bounds: Rect,
    query: &str,
    focused: bool,
) -> TextCursor {
    text_cursor(TextCursor {
        text_x: bounds.x + measures::SEARCH_TEXT_INSET,
        center_y: bounds.y + bounds.height / 2.0,
        content: query.to_owned(),
        size: measures::SEARCH_TEXT_SIZE,
        is_bold: false,
        max_x: bounds.x + bounds.width - measures::SEARCH_TEXT_INSET,
        gap: measures::SEARCH_CURSOR_GAP,
        width: measures::SEARCH_CURSOR_WIDTH,
        height: measures::SEARCH_CURSOR_HEIGHT,
        color: cursor_color,
        visible: focused,
    })
}
