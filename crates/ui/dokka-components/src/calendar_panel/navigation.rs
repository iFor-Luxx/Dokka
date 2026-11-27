use dokka_scene::Rect;

use super::{calendar_month, calendar_nav_pill, calendar_weeks};
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

/// Paso de mes si el clic cae en la píldora: nuevo desfase y mayor número
/// de semanas visto (el mes que se deja también cuenta para no encogerse).
pub fn calendar_step(
    today: (u16, u16, u16),
    offset: i32,
    weeks_seen: u16,
    bounds: Rect,
    x: f32,
    y: f32,
) -> Option<(i32, u16)> {
    let stepped = match nav_pill_at(&calendar_pill_props(bounds, None), x, y) {
        Some(NavHalf::Previous) => offset - 1,
        Some(NavHalf::Next) => offset + 1,
        None => return None,
    };
    let left = calendar_weeks(&calendar_month(today, offset));
    let weeks = calendar_weeks(&calendar_month(today, stepped));
    Some((stepped, weeks_seen.max(left).max(weeks)))
}
