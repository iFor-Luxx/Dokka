use dokka_scene::Label;
use dokka_theme::{Theme, tooltip as measures, typography};

/// Etiqueta centrada en `center_x` sobre el borde superior `target_top`.
pub fn tooltip(theme: &Theme, text: &str, center_x: f32, target_top: f32) -> Label {
    Label {
        text: shorten(text),
        center_x,
        bottom: target_top - measures::GAP,
        font_size: typography::TOOLTIP_SIZE,
        text_color: theme.tooltip.text,
        background: theme.tooltip.background,
        padding_x: measures::PADDING_X,
        padding_y: measures::PADDING_Y,
        corner_radius: measures::CORNER_RADIUS,
        pointer_height: measures::POINTER_HEIGHT,
    }
}

fn shorten(text: &str) -> String {
    if text.chars().count() <= measures::MAXIMUM_CHARACTERS {
        return text.to_owned();
    }
    let kept: String = text
        .chars()
        .take(measures::MAXIMUM_CHARACTERS - 1)
        .collect();
    format!("{}…", kept.trim_end())
}
