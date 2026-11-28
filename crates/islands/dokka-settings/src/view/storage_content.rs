//! Apartado "Almacenamiento": resumen con barra y lista por categorías.
//!
//! Arriba va el disco del sistema con "X usados de Y" y una barra con un
//! tramo de color por categoría (el libre en gris); cada color coincide con
//! el cuadrito de su fila de la lista, que sirve de leyenda. La lista trae
//! icono, nombre y tamaño a la derecha ("Calculando…" mientras se calcula).
//! Otros discos, si hay, van en su tarjeta con "X libres de Y". Abajo van
//! "Vaciar papelera" y "Ajustes de almacenamiento…". Las tarjetas viven en
//! `storage_cards`, `storage_rows` y `storage_disks`; la barra en `storage_bar`.

use std::sync::Arc;

use dokka_scene::{Drawing, Panel, Rect};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{storage_header_rows, storage_layout};
use super::super::painted::PaintedControl;
use super::super::state::Settings;
use super::about_buttons::button_pieces;
use super::content::ContentScene;
use super::general_rows::{hidden_icon, hide_above};
use super::storage_bar::storage_bar_panels;
use super::storage_cards::header_pieces;
use super::storage_disks::disks_card;
use super::storage_rows::category_card;

/// Ficha completa; `badges` y `glyphs` van en orden de categorías.
pub(crate) fn storage_content(
    theme: &Theme,
    settings: &Settings,
    badges: &[Arc<Drawing>],
    glyphs: &[Arc<Drawing>],
) -> ContentScene {
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let window_height = settings.window_height();
    let breakdown = settings.storage_breakdown();
    let disks = breakdown
        .as_ref()
        .map(|known| known.other_disks.len())
        .unwrap_or(0);
    let layout = storage_layout(scroll, disks);
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut icons = Vec::new();
    let mut texts = Vec::new();

    let header_rows = storage_header_rows();
    let (panel, card_dividers, hint) = header_pieces(
        theme,
        &layout,
        &header_rows,
        width,
        window_height,
        breakdown.as_ref(),
    );
    flats.push(panel);
    dividers.extend(hide_above(card_dividers, window_height));
    texts.extend(hint);
    flats.extend(storage_bar_panels(
        theme,
        width,
        layout.header + header_rows[0] + header_rows[1],
        breakdown.as_ref(),
        window_height,
    ));

    texts.extend(super::about_cards::section_header(
        theme,
        layout.hint,
        "Cada color de la barra es una categoría de la lista.",
        width,
        window_height,
    ));

    let (panel, mut card_dividers, mut card_texts, mut card_icons) = category_card(
        theme,
        &layout,
        badges,
        glyphs,
        breakdown.as_ref(),
        width,
        window_height,
    );
    flats.push(panel);
    dividers.append(&mut card_dividers);
    texts.append(&mut card_texts);
    icons.append(&mut card_icons);

    if disks > 0 {
        texts.extend(super::about_cards::section_header(
            theme,
            layout.disks_title,
            "Otros discos",
            width,
            window_height,
        ));
        let (panel, mut card_dividers, mut card_texts) =
            disks_card(theme, &layout, breakdown.as_ref(), width, window_height);
        flats.push(panel);
        dividers.append(&mut card_dividers);
        texts.append(&mut card_texts);
    }

    let (empty_panel, empty_text) =
        button_pieces(theme, layout.empty, "Vaciar papelera", width, window_height);
    flats.push(empty_panel);
    texts.push(empty_text);
    let (settings_panel, settings_text) = button_pieces(
        theme,
        layout.settings,
        "Ajustes de almacenamiento…",
        width,
        window_height,
    );
    flats.push(settings_panel);
    texts.push(settings_text);

    flats.push(storage_hover(theme, settings, &layout, width));
    icons.push(hidden_icon(theme));
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}

/// Resaltado del botón bajo el cursor: siempre existe, de tamaño cero en reposo.
fn storage_hover(
    theme: &Theme,
    settings: &Settings,
    layout: &super::super::logic::StorageLayout,
    width: f32,
) -> Panel {
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
    let Some(PaintedControl::StorageButton(index)) = settings.values().control else {
        return hidden;
    };
    let button_top = match index {
        0 => layout.empty,
        _ => layout.settings,
    };
    let button_width = measures::ABOUT_BUTTON_WIDTH;
    Panel {
        bounds: Rect {
            x: measures::CONTENT_LEFT + (width - button_width) / 2.0,
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
