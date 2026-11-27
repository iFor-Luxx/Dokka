use dokka_scene::Rect;

use super::layout::row_rect;
use super::{BackgroundApp, BackgroundHit};
use dokka_theme::background as measures;

/// Fila que hay en el punto, si es una app visible.
pub fn background_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    apps: &[BackgroundApp],
) -> Option<BackgroundHit> {
    if x < bounds.x || x >= bounds.x + bounds.width {
        return None;
    }
    let visible = measures::visible_rows(apps.len());
    apps
        .iter()
        .take(visible)
        .enumerate()
        .find(|(index, _)| {
            let row = row_rect(bounds, *index);
            y >= row.y && y < row.y + row.height
        })
        .map(|(_, app)| BackgroundHit::Row(app.id))
}
