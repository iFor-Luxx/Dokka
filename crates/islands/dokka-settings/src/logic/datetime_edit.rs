//! Geometría dinámica de "Fecha y hora" con sus editores.
//!
//! Las tarjetas crecen al abrir la casilla, el editor o el desplegable.

use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::content::content_row;
use crate::datetime_edit::{DateTimeEdit, visible_zones};

/// Filas de la tarjeta 0: automáticas y servidor (+ aplicar al editar).
fn card0_rows(edit: &DateTimeEdit) -> usize {
    if edit.server_editing { 3 } else { 2 }
}

/// Filas de la tarjeta 1: fecha viva y 24 horas (+ campos al editar).
fn card1_rows(edit: &DateTimeEdit) -> usize {
    if edit.manual_editing { 4 } else { 2 }
}

/// Filas de la tarjeta 2: zona auto y zona (+ lista al abrir).
fn card2_rows(edit: &DateTimeEdit) -> usize {
    if edit.zone_open {
        2 + edit.zones.len().min(visible_zones())
    } else {
        2
    }
}

/// Borde superior de la página con el desplazamiento aplicado.
pub(crate) fn edit_top(scroll: f32) -> f32 {
    measures::CONTENT_TOP - scroll
}

/// Borde superior de cada tarjeta con su altura real.
pub(crate) fn edit_card_tops(edit: &DateTimeEdit, scroll: f32) -> [f32; 3] {
    let top = edit_top(scroll);
    let first = top;
    let second = first + card0_rows(edit) as f32 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + admin_note_height(edit.can_change);
    let third = second
        + card1_rows(edit) as f32 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + admin_note_height(edit.can_change);
    [first, second, third]
}

/// Altura de la nota de administrador bajo cada tarjeta protegida.
fn admin_note_height(can_change: bool) -> f32 {
    if can_change {
        0.0
    } else {
        measures::ABOUT_SECTION_TITLE_HEIGHT
    }
}

/// Altura total del contenido con sus editores, sin recortar.
pub(crate) fn edit_content_height(edit: &DateTimeEdit) -> f32 {
    let rows = (card0_rows(edit) + card1_rows(edit) + card2_rows(edit)) as f32
        * measures::CONTENT_ROW_HEIGHT;
    rows + 2.0 * measures::GROUP_GAP
        + 2.0 * admin_note_height(edit.can_change)
        + measures::ABOUT_BUTTON_GAP
        + measures::ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
}

/// Lo que se puede bajar con la rueda con los editores abiertos.
pub(crate) fn edit_scroll_max(window_height: f32, edit: &DateTimeEdit) -> f32 {
    let visible = window_height - measures::CONTENT_TOP - measures::GROUP_GAP;
    (edit_content_height(edit) - visible).max(0.0)
}

/// Borde superior del botón final con los editores abiertos.
pub(crate) fn edit_button_top(edit: &DateTimeEdit, scroll: f32) -> f32 {
    let tops = edit_card_tops(edit, scroll);
    tops[2]
        + card2_rows(edit) as f32 * measures::CONTENT_ROW_HEIGHT
        + admin_note_height(edit.can_change)
        + measures::ABOUT_BUTTON_GAP
}

/// Fila completa de la tarjeta dinámica para dibujo y golpe.
pub(crate) fn edit_row(
    edit: &DateTimeEdit,
    card: usize,
    index: usize,
    width: f32,
    scroll: f32,
) -> Rect {
    let tops = edit_card_tops(edit, scroll);
    let rows = match card {
        0 => card0_rows(edit),
        1 => card1_rows(edit),
        _ => card2_rows(edit),
    };
    let heights = vec![measures::CONTENT_ROW_HEIGHT; rows];
    content_row(tops[card], &heights, index, width)
}

/// Rectángulos de menos y más de un campo (día 0, mes 1, año 2, hora 3...).
pub(crate) fn stepper_rects(row: &Rect, field: u8) -> (Rect, Rect) {
    let count = 5.0;
    let slot = measures::DATETIME_STEP_SLOT;
    let base = row.x + row.width - measures::CONTENT_PADDING - count * slot;
    let x = base + f32::from(field) * slot;
    let side = measures::DATETIME_STEP_SIDE;
    let middle = row.y + row.height / 2.0;
    (
        Rect {
            x,
            y: middle - side - measures::DATETIME_STEP_GAP,
            width: side,
            height: side,
        },
        Rect {
            x,
            y: middle + measures::DATETIME_STEP_GAP,
            width: side,
            height: side,
        },
    )
}
