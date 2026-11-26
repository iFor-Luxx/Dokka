use dokka_scene::Label;
use dokka_theme::tooltip as theme;

/// Etiqueta centrada en `center_x` sobre el borde superior `target_top`.
pub fn tooltip(text: &str, center_x: f32, target_top: f32) -> Label {
    Label {
        text: shorten(text),
        center_x,
        bottom: target_top - theme::GAP,
        font_size: theme::FONT_SIZE,
        text_color: theme::TEXT,
        background: theme::BACKGROUND,
        padding_x: theme::PADDING_X,
        padding_y: theme::PADDING_Y,
        corner_radius: theme::CORNER_RADIUS,
        pointer_height: theme::POINTER_HEIGHT,
    }
}

fn shorten(text: &str) -> String {
    if text.chars().count() <= theme::MAXIMUM_CHARACTERS {
        return text.to_owned();
    }
    let kept: String = text.chars().take(theme::MAXIMUM_CHARACTERS - 1).collect();
    format!("{}…", kept.trim_end())
}
