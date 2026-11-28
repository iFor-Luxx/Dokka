//! Botones de "Acerca de este PC": paneles centrados y su resaltado.
//!
//! Los tres botones comparten ancho y radio concéntrico con las tarjetas.
//! El resaltado siempre existe (de tamaño cero en reposo) para no
//! reconstruir capas al mover el ratón.

use dokka_scene::{Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::painted::PaintedControl;
use super::super::state::Settings;

/// Botón centrado de ancho fijo con su texto.
pub(crate) fn button_pieces(
    theme: &Theme,
    top: f32,
    label: &str,
    width: f32,
    window_height: f32,
) -> (Panel, Text) {
    let button_width = measures::ABOUT_BUTTON_WIDTH;
    let x = measures::CONTENT_LEFT + (width - button_width) / 2.0;
    let middle = top + measures::ABOUT_BUTTON_HEIGHT / 2.0;
    let hidden = top + measures::ABOUT_BUTTON_HEIGHT < measures::CONTENT_TOP || top > window_height;
    let panel = if hidden {
        Panel {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            corner_radius: measures::ABOUT_BUTTON_RADIUS,
            border_width: measures::CARD_BORDER_WIDTH,
            tint: theme.settings.card,
            border: theme.settings.card_border,
        }
    } else {
        Panel {
            bounds: Rect {
                x,
                y: top,
                width: button_width,
                height: measures::ABOUT_BUTTON_HEIGHT,
            },
            corner_radius: measures::ABOUT_BUTTON_RADIUS,
            border_width: measures::CARD_BORDER_WIDTH,
            tint: theme.settings.card,
            border: theme.settings.card_border,
        }
    };
    let text = Text {
        x: measures::CONTENT_LEFT + width / 2.0,
        center_y: if hidden { -1000.0 } else { middle },
        content: label.to_owned(),
        size: measures::CONTENT_LABEL_SIZE,
        is_bold: false,
        color: theme.settings.row_text,
        align: TextAlign::Center,
    };
    (panel, text)
}

/// Resaltado del botón bajo el cursor.
pub(crate) fn button_hover(theme: &Theme, settings: &Settings, top: f32, width: f32) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::ABOUT_BUTTON_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    };
    let Some(PaintedControl::AboutButton(index)) = settings.values().control else {
        return hidden;
    };
    let button_width = measures::ABOUT_BUTTON_WIDTH;
    let x = measures::CONTENT_LEFT + (width - button_width) / 2.0;
    let button_top = match index {
        0 => display_button_top(top),
        1 => storage_button_top(top),
        _ => report_button_top(top),
    };
    Panel {
        bounds: Rect {
            x,
            y: button_top,
            width: button_width,
            height: measures::ABOUT_BUTTON_HEIGHT,
        },
        corner_radius: measures::ABOUT_BUTTON_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    }
}

fn display_button_top(top: f32) -> f32 {
    top + measures::ABOUT_HERO_HEIGHT
        + measures::GROUP_GAP
        + 5.0 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + measures::ABOUT_SECTION_TITLE_HEIGHT
        + 2.0 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + measures::ABOUT_SECTION_TITLE_HEIGHT
        + 2.0 * measures::CONTENT_ROW_HEIGHT
        + measures::ABOUT_BUTTON_GAP
}

fn storage_button_top(top: f32) -> f32 {
    display_button_top(top)
        + measures::ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
        + measures::ABOUT_SECTION_TITLE_HEIGHT
        + measures::CONTENT_ROW_HEIGHT
        + measures::ABOUT_BUTTON_GAP
}

fn report_button_top(top: f32) -> f32 {
    storage_button_top(top)
        + measures::ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
        + measures::ABOUT_BUTTON_HEIGHT
}
