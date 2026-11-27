use dokka_components::calendar_panel::calendar_hovered;
use dokka_components::nav_pill::NavHalf;
use dokka_components::preview_card::preview_slot;
use dokka_scene::Rect;

use crate::{PreviewContent, PreviewRequest};

/// Mitad de la píldora bajo el cursor, si el calendario está abierto.
/// Ambas mitades siempre están disponibles.
pub(crate) fn nav_hovered_at(
    request: &PreviewRequest,
    bounds: Rect,
    x: f32,
    y: f32,
) -> Option<NavHalf> {
    if !matches!(request.content, PreviewContent::Calendar) {
        return None;
    }
    calendar_hovered(bounds, x, y)
}

/// Ventana bajo el cursor, si está sobre una mini del panel abierto.
pub(crate) fn hovered_at(request: &PreviewRequest, bounds: Rect, x: f32, y: f32) -> Option<u64> {
    let PreviewContent::Windows(windows) = &request.content else {
        return None;
    };
    windows
        .iter()
        .enumerate()
        .find(|(index, _)| contains(preview_slot(bounds, *index), x, y))
        .map(|(_, item)| item.window)
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x <= area.x + area.width && y >= area.y && y <= area.y + area.height
}
