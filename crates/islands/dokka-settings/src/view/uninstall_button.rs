//! Botón "Desinstalar Dokka…" bajo el aviso de actualización.
//!
//! Una fila propia separada del resto, con el texto en rojo centrado. Sin
//! instalar se atenúa y debajo sale su nota. El resaltado solo existe al
//! pasar el cursor con el botón disponible.

use dokka_scene::{Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{uninstall_note_top, uninstall_top};
use super::super::painted::PaintedControl;
use super::super::state::Settings;
use super::about_cards::section_header;
use super::content::ContentScene;
use super::general_rows::{hidden_icon, hidden_panel};

/// Piezas del botón y su nota; `flats`, `icons` y `texts` para la escena.
pub(crate) fn uninstall_pieces(
    theme: &Theme,
    settings: &Settings,
    width: f32,
    window_height: f32,
    scroll: f32,
) -> ContentScene {
    let available = settings.can_uninstall();
    let top = uninstall_top(scroll);
    let hidden =
        top + measures::UNINSTALL_ROW_HEIGHT < measures::CONTENT_TOP || top > window_height;
    let panel = if hidden {
        hidden_panel(theme)
    } else {
        Panel {
            bounds: Rect {
                x: measures::CONTENT_LEFT,
                y: top,
                width,
                height: measures::UNINSTALL_ROW_HEIGHT,
            },
            corner_radius: measures::CARD_RADIUS,
            border_width: measures::CARD_BORDER_WIDTH,
            tint: theme.settings.card,
            border: theme.settings.card_border,
        }
    };
    let middle = top + measures::UNINSTALL_ROW_HEIGHT / 2.0;
    let mut texts = Vec::new();
    if !hidden && middle >= measures::CONTENT_TOP && middle <= window_height {
        texts.push(Text {
            x: measures::CONTENT_LEFT + width / 2.0,
            center_y: middle,
            content: measures::UNINSTALL_LABEL.to_owned(),
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: button_color(theme, available),
            align: TextAlign::Center,
        });
    }
    if !available {
        texts.extend(section_header(
            theme,
            uninstall_note_top(scroll),
            measures::UNINSTALL_DISABLED_NOTE,
            width,
            window_height,
        ));
    }
    let flats = vec![panel, uninstall_hover(theme, settings, width)];
    let icons = vec![hidden_icon(theme), hidden_icon(theme)];
    ContentScene {
        flats,
        dividers: Vec::new(),
        icons,
        texts,
    }
}

/// Rojo disponible, atenuado si no está instalado.
fn button_color(theme: &Theme, available: bool) -> dokka_scene::Color {
    if available {
        theme.settings.danger
    } else {
        theme.settings.card_text
    }
}

/// Resaltado de la fila bajo el cursor, solo si responde.
fn uninstall_hover(theme: &Theme, settings: &Settings, width: f32) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::REORDER_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    };
    if !settings.can_uninstall() {
        return hidden;
    }
    if settings.values().control != Some(PaintedControl::Uninstall) {
        return hidden;
    }
    Panel {
        bounds: Rect {
            x: measures::CONTENT_LEFT,
            y: uninstall_top(settings.scroll_offset()),
            width,
            height: measures::UNINSTALL_ROW_HEIGHT,
        },
        corner_radius: measures::REORDER_HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    }
}
