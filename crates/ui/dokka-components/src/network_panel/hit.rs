use dokka_scene::Rect;

use super::NetworkHit;
use super::layout::settings_row;

/// Fila de ajustes que hay en el punto, si cae en ella.
pub fn network_hit(bounds: Rect, x: f32, y: f32) -> Option<NetworkHit> {
    let row = settings_row(bounds);
    if x >= row.x && x < row.x + row.width && y >= row.y && y < row.y + row.height {
        Some(NetworkHit::Settings)
    } else {
        None
    }
}
