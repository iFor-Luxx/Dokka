use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Panel, Rect, Text, TextAlign};
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

/// Lupa (glifo Lucide): con "Buscar" va justo a su izquierda como grupo
/// centrado; con texto escrito, a la izquierda de la casilla.
/// El dibujo lo crea la isla una vez con `dokka_icons::search_icon` y lo reutiliza.
pub fn search_magnifier(theme: &Theme, props: &SearchFieldProps, drawing: &Arc<Drawing>) -> Icon {
    let bounds = search_bounds(props);
    let y = bounds.y + (bounds.height - measures::SEARCH_ICON_SIDE) / 2.0;
    let x = if props.query.is_empty() {
        bounds.x + bounds.width / 2.0
            - measures::SEARCH_PLACEHOLDER_HALF_WIDTH
            - measures::SEARCH_MAGNIFIER_GAP
            - measures::SEARCH_ICON_SIDE
    } else {
        bounds.x + measures::SEARCH_ICON_INSET
    };
    Icon {
        x,
        y,
        side: measures::SEARCH_ICON_SIDE,
        image: Some(dokka_scene::IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.launchpad.search_text,
        placeholder_radius: 0.0,
    }
}

/// Lo escrito o el texto "Buscar", centrado en la casilla.
pub fn search_label(theme: &Theme, props: &SearchFieldProps) -> Text {
    let bounds = search_bounds(props);
    let content = if props.query.is_empty() {
        measures::SEARCH_PLACEHOLDER.to_owned()
    } else {
        props.query.clone()
    };
    Text {
        x: bounds.x + bounds.width / 2.0,
        center_y: bounds.y + bounds.height / 2.0,
        content,
        size: measures::SEARCH_TEXT_SIZE,
        is_bold: false,
        color: theme.launchpad.search_text,
        align: TextAlign::Center,
    }
}

/// `true` si el punto cae dentro de la casilla.
pub fn search_contains(props: &SearchFieldProps, x: f32, y: f32) -> bool {
    let bounds = search_bounds(props);
    x >= bounds.x && x < bounds.x + bounds.width && y >= bounds.y && y < bounds.y + bounds.height
}
