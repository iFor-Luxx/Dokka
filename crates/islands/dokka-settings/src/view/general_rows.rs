//! Filas de la lista de General: etiquetas, cuadritos y resaltado.
//!
//! Los cuadritos, glifos y chevrones van como iconos propios, nunca
//! caracteres. El resaltado siempre existe (de tamaño cero en reposo)
//! para no reconstruir capas al mover el ratón.

use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::painted::PaintedControl;
use super::super::state::Settings;

/// Etiquetas de las filas con el aire del cuadrito a la izquierda.
pub(crate) fn row_labels(
    theme: &Theme,
    top: f32,
    names: &[&str],
    width: f32,
    window_height: f32,
) -> Vec<Text> {
    let badge_left = measures::CONTENT_LEFT + measures::CONTENT_PADDING;
    let text_x = badge_left + measures::BADGE_SIDE + measures::BADGE_TEXT_GAP;
    names
        .iter()
        .enumerate()
        .filter_map(|(index, name)| {
            let y = top
                + index as f32 * measures::CONTENT_ROW_HEIGHT
                + measures::CONTENT_ROW_HEIGHT / 2.0;
            if y < measures::CONTENT_TOP || y > window_height {
                return None;
            }
            let _ = width;
            Some(Text {
                x: text_x,
                center_y: y,
                content: name.to_string(),
                size: measures::CONTENT_LABEL_SIZE,
                is_bold: false,
                color: theme.settings.row_text,
                align: TextAlign::Left,
            })
        })
        .collect()
}

/// Cuadritos, glifos y chevrones de las filas que empiezan en `top`.
pub(crate) fn row_badges(
    theme: &Theme,
    top: f32,
    base: usize,
    badges: &[Arc<Drawing>],
    glyphs: &[Arc<Drawing>],
    chevron: &Arc<Drawing>,
    width: f32,
    window_height: f32,
) -> Vec<Icon> {
    let mut icons = Vec::new();
    let count = if base == 0 { 3 } else { 2 };
    for index in 0..count {
        let row = base + index;
        let y = top + index as f32 * measures::CONTENT_ROW_HEIGHT;
        let middle = y + measures::CONTENT_ROW_HEIGHT / 2.0;
        if middle < measures::CONTENT_TOP || middle > window_height {
            icons.push(hidden_icon());
            icons.push(hidden_icon());
            icons.push(hidden_icon());
            continue;
        }
        let badge_x = measures::CONTENT_LEFT + measures::CONTENT_PADDING;
        let badge_y = y + (measures::CONTENT_ROW_HEIGHT - measures::BADGE_SIDE) / 2.0;
        let badge = badges
            .get(row)
            .cloned()
            .unwrap_or_else(|| badges[0].clone());
        let glyph = glyphs
            .get(row)
            .cloned()
            .unwrap_or_else(|| glyphs[0].clone());
        let inset = (measures::BADGE_SIDE - measures::ROW_ICON_SIDE) / 2.0;
        icons.push(Icon {
            x: badge_x,
            y: badge_y,
            side: measures::BADGE_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(badge)),
            placeholder: theme.settings.badge[row % 8],
            placeholder_radius: measures::BADGE_RADIUS,
        });
        icons.push(Icon {
            x: badge_x + inset,
            y: badge_y + inset,
            side: measures::ROW_ICON_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(glyph)),
            placeholder: theme.settings.badge[row % 8],
            placeholder_radius: 0.0,
        });
        icons.push(Icon {
            x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING - 16.0,
            y: middle - 8.0,
            side: 16.0,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(chevron))),
            placeholder: theme.settings.card_text,
            placeholder_radius: 0.0,
        });
    }
    icons
}

/// Resaltado suave de la fila de General bajo el cursor.
pub(crate) fn hover_panel(
    theme: &Theme,
    settings: &Settings,
    first_top: f32,
    second_top: f32,
    width: f32,
) -> Panel {
    let hovered = match settings.values().control {
        Some(PaintedControl::GeneralRow(row)) => Some(row),
        _ => None,
    };
    let Some(row) = hovered else {
        return Panel {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            corner_radius: measures::REORDER_HIGHLIGHT_RADIUS,
            border_width: 0.0,
            tint: theme.settings.hover,
            border: theme.settings.hover,
        };
    };
    let top = if row < 3 {
        first_top + row as f32 * measures::CONTENT_ROW_HEIGHT
    } else {
        second_top + (row - 3) as f32 * measures::CONTENT_ROW_HEIGHT
    };
    Panel {
        bounds: Rect {
            x: measures::CONTENT_LEFT,
            y: top,
            width,
            height: measures::CONTENT_ROW_HEIGHT,
        },
        corner_radius: measures::REORDER_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    }
}

pub(crate) fn hidden_panel(theme: &Theme) -> Panel {
    Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::CARD_RADIUS,
        border_width: measures::CARD_BORDER_WIDTH,
        tint: theme.settings.card,
        border: theme.settings.card_border,
    }
}

pub(crate) fn hidden_icon() -> Icon {
    Icon {
        x: -1000.0,
        y: -1000.0,
        side: 16.0,
        image: None,
        placeholder: dokka_scene::Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
        placeholder_radius: 0.0,
    }
}

pub(crate) fn hide_above(
    dividers: Vec<dokka_scene::Divider>,
    window_height: f32,
) -> Vec<dokka_scene::Divider> {
    dividers
        .into_iter()
        .map(|mut divider| {
            let middle = divider.bounds.y + divider.bounds.height / 2.0;
            if middle < measures::CONTENT_TOP || middle > window_height {
                divider.bounds = Rect {
                    x: 0.0,
                    y: -1000.0,
                    width: 0.0,
                    height: 0.0,
                };
            }
            divider
        })
        .collect()
}
