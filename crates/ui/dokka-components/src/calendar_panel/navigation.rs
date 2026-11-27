use dokka_scene::Rect;

use super::calendar_nav_pill;
use crate::nav_pill::{NavHalf, NavPillProps, nav_pill_at};

/// Píldora del calendario sobre `bounds`, con ambas mitades disponibles.
pub fn calendar_pill_props(bounds: Rect, hovered: Option<NavHalf>) -> NavPillProps {
    NavPillProps {
        bounds: calendar_nav_pill(bounds),
        can_previous: true,
        can_next: true,
        hovered,
    }
}

/// Mitad de la píldora bajo el cursor, si el calendario está abierto.
pub fn calendar_hovered(bounds: Rect, x: f32, y: f32) -> Option<NavHalf> {
    nav_pill_at(&calendar_pill_props(bounds, None), x, y)
}

/// Paso de mes si el clic cae en la píldora: el nuevo desfase.
pub fn calendar_step(offset: i32, bounds: Rect, x: f32, y: f32) -> Option<i32> {
    match nav_pill_at(&calendar_pill_props(bounds, None), x, y) {
        Some(NavHalf::Previous) => Some(offset - 1),
        Some(NavHalf::Next) => Some(offset + 1),
        None => None,
    }
}
