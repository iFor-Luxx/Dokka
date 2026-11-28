//! Golpe de los editores de "Fecha y hora".
//!
//! Usa los mismos rectángulos que el dibujo para no dejar piezas a medias.

use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::content::content_switch;
use super::datetime_edit::{edit_card_tops, edit_row, stepper_rects};
use crate::datetime_edit::{DateTimeEdit, visible_zones};

/// Golpe de la fila del servidor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ServerHit {
    Change,
    Apply,
    Cancel,
}

/// Fila del servidor bajo el punto.
pub(crate) fn server_hit_at(
    edit: &DateTimeEdit,
    width: f32,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<ServerHit> {
    if !edit.server_editing {
        let row = edit_row(edit, 0, 1, width, scroll);
        return in_touch(&row, x, y).then_some(ServerHit::Change);
    }
    let apply = edit_row(edit, 0, 2, width, scroll);
    let half = apply.x + apply.width / 2.0;
    if in_touch(&apply, x, y) {
        if x < half {
            return Some(ServerHit::Apply);
        }
        return Some(ServerHit::Cancel);
    }
    let row = edit_row(edit, 0, 1, width, scroll);
    if in_touch(&row, x, y) {
        return Some(ServerHit::Change);
    }
    None
}

/// Golpe del editor de fecha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ManualHit {
    Change,
    Apply,
    Cancel,
    Step(u8, i8),
}

/// Fila de fecha bajo el punto.
pub(crate) fn manual_hit_at(
    edit: &DateTimeEdit,
    auto_on: bool,
    width: f32,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<ManualHit> {
    if !edit.manual_editing {
        if auto_on {
            return None;
        }
        let row = edit_row(edit, 1, 0, width, scroll);
        return in_touch(&row, x, y).then_some(ManualHit::Change);
    }
    let fields = edit_row(edit, 1, 1, width, scroll);
    for field in 0..5 {
        let (minus, plus) = stepper_rects(&fields, field);
        if in_touch(&minus, x, y) {
            return Some(ManualHit::Step(field, -1));
        }
        if in_touch(&plus, x, y) {
            return Some(ManualHit::Step(field, 1));
        }
    }
    let buttons = edit_row(edit, 1, 2, width, scroll);
    let half = buttons.x + buttons.width / 2.0;
    if in_touch(&buttons, x, y) {
        if x < half {
            return Some(ManualHit::Apply);
        }
        return Some(ManualHit::Cancel);
    }
    None
}

/// Golpe del desplegable de zonas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ZoneHit {
    Toggle,
    Select(usize),
}

/// Fila de zona bajo el punto.
pub(crate) fn zone_hit_at(
    edit: &DateTimeEdit,
    auto_zone_on: bool,
    width: f32,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<ZoneHit> {
    if auto_zone_on {
        return None;
    }
    let row = edit_row(edit, 2, 1, width, scroll);
    if in_touch(&row, x, y) {
        return Some(ZoneHit::Toggle);
    }
    if !edit.zone_open {
        return None;
    }
    for slot in 0..edit.zones.len().min(visible_zones()) {
        let row = edit_row(edit, 2, 2 + slot, width, scroll);
        if in_touch(&row, x, y) {
            let index = edit.zone_scroll.saturating_add(slot);
            if index < edit.zones.len() {
                return Some(ZoneHit::Select(index));
            }
        }
    }
    None
}

/// Interruptor de automáticas bajo el punto, si cae en él.
pub(crate) fn auto_time_at(edit: &DateTimeEdit, width: f32, scroll: f32, x: f32, y: f32) -> bool {
    let tops = edit_card_tops(edit, scroll);
    let rows = if edit.server_editing { 3 } else { 2 };
    let heights = vec![measures::CONTENT_ROW_HEIGHT; rows];
    in_touch(
        &content_switch(tops[0], &heights, 0, width),
        x,
        y,
    )
}

/// Interruptor de zona automática bajo el punto, si cae en él.
pub(crate) fn auto_zone_at(edit: &DateTimeEdit, width: f32, scroll: f32, x: f32, y: f32) -> bool {
    let tops = edit_card_tops(edit, scroll);
    let rows = if edit.zone_open {
        2 + edit.zones.len().min(visible_zones())
    } else {
        2
    };
    let heights = vec![measures::CONTENT_ROW_HEIGHT; rows];
    in_touch(
        &content_switch(tops[2], &heights, 0, width),
        x,
        y,
    )
}

/// Interruptor de formato de 24 horas bajo el punto, con su altura real.
pub(crate) fn hour24_at(edit: &DateTimeEdit, width: f32, scroll: f32, x: f32, y: f32) -> bool {
    let tops = edit_card_tops(edit, scroll);
    let rows = if edit.manual_editing { 4 } else { 2 };
    let heights = vec![measures::CONTENT_ROW_HEIGHT; rows];
    let index = if edit.manual_editing { 3 } else { 1 };
    in_touch(&content_switch(tops[1], &heights, index, width), x, y)
}

fn in_touch(touch: &Rect, x: f32, y: f32) -> bool {
    (touch.x..touch.x + touch.width).contains(&x) && (touch.y..touch.y + touch.height).contains(&y)
}
