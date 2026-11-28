//! Lista de General: cabecera centrada y dos tarjetas con filas.
//!
//! Reutiliza `settings_group` para tarjetas y líneas; las etiquetas,
//! cuadritos y el resaltado viven en `general_rows`.

use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Panel, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{
    GENERAL_HEADER_HELP_FIRST, GENERAL_HEADER_HELP_SECOND, general_badge_color, general_card_top,
    general_rows,
};
use super::super::state::Settings;
use super::content::{ContentScene, coming_soon};
use super::general_rows::{
    RowBadgeArgs, hidden_icon, hidden_panel, hide_above, hover_panel, row_badges, row_labels,
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
    let (header, tile, header_texts, header_icons) =
        header_pieces(theme, scroll, width, gear, settings);
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
    let mut flats = vec![header, tile, first_panel, second_panel];
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
    icons.extend(row_badges(RowBadgeArgs {
        theme,
        top: first_top,
        base: 0,
        badges,
        glyphs,
        chevron,
        width,
        window_height,
    }));
    icons.extend(row_badges(RowBadgeArgs {
        theme,
        top: second_top,
        base: 3,
        badges,
        glyphs,
        chevron,
        width,
        window_height,
    }));
    // Resaltado de la fila bajo el cursor: siempre existe para no reconstruir.
    flats.push(hover_panel(theme, settings, first_top, second_top, width));
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}

/// Tarjeta de cabecera: baldosa gris con el engranaje blanco, "General" y su
/// ayuda en dos líneas, todo centrado.
fn header_pieces(
    theme: &Theme,
    scroll: f32,
    width: f32,
    gear: &Arc<Drawing>,
    settings: &Settings,
) -> (Panel, Panel, Vec<Text>, Vec<Icon>) {
    use super::super::logic::general_card_top as top_of;
    let top = top_of(0, scroll);
    let window_height = settings.window_height();
    let panel =
        if top + measures::GENERAL_HEADER_HEIGHT < measures::CONTENT_TOP || top > window_height {
            hidden_panel(theme)
        } else {
            Panel {
                bounds: dokka_scene::Rect {
                    x: measures::CONTENT_LEFT,
                    y: top,
                    width,
                    height: measures::GENERAL_HEADER_HEIGHT,
                },
                corner_radius: measures::CARD_RADIUS,
                border_width: measures::CARD_BORDER_WIDTH,
                tint: theme.settings.card,
                border: theme.settings.card_border,
            }
        };
    let center = measures::CONTENT_LEFT + width / 2.0;
    let tile_top = top + measures::GENERAL_HEADER_PAD;
    let tile_color = general_badge_color(theme, 0);
    let tile_hidden = tile_top + measures::GENERAL_HEADER_BADGE_SIDE < measures::CONTENT_TOP
        || tile_top > window_height;
    let tile = if tile_hidden {
        hidden_panel(theme)
    } else {
        Panel {
            bounds: dokka_scene::Rect {
                x: center - measures::GENERAL_HEADER_BADGE_SIDE / 2.0,
                y: tile_top,
                width: measures::GENERAL_HEADER_BADGE_SIDE,
                height: measures::GENERAL_HEADER_BADGE_SIDE,
            },
            corner_radius: measures::GENERAL_HEADER_BADGE_RADIUS,
            border_width: 0.0,
            tint: tile_color,
            border: tile_color,
        }
    };
    let gear_inset = (measures::GENERAL_HEADER_BADGE_SIDE - measures::GENERAL_GEAR_SIDE) / 2.0;
    let gear_top = tile_top + gear_inset;
    let gear_icon = if tile_hidden
        || gear_top + measures::GENERAL_GEAR_SIDE < measures::CONTENT_TOP
        || gear_top > window_height
    {
        hidden_icon(theme)
    } else {
        Icon {
            x: center - measures::GENERAL_GEAR_SIDE / 2.0,
            y: gear_top,
            side: measures::GENERAL_GEAR_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(gear))),
            placeholder: theme.settings.row_text_selected,
            placeholder_radius: 0.0,
        }
    };
    let title_y = tile_top
        + measures::GENERAL_HEADER_BADGE_SIDE
        + measures::GENERAL_HEADER_GAP
        + measures::GENERAL_HEADER_TITLE_SIZE / 2.0;
    let help_first_y = title_y
        + measures::GENERAL_HEADER_TITLE_SIZE / 2.0
        + measures::GENERAL_HEADER_GAP
        + measures::GENERAL_HEADER_SUBTITLE_SIZE / 2.0;
    let help_second_y = help_first_y
        + measures::GENERAL_HEADER_SUBTITLE_SIZE / 2.0
        + measures::GENERAL_HEADER_HELP_GAP
        + measures::GENERAL_HEADER_SUBTITLE_SIZE / 2.0;
    let mut texts = Vec::new();
    if title_y >= measures::CONTENT_TOP && title_y <= window_height {
        texts.push(Text {
            x: center,
            center_y: title_y,
            content: "General".to_owned(),
            size: measures::GENERAL_HEADER_TITLE_SIZE,
            is_bold: true,
            color: theme.settings.row_text,
            align: TextAlign::Center,
        });
    }
    if help_first_y >= measures::CONTENT_TOP && help_first_y <= window_height {
        texts.push(Text {
            x: center,
            center_y: help_first_y,
            content: GENERAL_HEADER_HELP_FIRST.to_owned(),
            size: measures::GENERAL_HEADER_SUBTITLE_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Center,
        });
    }
    if help_second_y >= measures::CONTENT_TOP && help_second_y <= window_height {
        texts.push(Text {
            x: center,
            center_y: help_second_y,
            content: GENERAL_HEADER_HELP_SECOND.to_owned(),
            size: measures::GENERAL_HEADER_SUBTITLE_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Center,
        });
    }
    (panel, tile, texts, vec![gear_icon])
}
