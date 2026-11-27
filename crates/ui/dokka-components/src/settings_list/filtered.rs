use std::sync::Arc;

use dokka_scene::{Drawing, Icon, IconImage, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

/// Borde superior del cuadrado de color de la fila visible `display`.
fn badge_top(display: usize) -> f32 {
    measures::row_middle(display) - measures::BADGE_SIDE / 2.0
}

/// Panel oculto de tamaño cero con el estilo dado, para no reconstruir capas.
fn hidden_panel(theme: &Theme) -> Panel {
    Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::ROW_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.selection,
        border: theme.settings.selection,
    }
}

/// Cuadrados de color de las filas visibles, en su orden de arriba abajo.
/// Van como imagen para que al filtrar cada capa repinte su color (el
/// marcador de un icono no cambia al actualizarse). `badges` ya viene en
/// orden visible, como `order`.
pub fn filtered_badges(theme: &Theme, order: &[usize], badges: &[Arc<Drawing>]) -> Vec<Icon> {
    order
        .iter()
        .zip(badges.iter())
        .enumerate()
        .map(|(display, (section, badge))| Icon {
            x: measures::BADGE_LEFT,
            y: badge_top(display),
            side: measures::BADGE_SIDE,
            image: Some(IconImage::Drawing(Arc::clone(badge))),
            placeholder: theme.settings.badge[*section],
            placeholder_radius: measures::BADGE_RADIUS,
        })
        .collect()
}

/// Glifos blancos centrados sobre los cuadrados visibles.
pub fn filtered_icons(theme: &Theme, order: &[usize], glyphs: &[Arc<Drawing>]) -> Vec<Icon> {
    let inset = (measures::BADGE_SIDE - measures::ROW_ICON_SIDE) / 2.0;
    order
        .iter()
        .zip(glyphs.iter())
        .enumerate()
        .map(|(display, (section, glyph))| Icon {
            x: measures::BADGE_LEFT + inset,
            y: badge_top(display) + inset,
            side: measures::ROW_ICON_SIDE,
            image: Some(IconImage::Drawing(Arc::clone(glyph))),
            placeholder: theme.settings.badge[*section],
            placeholder_radius: 0.0,
        })
        .collect()
}

/// Nombres de las filas visibles, seguidos desde arriba; la elegida en blanco.
pub fn filtered_labels(
    theme: &Theme,
    selected: usize,
    order: &[usize],
    names: &[String],
) -> Vec<Text> {
    order
        .iter()
        .enumerate()
        .map(|(display, section)| Text {
            x: measures::BADGE_LEFT + measures::BADGE_SIDE + measures::BADGE_TEXT_GAP,
            center_y: measures::row_middle(display),
            content: names[*section].clone(),
            size: measures::ROW_TEXT_SIZE,
            is_bold: false,
            color: if *section == selected {
                theme.settings.row_text_selected
            } else {
                theme.settings.row_text
            },
            align: TextAlign::Left,
        })
        .collect()
}

/// Fondo azul de la fila elegida en su sitio visible; oculto si el filtro la
/// esconde, con el mismo estilo para no reconstruir capas.
pub fn filtered_selection(theme: &Theme, selected: usize, order: &[usize]) -> Panel {
    let Some(display) = order.iter().position(|known| *known == selected) else {
        return hidden_panel(theme);
    };
    Panel {
        bounds: Rect {
            x: measures::SIDEBAR_INSET + measures::ROW_HIGHLIGHT_MARGIN,
            y: measures::row_middle(display) - measures::ROW_HIGHLIGHT_HEIGHT / 2.0,
            width: measures::sidebar_panel_width() - measures::ROW_HIGHLIGHT_MARGIN * 2.0,
            height: measures::ROW_HIGHLIGHT_HEIGHT,
        },
        corner_radius: measures::ROW_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.selection,
        border: theme.settings.selection,
    }
}

/// Fondo suave de la fila visible bajo el cursor; oculto sin fila, sobre la
/// elegida o cuando el filtro la esconde.
pub fn filtered_hover(
    theme: &Theme,
    selected: usize,
    hovered: Option<usize>,
    order: &[usize],
) -> Panel {
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
    let Some(hovered) = hovered else {
        return hidden;
    };
    if hovered == selected {
        return hidden;
    }
    let Some(display) = order.iter().position(|known| *known == hovered) else {
        return hidden;
    };
    Panel {
        bounds: Rect {
            x: measures::SIDEBAR_INSET + measures::ROW_HIGHLIGHT_MARGIN,
            y: measures::row_middle(display) - measures::ROW_HIGHLIGHT_HEIGHT / 2.0,
            width: measures::sidebar_panel_width() - measures::ROW_HIGHLIGHT_MARGIN * 2.0,
            height: measures::ROW_HIGHLIGHT_HEIGHT,
        },
        corner_radius: measures::ROW_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    }
}

/// Fila visible que hay en el punto, siguiendo lo mostrado.
pub fn filtered_row_at(order: &[usize], x: f32, y: f32) -> Option<usize> {
    (0..order.len()).find_map(|display| {
        let top = measures::row_middle(display) - measures::ROW_HIGHLIGHT_HEIGHT / 2.0;
        if (measures::row_highlight_left()
            ..measures::SIDEBAR_WIDTH - measures::ROW_HIGHLIGHT_MARGIN)
            .contains(&x)
            && (top..top + measures::ROW_HIGHLIGHT_HEIGHT).contains(&y)
        {
            Some(order[display])
        } else {
            None
        }
    })
}

/// Mensaje centrado en la barra cuando el filtro no encuentra nada.
pub fn filtered_empty(theme: &Theme) -> Text {
    Text {
        x: measures::SIDEBAR_INSET + measures::sidebar_panel_width() / 2.0,
        center_y: measures::row_middle(0),
        content: measures::EMPTY_TEXT.to_owned(),
        size: measures::EMPTY_TEXT_SIZE,
        is_bold: false,
        color: theme.settings.search_text,
        align: TextAlign::Center,
    }
}
