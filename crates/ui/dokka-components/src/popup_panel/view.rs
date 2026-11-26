use dokka_scene::{Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, popup};

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

pub fn popup_panel(theme: &Theme, bounds: Rect) -> Panel {
    Panel {
        bounds,
        corner_radius: popup::CORNER_RADIUS,
        border_width: popup::BORDER_WIDTH,
        tint: theme.panel.background,
        border: theme.panel.border,
    }
}

pub fn text(theme: &Theme, content: String, center: (f32, f32), size: f32, is_bold: bool) -> Text {
    Text {
        x: center.0,
        center_y: center.1,
        content,
        size,
        is_bold,
        color: theme.panel.text,
        align: TextAlign::Center,
    }
}
