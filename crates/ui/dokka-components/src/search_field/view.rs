use std::sync::Arc;

use dokka_scene::{Color, Divider, Drawing, Icon, Panel, Rect, Text, TextAlign};
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
    search_panel_at(
        theme.launchpad.search_background,
        theme.launchpad.search_border,
        search_bounds(props),
        measures::search_radius(props.screen.height),
        measures::SEARCH_BORDER_WIDTH,
    )
}

/// Fondo de una casilla en un sitio fijo, como la de la barra lateral de
/// Preferencias: reutiliza el mismo aspecto sin atarse a sus medidas.
pub fn search_panel_at(
    tint: Color,
    border: Color,
    bounds: Rect,
    corner_radius: f32,
    border_width: f32,
) -> Panel {
    Panel {
        bounds,
        corner_radius,
        border_width,
        tint,
        border,
    }
}

/// Lupa (glifo Lucide): siempre a la izquierda de la casilla, haya foco o no.
/// El dibujo lo crea la isla una vez con `dokka_icons::search_icon` y lo reutiliza.
pub fn search_magnifier(theme: &Theme, props: &SearchFieldProps, drawing: &Arc<Drawing>) -> Icon {
    let bounds = search_bounds(props);
    search_magnifier_at(
        theme.launchpad.search_text,
        bounds.x + measures::SEARCH_ICON_INSET,
        bounds.y + (bounds.height - measures::SEARCH_ICON_SIDE) / 2.0,
        measures::SEARCH_ICON_SIDE,
        drawing,
    )
}

/// Lupa en un punto fijo, como la de Preferencias.
pub fn search_magnifier_at(
    placeholder: Color,
    x: f32,
    y: f32,
    side: f32,
    drawing: &Arc<Drawing>,
) -> Icon {
    Icon {
        x,
        y,
        side,
        image: Some(dokka_scene::IconImage::Drawing(Arc::clone(drawing))),
        placeholder,
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
    search_contains_at(search_bounds(props), x, y)
}

/// `true` si el punto cae dentro de una casilla en un sitio fijo.
pub fn search_contains_at(bounds: Rect, x: f32, y: f32) -> bool {
    (bounds.x..bounds.x + bounds.width).contains(&x)
        && (bounds.y..bounds.y + bounds.height).contains(&y)
}

/// Texto estático de una casilla en un sitio fijo, como el "Buscar" de
/// Preferencias: a la izquierda, sin foco ni cursor.
pub fn search_label_at(color: Color, x: f32, center_y: f32, content: &str, size: f32) -> Text {
    Text {
        x,
        center_y,
        content: content.to_owned(),
        size,
        is_bold: false,
        color,
        align: TextAlign::Left,
    }
}
