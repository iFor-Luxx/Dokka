//! Lista de General: cabecera centrada y dos tarjetas con filas.
//!
//! Reutiliza `settings_group` para tarjetas y líneas; las etiquetas,
//! cuadritos y el resaltado viven en `general_rows`.

use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Panel, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{
    GENERAL_GEAR_SIDE, GENERAL_HEADER_GAP, GENERAL_HEADER_HEIGHT, GENERAL_HEADER_HELP,
    GENERAL_HEADER_SUBTITLE, GENERAL_HEADER_TITLE, general_card_top, general_rows,
};
use super::super::state::Settings;
use super::content::{ContentScene, coming_soon};
use super::general_rows::{
    hidden_icon, hidden_panel, hide_above, hover_panel, row_badges, row_labels,
};

/// Cabecera + dos tarjetas de General; `badges`, `glyphs` y `chevron` ya
/// vienen en orden visible, como el lateral.
pub(crate) fn general_content(
    theme: &Theme,
    settings: &Settings,
    badges: &[Arc<Drawing>],
    glyphs: &[Arc<Drawing>],
    chevron: &Arc<Drawing>,
    gear: &Arc<Drawing>,
) -> ContentScene {
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let window_height = settings.window_height();
    if settings.general() != super::super::general::GeneralPage::Root {
        return coming_soon(theme, width);
    }
    let rows = general_rows();
    let first_top = general_card_top(1, scroll);
    let second_top = general_card_top(2, scroll);
    let heights_first = vec![measures::CONTENT_ROW_HEIGHT; 3];
    let heights_second = vec![measures::CONTENT_ROW_HEIGHT; 2];
    let (header, header_texts, header_icons) = header_pieces(theme, scroll, width, gear, settings);
    // Tarjetas y líneas con el componente; las etiquetas van aparte con el
    // aire del cuadrito, como en la barra lateral.
    let (first_panel, first_dividers, _) = super::content::group_pieces(
        theme,
        first_top,
        &["", "", ""],
        &heights_first,
        &[false; 3],
        width,
    );
    let (second_panel, second_dividers, _) = super::content::group_pieces(
        theme,
        second_top,
        &["", ""],
        &heights_second,
        &[false; 2],
        width,
    );
    let mut flats = vec![header, first_panel, second_panel];
    let mut dividers = Vec::new();
    dividers.extend(hide_above(first_dividers, window_height));
    dividers.extend(hide_above(second_dividers, window_height));
    let mut texts = header_texts;
    texts.extend(row_labels(
        theme,
        first_top,
        &rows[..3],
        width,
        window_height,
    ));
    texts.extend(row_labels(
        theme,
        second_top,
        &rows[3..],
        width,
        window_height,
    ));
    let mut icons = header_icons;
    icons.extend(row_badges(
        theme,
        first_top,
        0,
        badges,
        glyphs,
        chevron,
        width,
        window_height,
    ));
    icons.extend(row_badges(
        theme,
        second_top,
        3,
        badges,
        glyphs,
        chevron,
        width,
        window_height,
    ));
    // Resaltado de la fila bajo el cursor: siempre existe para no reconstruir.
    flats.push(hover_panel(theme, settings, first_top, second_top, width));
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}

/// Tarjeta de cabecera: engranaje grande, "General" y su ayuda, centrados.
fn header_pieces(
    theme: &Theme,
    scroll: f32,
    width: f32,
    gear: &Arc<Drawing>,
    settings: &Settings,
) -> (Panel, Vec<Text>, Vec<Icon>) {
    use super::super::logic::general_card_top as top_of;
    let top = top_of(0, scroll);
    let window_height = settings.window_height();
    let panel = if top + GENERAL_HEADER_HEIGHT < measures::CONTENT_TOP || top > window_height {
        hidden_panel(theme)
    } else {
        Panel {
            bounds: dokka_scene::Rect {
                x: measures::CONTENT_LEFT,
                y: top,
                width,
                height: GENERAL_HEADER_HEIGHT,
            },
            corner_radius: measures::CARD_RADIUS,
            border_width: measures::CARD_BORDER_WIDTH,
            tint: theme.settings.card,
            border: theme.settings.card_border,
        }
    };
    let center = measures::CONTENT_LEFT + width / 2.0;
    let gear_top = top + 16.0;
    let gear_icon =
        if gear_top + GENERAL_GEAR_SIDE < measures::CONTENT_TOP || gear_top > window_height {
            hidden_icon()
        } else {
            Icon {
                x: center - GENERAL_GEAR_SIDE / 2.0,
                y: gear_top,
                side: GENERAL_GEAR_SIDE,
                image: Some(dokka_scene::IconImage::Drawing(Arc::clone(gear))),
                placeholder: theme.settings.card_text,
                placeholder_radius: 0.0,
            }
        };
    let title_y = gear_top + GENERAL_GEAR_SIDE + GENERAL_HEADER_GAP + GENERAL_HEADER_TITLE / 2.0;
    let help_y =
        title_y + GENERAL_HEADER_TITLE / 2.0 + GENERAL_HEADER_GAP + GENERAL_HEADER_SUBTITLE / 2.0;
    let mut texts = Vec::new();
    if title_y >= measures::CONTENT_TOP && title_y <= window_height {
        texts.push(Text {
            x: center,
            center_y: title_y,
            content: "General".to_owned(),
            size: GENERAL_HEADER_TITLE,
            is_bold: true,
            color: theme.settings.row_text,
            align: TextAlign::Center,
        });
    }
    if help_y >= measures::CONTENT_TOP && help_y <= window_height {
        texts.push(Text {
            x: center,
            center_y: help_y,
            content: GENERAL_HEADER_HELP.to_owned(),
            size: GENERAL_HEADER_SUBTITLE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Center,
        });
    }
    (panel, texts, vec![gear_icon])
}
