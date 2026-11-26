use std::sync::Arc;

use dokka_scene::{Divider, Drawing, Icon, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, launchpad as measures};

use super::SearchFieldProps;

/// Esquina superior izquierda y tamaño de la casilla, centrada.
fn search_bounds(props: &SearchFieldProps) -> Rect {
    let width = measures::search_width(props.screen.width);
    let height = measures::search_height(props.screen.height);
    Rect {
        x: (props.screen.width - width) / 2.0,
        y: measures::search_top(props.screen.height),
        width,
        height,
    }
}

/// Borde izquierdo donde empieza el texto o el marcador: tras la lupa.
fn text_left(props: &SearchFieldProps) -> f32 {
    search_bounds(props).x + measures::SEARCH_TEXT_INSET
}

/// Fondo de la casilla: panel nítido con tinte, borde fino y radio.
pub fn search_panel(theme: &Theme, props: &SearchFieldProps) -> Panel {
    Panel {
        bounds: search_bounds(props),
        corner_radius: measures::search_radius(props.screen.height),
        border_width: measures::SEARCH_BORDER_WIDTH,
        tint: theme.launchpad.search_background,
        border: theme.launchpad.search_border,
    }
}

/// Lupa (glifo Lucide): siempre a la izquierda de la casilla, haya foco o no.
/// El dibujo lo crea la isla una vez con `dokka_icons::search_icon` y lo reutiliza.
pub fn search_magnifier(theme: &Theme, props: &SearchFieldProps, drawing: &Arc<Drawing>) -> Icon {
    let bounds = search_bounds(props);
    Icon {
        x: bounds.x + measures::SEARCH_ICON_INSET,
        y: bounds.y + (bounds.height - measures::SEARCH_ICON_SIDE) / 2.0,
        side: measures::SEARCH_ICON_SIDE,
        image: Some(dokka_scene::IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.launchpad.search_text,
        placeholder_radius: 0.0,
    }
}

/// Lo escrito a la izquierda tras la lupa; con la casilla vacía, el marcador
/// "Buscar" también a la izquierda: tenue junto al cursor si hay foco, del
/// mismo blanco si no lo hay.
pub fn search_label(theme: &Theme, props: &SearchFieldProps) -> Text {
    let bounds = search_bounds(props);
    let middle = bounds.y + bounds.height / 2.0;
    if props.query.is_empty() {
        let (x, color) = if props.focused {
            (text_end(props), theme.launchpad.search_placeholder)
        } else {
            (text_left(props), theme.launchpad.search_text)
        };
        return Text {
            x,
            center_y: middle,
            content: measures::SEARCH_PLACEHOLDER.to_owned(),
            size: measures::SEARCH_TEXT_SIZE,
            is_bold: false,
            color,
            align: TextAlign::Left,
        };
    }
    Text {
        x: text_left(props),
        center_y: middle,
        content: props.query.clone(),
        size: measures::SEARCH_TEXT_SIZE,
        is_bold: false,
        color: theme.launchpad.search_text,
        align: TextAlign::Left,
    }
}

/// Cursor vertical tras el texto (o tras el borde si está vacío). Siempre hay
/// uno, de ancho cero cuando está oculto, para no reconstruir capas al
/// parpadear: solo se actualiza en posición y tamaño, como el resaltado.
pub fn search_cursor(theme: &Theme, props: &SearchFieldProps) -> Divider {
    let bounds = search_bounds(props);
    if !props.focused || !props.cursor_visible {
        return Divider {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            color: theme.launchpad.search_text,
        };
    }
    Divider {
        bounds: Rect {
            x: text_end(props),
            y: bounds.y + (bounds.height - measures::SEARCH_CURSOR_HEIGHT) / 2.0,
            width: measures::SEARCH_CURSOR_WIDTH,
            height: measures::SEARCH_CURSOR_HEIGHT,
        },
        color: theme.launchpad.search_text,
    }
}

/// Borde derecho del texto escrito, con un aire antes del cursor o del marcador.
/// Con la casilla vacía es el borde izquierdo: el avance es aproximado para no
/// medir la fuente en cada redibujado.
fn text_end(props: &SearchFieldProps) -> f32 {
    let advance = props.query.chars().count() as f32 * measures::SEARCH_CHAR_ADVANCE;
    let gap = if props.query.is_empty() {
        0.0
    } else {
        measures::SEARCH_CURSOR_GAP
    };
    text_left(props) + advance + gap
}

/// `true` si el punto cae dentro de la casilla.
pub fn search_contains(props: &SearchFieldProps, x: f32, y: f32) -> bool {
    let bounds = search_bounds(props);
    x >= bounds.x && x < bounds.x + bounds.width && y >= bounds.y && y < bounds.y + bounds.height
}
