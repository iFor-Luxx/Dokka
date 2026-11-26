use std::f32::consts::TAU;
use std::sync::Arc;

use dokka_scene::{Color, Drawing, Icon, Panel, Rect, Shape, Text, TextAlign};
use dokka_theme::{Theme, launchpad as measures};

use super::SearchFieldProps;

/// Esquinas del aro de la lupa en el lienzo de dibujo.
const RING_CENTER_X: f32 = 36.0;
const RING_CENTER_Y: f32 = 36.0;
const RING_RADIUS: f32 = 20.0;
/// Segmentos que forman el aro.
const RING_SEGMENTS: usize = 16;

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

/// Aro de la lupa con 16 segmentos más su mango. La isla lo crea una vez y lo reutiliza.
pub fn magnifier_drawing(color: Color) -> Drawing {
    let mut shapes = Vec::with_capacity(RING_SEGMENTS + 1);
    for index in 0..RING_SEGMENTS {
        let start = TAU * index as f32 / RING_SEGMENTS as f32;
        let end = TAU * (index + 1) as f32 / RING_SEGMENTS as f32;
        shapes.push(Shape::Line {
            from: (
                RING_CENTER_X + RING_RADIUS * start.cos(),
                RING_CENTER_Y + RING_RADIUS * start.sin(),
            ),
            to: (
                RING_CENTER_X + RING_RADIUS * end.cos(),
                RING_CENTER_Y + RING_RADIUS * end.sin(),
            ),
            width: 10.0,
            color,
        });
    }
    let tilt = TAU / 8.0;
    let along = (tilt.cos(), tilt.sin());
    shapes.push(Shape::Line {
        from: (
            RING_CENTER_X + (RING_RADIUS - 4.0) * along.0,
            RING_CENTER_Y + (RING_RADIUS - 4.0) * along.1,
        ),
        to: (
            RING_CENTER_X + (RING_RADIUS + 22.0) * along.0,
            RING_CENTER_Y + (RING_RADIUS + 22.0) * along.1,
        ),
        width: 11.0,
        color,
    });
    Drawing { shapes }
}

/// Lupa: con "Buscar" va justo a su izquierda como grupo centrado; con texto
/// escrito, a la izquierda de la casilla.
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
