use dokka_scene::{Panel, Rect, Size, Text};
use dokka_theme::popup as theme;

/// Rectángulo de tamaño `size` centrado en `center_x` y apoyado en `bottom`,
/// sin salirse de un ancho `limit`.
pub fn place_above(size: Size, center_x: f32, bottom: f32, limit: f32) -> Rect {
    let maximum_x = (limit - size.width).max(0.0);
    Rect {
        x: (center_x - size.width / 2.0).clamp(0.0, maximum_x),
        y: bottom - size.height,
        width: size.width,
        height: size.height,
    }
}

pub fn popup_panel(bounds: Rect) -> Panel {
    Panel {
        bounds,
        corner_radius: theme::CORNER_RADIUS,
        tint: theme::TINT,
        border: theme::BORDER,
    }
}

pub fn text(content: String, center_x: f32, center_y: f32, size: f32, is_bold: bool) -> Text {
    Text {
        center_x,
        center_y,
        content,
        size,
        is_bold,
        color: theme::INK,
    }
}
