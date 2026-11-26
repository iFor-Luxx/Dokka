use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::SettingsListProps;

/// Fondo translúcido de la barra lateral izquierda.
pub fn sidebar_panel(theme: &Theme) -> Panel {
    Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: measures::SIDEBAR_WIDTH,
            height: measures::HEIGHT,
        },
        corner_radius: measures::CORNER_RADIUS,
        border_width: 0.0,
        tint: theme.settings.sidebar,
        border: theme.settings.sidebar,
    }
}

/// Sitio de la casilla de búsqueda bajo los semáforos.
pub fn search_bounds() -> Rect {
    Rect {
        x: measures::SIDEBAR_MARGIN,
        y: measures::SEARCH_TOP,
        width: measures::search_width(),
        height: measures::SEARCH_HEIGHT,
    }
}

/// Borde superior del cuadrado de color de la fila.
fn badge_top(index: usize) -> f32 {
    measures::row_middle(index) - measures::BADGE_SIDE / 2.0
}

/// Cuadrados de color de cada fila, sin imagen: el glifo blanco va encima.
pub fn row_badges(theme: &Theme) -> Vec<Icon> {
    theme
        .settings
        .badge
        .into_iter()
        .enumerate()
        .map(|(index, color)| Icon {
            x: measures::BADGE_LEFT,
            y: badge_top(index),
            side: measures::BADGE_SIDE,
            image: None,
            placeholder: color,
            placeholder_radius: measures::BADGE_RADIUS,
        })
        .collect()
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

/// Nombres de las secciones; la elegida va en blanco sobre el azul.
pub fn row_labels(theme: &Theme, props: &SettingsListProps, names: &[String]) -> Vec<Text> {
    names
        .iter()
        .enumerate()
        .map(|(index, name)| Text {
            x: measures::BADGE_LEFT + measures::BADGE_SIDE + measures::BADGE_TEXT_GAP,
            center_y: measures::row_middle(index),
            content: name.clone(),
            size: measures::ROW_TEXT_SIZE,
            is_bold: false,
            color: if index == props.selected {
                theme.settings.row_text_selected
            } else {
                theme.settings.row_text
            },
            align: TextAlign::Left,
        })
        .collect()
}

/// Fondo azul de la fila elegida. Siempre tiene el mismo estilo.
pub fn row_selection(theme: &Theme, props: &SettingsListProps) -> Panel {
    Panel {
        bounds: Rect {
            x: measures::ROW_HIGHLIGHT_MARGIN,
            y: measures::row_middle(props.selected) - measures::ROW_HIGHLIGHT_HEIGHT / 2.0,
            width: measures::SIDEBAR_WIDTH - measures::ROW_HIGHLIGHT_MARGIN * 2.0,
            height: measures::ROW_HIGHLIGHT_HEIGHT,
        },
        corner_radius: measures::ROW_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.selection,
        border: theme.settings.selection,
    }
}

/// Fondo suave de la fila bajo el cursor. Sin fila es de tamaño cero y no se
/// ve; sobre la elegida se esconde para no tapar el azul.
pub fn row_hover(theme: &Theme, props: &SettingsListProps) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::ROW_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    };
    let Some(hovered) = props.hovered else {
        return hidden;
    };
    if hovered == props.selected {
        return hidden;
    }
    Panel {
        bounds: Rect {
            x: measures::ROW_HIGHLIGHT_MARGIN,
            y: measures::row_middle(hovered) - measures::ROW_HIGHLIGHT_HEIGHT / 2.0,
            width: measures::SIDEBAR_WIDTH - measures::ROW_HIGHLIGHT_MARGIN * 2.0,
            height: measures::ROW_HIGHLIGHT_HEIGHT,
        },
        corner_radius: measures::ROW_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    }
}

/// Fila que hay en el punto, si cae en su resaltado.
pub fn row_at(x: f32, y: f32) -> Option<usize> {
    (0..measures::SECTIONS).find(|&index| {
        let top = measures::row_middle(index) - measures::ROW_HIGHLIGHT_HEIGHT / 2.0;
        (measures::ROW_HIGHLIGHT_MARGIN..measures::SIDEBAR_WIDTH - measures::ROW_HIGHLIGHT_MARGIN)
            .contains(&x)
            && (top..top + measures::ROW_HIGHLIGHT_HEIGHT).contains(&y)
    })
}
