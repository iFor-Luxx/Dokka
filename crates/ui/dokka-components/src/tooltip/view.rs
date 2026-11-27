use dokka_scene::{Label, LabelSide};
use dokka_theme::{Theme, tooltip as measures, typography};

/// Etiqueta centrada en `center_x` sobre el borde superior `target_top`.
pub fn tooltip(theme: &Theme, text: &str, center_x: f32, target_top: f32) -> Label {
    Label {
        text: shorten(text),
        center_x,
        center_y: 0.0,
        bottom: target_top - measures::GAP,
        side: LabelSide::Below,
        font_size: typography::TOOLTIP_SIZE,
        text_color: theme.tooltip.text,
        tint: theme.tooltip.tint,
        border: theme.tooltip.border,
        border_width: measures::BORDER_WIDTH,
        padding_x: measures::PADDING_X,
        padding_y: measures::PADDING_Y,
        pointer_height: measures::POINTER_HEIGHT,
        pointer_radius: measures::POINTER_RADIUS,
    }
}

/// Etiqueta al lado del icono, hacia dentro.
/// `target_side` es el borde del icono hacia dentro; `after` la pone a la
/// derecha (dock a la izquierda) y si no a la izquierda.
pub fn tooltip_side(
    theme: &Theme,
    text: &str,
    center_y: f32,
    target_side: f32,
    after: bool,
) -> Label {
    let (center_x, tip, side) = if after {
        (0.0, target_side + measures::GAP, LabelSide::After)
    } else {
        (0.0, target_side - measures::GAP, LabelSide::Before)
    };
    Label {
        text: shorten(text),
        center_x,
        center_y,
        bottom: tip,
        side,
        font_size: typography::TOOLTIP_SIZE,
        text_color: theme.tooltip.text,
        tint: theme.tooltip.tint,
        border: theme.tooltip.border,
        border_width: measures::BORDER_WIDTH,
        padding_x: measures::PADDING_X,
        padding_y: measures::PADDING_Y,
        pointer_height: measures::POINTER_HEIGHT,
        pointer_radius: measures::POINTER_RADIUS,
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
