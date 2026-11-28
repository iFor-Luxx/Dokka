//! Burbuja "Editar controles" y botones "−" del modo editar.
//!
//! La burbuja va bajo la rejilla solo sin editar, centrada a lo ancho; los
//! botones van centrados en la esquina superior izquierda de cada pieza
//! visible al editar. El menos es un rectángulo plano, nunca texto. Misma
//! geometría para dibujar y para pulsar.

use dokka_model::ControlItem;
use dokka_scene::{Rect, Text, TextAlign};
use dokka_theme::{Theme, control_center as measures};

use super::layout::ControlLayout;
use super::paint::{Pieces, flat, tile};

/// Rectángulo de la burbuja bajo la rejilla, centrada a lo ancho.
pub(crate) fn bubble_rect(bounds: Rect, grid_height: f32) -> Rect {
    Rect {
        x: bounds.x + (bounds.width - measures::EDIT_BUBBLE_WIDTH) / 2.0,
        y: bounds.y + grid_height + measures::EDIT_GAP,
        width: measures::EDIT_BUBBLE_WIDTH,
        height: measures::EDIT_BUBBLE_HEIGHT,
    }
}

/// Alto total del panel: rejilla sola al editar; rejilla más hueco y burbuja
/// sin editar (la burbuja cabe en su aire inferior como las piezas).
pub(crate) fn total_height(grid_height: f32, editing: bool) -> f32 {
    if editing {
        grid_height
    } else {
        grid_height + measures::EDIT_GAP + measures::EDIT_BUBBLE_HEIGHT
    }
}

/// Cuadrado del botón "−" de esta pieza, centrado en su esquina superior
/// izquierda.
fn remove_square(slot: Rect) -> Rect {
    let half = measures::REMOVE_DIAMETER / 2.0;
    Rect {
        x: slot.x - half,
        y: slot.y - half,
        width: measures::REMOVE_DIAMETER,
        height: measures::REMOVE_DIAMETER,
    }
}

/// Botones "−" de cada control visible, en el orden de fábrica.
pub(crate) fn remove_badges(layout: &ControlLayout) -> Vec<(ControlItem, Rect)> {
    let mut badges = Vec::new();
    for item in ControlItem::FACTORY_ORDER {
        if let Some(slot) = layout.rect(item) {
            badges.push((item, remove_square(slot)));
        }
    }
    badges
}

/// Control cuyo "−" cubre el punto, si hay alguno al editar.
pub(crate) fn remove_at(layout: &ControlLayout, x: f32, y: f32) -> Option<ControlItem> {
    for (item, badge) in remove_badges(layout) {
        if contains(badge, x, y) {
            return Some(item);
        }
    }
    None
}

/// `true` si el punto cae en la burbuja.
pub(crate) fn bubble_hit(bounds: Rect, grid_height: f32, x: f32, y: f32) -> bool {
    contains(bubble_rect(bounds, grid_height), x, y)
}

/// Suma la burbuja (sin editar) o los botones "−" (al editar).
pub(crate) fn add_edit_chrome(
    pieces: &mut Pieces,
    theme: &Theme,
    layout: &ControlLayout,
    bounds: Rect,
    grid_height: f32,
    editing: bool,
) {
    if editing {
        for (_, badge) in remove_badges(layout) {
            pieces.tiles.push(tile(theme, badge, badge.width / 2.0));
            pieces.flats.push(flat(
                remove_line(badge),
                measures::REMOVE_LINE_HEIGHT / 2.0,
                theme.tray.circle_icon,
            ));
        }
        return;
    }
    let bubble = bubble_rect(bounds, grid_height);
    pieces.tiles.push(tile(theme, bubble, bubble.height / 2.0));
    pieces.texts.push(Text {
        x: bubble.x + bubble.width / 2.0,
        center_y: bubble.y + bubble.height / 2.0,
        content: String::from("Editar controles"),
        size: measures::EDIT_TEXT_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Center,
    });
}

/// Línea horizontal del menos, centrada en su círculo.
fn remove_line(badge: Rect) -> Rect {
    Rect {
        x: badge.x + (badge.width - measures::REMOVE_LINE_WIDTH) / 2.0,
        y: badge.y + (badge.height - measures::REMOVE_LINE_HEIGHT) / 2.0,
        width: measures::REMOVE_LINE_WIDTH,
        height: measures::REMOVE_LINE_HEIGHT,
    }
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
