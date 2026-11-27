use dokka_scene::{Color, Divider, Rect, Text, TextAlign};
use dokka_theme::settings as measures;

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

/// Cursor vertical tras lo escrito (o tras el borde si está vacío). Siempre
/// hay uno, de ancho cero cuando está oculto, para no reconstruir capas al
/// enfocar: solo se actualiza en posición y tamaño, como el resaltado.
pub fn settings_search_cursor(
    cursor_color: Color,
    bounds: Rect,
    query: &str,
    focused: bool,
) -> Divider {
    if !focused {
        return Divider {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            color: cursor_color,
        };
    }
    Divider {
        bounds: Rect {
            x: settings_text_end(bounds, query),
            y: bounds.y + (bounds.height - measures::SEARCH_CURSOR_HEIGHT) / 2.0,
            width: measures::SEARCH_CURSOR_WIDTH,
            height: measures::SEARCH_CURSOR_HEIGHT,
        },
        color: cursor_color,
    }
}

/// Borde derecho de lo escrito, con un aire antes del cursor. Con la casilla
/// vacía es el borde izquierdo: el avance es aproximado para no medir la
/// fuente en cada redibujado.
fn settings_text_end(bounds: Rect, query: &str) -> f32 {
    let advance = query.chars().count() as f32 * measures::SEARCH_CHAR_ADVANCE;
    let gap = if query.is_empty() {
        0.0
    } else {
        measures::SEARCH_CURSOR_GAP
    };
    bounds.x + measures::SEARCH_TEXT_INSET + advance + gap
}
