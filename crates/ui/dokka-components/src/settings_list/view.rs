use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Panel, Rect};
use dokka_theme::{Theme, settings as measures};

/// Panel flotante para esta altura de ventana.
pub fn sidebar_panel_for(theme: &Theme, window_height: f32) -> Panel {
    Panel {
        bounds: Rect {
            x: measures::SIDEBAR_INSET,
            y: measures::SIDEBAR_INSET,
            width: measures::sidebar_panel_width(),
            height: measures::sidebar_panel_height_for(window_height),
        },
        corner_radius: measures::SIDEBAR_RADIUS,
        border_width: measures::SIDEBAR_BORDER_WIDTH,
        tint: theme.settings.sidebar,
        border: theme.settings.sidebar_border,
    }
}

/// Sitio de la casilla de búsqueda bajo los semáforos, dentro del panel.
pub fn search_bounds() -> Rect {
    Rect {
        x: measures::SIDEBAR_INSET + measures::SIDEBAR_MARGIN,
        y: measures::SEARCH_TOP,
        width: measures::search_width(),
        height: measures::SEARCH_HEIGHT,
    }
}

/// Borde superior del cuadrado de color de la fila.
fn badge_top(index: usize) -> f32 {
    measures::row_middle(index) - measures::BADGE_SIDE / 2.0
}

/// Glifos blancos centrados sobre los cuadrados, creados una vez por la isla.
/// El marcador nunca se ve (siempre hay imagen): usa del tema el color del
/// cuadrado que tiene detrás.
pub fn row_icons(theme: &Theme, drawings: &[Arc<Drawing>]) -> Vec<Icon> {
    let inset = (measures::BADGE_SIDE - measures::ROW_ICON_SIDE) / 2.0;
    drawings
        .iter()
        .enumerate()
        .map(|(index, drawing)| Icon {
            x: measures::BADGE_LEFT + inset,
            y: badge_top(index) + inset,
            side: measures::ROW_ICON_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(drawing))),
            placeholder: theme.settings.badge[index],
            placeholder_radius: 0.0,
        })
        .collect()
}

/// Fila que hay en el punto, si cae en su resaltado dentro del panel: la misma
/// columna dibujada.
pub fn row_at(x: f32, y: f32) -> Option<usize> {
    (0..measures::SECTIONS).find(|&index| {
        let top = measures::row_middle(index) - measures::ROW_HIGHLIGHT_HEIGHT / 2.0;
        (measures::row_highlight_left()
            ..measures::row_highlight_left() + measures::row_highlight_width())
            .contains(&x)
            && (top..top + measures::ROW_HIGHLIGHT_HEIGHT).contains(&y)
    })
}
