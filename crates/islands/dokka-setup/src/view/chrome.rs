//! Piezas comunes: botón redondo, píldora, enlace, logo y textos.

use std::sync::Arc;

use dokka_scene::{Dot, Drawing, Icon, IconImage, Panel, Rect, Shape, Text, TextAlign};
use dokka_theme::{Theme, installer as measures};

use super::super::{Control, Setup};

/// Dibujo de la X de cerrar, centrada en el lienzo de 100.
pub fn close_drawing(color: dokka_scene::Color) -> Arc<Drawing> {
    Arc::new(Drawing {
        shapes: vec![
            Shape::Line {
                from: (30.0, 30.0),
                to: (70.0, 70.0),
                width: 8.0,
                color,
            },
            Shape::Line {
                from: (70.0, 30.0),
                to: (30.0, 70.0),
                width: 8.0,
                color,
            },
        ],
    })
}

/// Dibujo del chevrón de volver (<), centrado en el lienzo.
pub fn back_drawing(color: dokka_scene::Color) -> Arc<Drawing> {
    Arc::new(Drawing {
        shapes: vec![
            Shape::Line {
                from: (62.0, 26.0),
                to: (38.0, 50.0),
                width: 8.0,
                color,
            },
            Shape::Line {
                from: (38.0, 50.0),
                to: (62.0, 74.0),
                width: 8.0,
                color,
            },
        ],
    })
}

/// Botón redondo translúcido de cerrar (X) o volver (chevrón).
pub fn round_button(theme: &Theme, setup: &Setup, back: bool) -> (Panel, Icon) {
    let (x, y, w, h) = measures::round_rect();
    let bounds = Rect {
        x,
        y,
        width: w,
        height: h,
    };
    let hovered = setup.hovered() == Some(Control::Round);
    let pressed = setup.pressed() == Some(Control::Round);
    let tint = if pressed {
        theme.installer.card_border
    } else if hovered {
        theme.installer.card
    } else {
        theme.installer.close
    };
    let panel = Panel {
        bounds,
        corner_radius: measures::ROUND_RADIUS,
        border_width: 1.0,
        tint,
        border: theme.installer.close_border,
    };
    let side = measures::ROUND_GLYPH_SIDE;
    let drawing = if back {
        back_drawing(theme.installer.text)
    } else {
        close_drawing(theme.installer.text)
    };
    let icon = Icon {
        x: x + (w - side) / 2.0,
        y: y + (h - side) / 2.0,
        side,
        image: Some(IconImage::Drawing(drawing)),
        placeholder: theme.tray.clear,
        placeholder_radius: 0.0,
    };
    (panel, icon)
}

/// Píldora principal (`accent` o `danger`), con respuesta al pasar y pulsar.
pub fn primary_button(theme: &Theme, setup: &Setup, label: &str, danger: bool) -> (Panel, Text) {
    let (x, y, w, h) = measures::button_rect();
    let hovered = setup.hovered() == Some(Control::Primary);
    let pressed = setup.pressed() == Some(Control::Primary);
    let tint = if danger {
        if pressed {
            theme.installer.danger_pressed
        } else if hovered {
            theme.installer.danger_hover
        } else {
            theme.installer.danger
        }
    } else if pressed {
        theme.installer.accent_pressed
    } else if hovered {
        theme.installer.accent_hover
    } else {
        theme.installer.accent
    };
    let panel = Panel {
        bounds: Rect {
            x,
            y,
            width: w,
            height: h,
        },
        corner_radius: measures::BUTTON_RADIUS,
        border_width: 0.0,
        tint,
        border: tint,
    };
    let text = Text {
        x: measures::center_x(),
        center_y: y + h / 2.0,
        content: label.to_owned(),
        size: measures::BUTTON_TEXT_SIZE,
        is_bold: true,
        color: theme.menu.highlighted_text,
        align: TextAlign::Center,
    };
    (panel, text)
}

/// Enlace discreto bajo el botón (`Close` o `ShowOptions`).
pub fn link_text(theme: &Theme, label: &str) -> Text {
    let (_, top, _, height) = measures::button_rect();
    Text {
        x: measures::center_x(),
        center_y: top + height + measures::LINK_GAP + measures::LINK_TEXT_SIZE / 2.0,
        content: label.to_owned(),
        size: measures::LINK_TEXT_SIZE,
        is_bold: false,
        color: theme.installer.secondary,
        align: TextAlign::Center,
    }
}

/// Hueco del logo con el texto "Tu logo".
pub fn logo_placeholder(theme: &Theme, top: f32, side: f32) -> (Panel, Text) {
    let x = measures::center_x() - side / 2.0;
    let panel = Panel {
        bounds: Rect {
            x,
            y: top,
            width: side,
            height: side,
        },
        corner_radius: measures::LOGO_RADIUS.min(side / 2.0),
        border_width: 1.0,
        tint: theme.installer.card,
        border: theme.installer.card_border,
    };
    let text = Text {
        x: measures::center_x(),
        center_y: top + side / 2.0,
        content: "Tu logo".to_owned(),
        size: measures::LOGO_TEXT_SIZE,
        is_bold: false,
        color: theme.installer.secondary,
        align: TextAlign::Center,
    };
    (panel, text)
}

/// Título grande centrado y subtítulo secundario.
pub fn title_texts(theme: &Theme, title: &str, subtitle: &str, top: f32) -> [Text; 2] {
    [
        Text {
            x: measures::center_x(),
            center_y: top + measures::TITLE_SIZE / 2.0,
            content: title.to_owned(),
            size: measures::TITLE_SIZE,
            is_bold: true,
            color: theme.installer.text,
            align: TextAlign::Center,
        },
        Text {
            x: measures::center_x(),
            center_y: top
                + measures::TITLE_SIZE
                + measures::TITLE_GAP
                + measures::SUBTITLE_SIZE / 2.0,
            content: subtitle.to_owned(),
            size: measures::SUBTITLE_SIZE,
            is_bold: false,
            color: theme.installer.secondary,
            align: TextAlign::Center,
        },
    ]
}

/// Círculo de estado con su dibujo (check verde o aviso naranja).
pub fn badge(theme: &Theme, color: dokka_scene::Color, drawing: Arc<Drawing>) -> (Dot, Icon) {
    let x = measures::center_x() - measures::BADGE_SIDE / 2.0;
    let dot = Dot {
        x,
        y: measures::BADGE_TOP,
        diameter: measures::BADGE_SIDE,
        color,
    };
    let side = measures::BADGE_GLYPH_SIDE;
    let icon = Icon {
        x: x + (measures::BADGE_SIDE - side) / 2.0,
        y: measures::BADGE_TOP + (measures::BADGE_SIDE - side) / 2.0,
        side,
        image: Some(IconImage::Drawing(drawing)),
        placeholder: theme.tray.clear,
        placeholder_radius: 0.0,
    };
    (dot, icon)
}

/// Dibujo del check, en el lienzo de 100.
pub fn check_drawing(color: dokka_scene::Color) -> Arc<Drawing> {
    Arc::new(Drawing {
        shapes: vec![
            Shape::Line {
                from: (28.0, 54.0),
                to: (44.0, 70.0),
                width: 9.0,
                color,
            },
            Shape::Line {
                from: (44.0, 70.0),
                to: (74.0, 32.0),
                width: 9.0,
                color,
            },
        ],
    })
}
