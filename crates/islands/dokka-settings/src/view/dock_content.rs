//! Tarjeta del Dock: tamaño y lupa, posición y hábitos.
//!
//! Tres grupos estilo Tahoe: deslizadores con relleno desde la fábrica,
//! segmentado de posición e interruptores. Usa la misma geometría que el
//! golpe y el arrastre.

use dokka_components::segmented_control::{SegmentedProps, segmented_panels, segmented_texts};
use dokka_theme::Theme;

use super::super::logic::{
    SECTION_DOCK, card_heights, card_tops, content_segmented, icon_scale_neutral, icon_scale_to,
    magnification_neutral, magnification_to, position_options, position_to_index, section_cards,
};
use super::super::pointer::display_preferences;
use super::super::state::Settings;
use super::content::{ContentScene, coming_soon, group_pieces, slider_pieces, switch_pieces};

/// Contenido del Dock en tres tarjetas compactas que caben en 720 de alto.
pub(crate) fn dock_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let shown = display_preferences(settings.values());
    let tops = card_tops(SECTION_DOCK);
    let cards = section_cards(SECTION_DOCK);
    let (Some(first_top), Some(first_labels)) = (tops.first().copied(), cards.first()) else {
        return coming_soon(theme);
    };
    let first_heights = card_heights(SECTION_DOCK, 0);
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut texts = Vec::new();
    let strength_dim = !shown.dock.magnification;
    let dim = [false, false, strength_dim];
    let (panel, mut card_dividers, mut card_texts) =
        group_pieces(theme, first_top, first_labels, &first_heights, &dim);
    flats.push(panel);
    dividers.append(&mut card_dividers);
    texts.append(&mut card_texts);
    let (mut panels, mut captions) = slider_pieces(
        theme,
        first_top,
        &first_heights,
        0,
        icon_scale_to(shown.dock.icon_scale),
        icon_scale_neutral(),
        true,
    );
    flats.append(&mut panels);
    texts.append(&mut captions);
    flats.append(&mut switch_pieces(
        theme,
        first_top,
        &first_heights,
        1,
        shown.dock.magnification,
    ));
    let (mut panels, mut captions) = slider_pieces(
        theme,
        first_top,
        &first_heights,
        2,
        magnification_to(shown.dock.magnification_strength),
        magnification_neutral(),
        !strength_dim,
    );
    flats.append(&mut panels);
    texts.append(&mut captions);
    let (Some(second_top), Some(second_labels)) = (tops.get(1).copied(), cards.get(1)) else {
        return ContentScene {
            flats,
            dividers,
            icons: Vec::new(),
            texts,
        };
    };
    let second_heights = card_heights(SECTION_DOCK, 1);
    let (panel, mut card_dividers, mut card_texts) = group_pieces(
        theme,
        second_top,
        second_labels,
        &second_heights,
        &[false; 2],
    );
    flats.push(panel);
    dividers.append(&mut card_dividers);
    texts.append(&mut card_texts);
    let touch = content_segmented(second_top, &second_heights, 0);
    let props = SegmentedProps {
        options: position_options(),
        selected: position_to_index(shown.dock.position),
    };
    flats.extend(segmented_panels(theme, touch, &props));
    texts.extend(segmented_texts(theme, touch, &props));
    flats.append(&mut switch_pieces(
        theme,
        second_top,
        &second_heights,
        1,
        shown.dock.auto_hide,
    ));
    let (Some(third_top), Some(third_labels)) = (tops.get(2).copied(), cards.get(2)) else {
        return ContentScene {
            flats,
            dividers,
            icons: Vec::new(),
            texts,
        };
    };
    let third_heights = card_heights(SECTION_DOCK, 2);
    let (panel, mut card_dividers, mut card_texts) =
        group_pieces(theme, third_top, third_labels, &third_heights, &[false; 3]);
    flats.push(panel);
    dividers.append(&mut card_dividers);
    texts.append(&mut card_texts);
    flats.append(&mut switch_pieces(
        theme,
        third_top,
        &third_heights,
        0,
        shown.dock.animate_launch,
    ));
    flats.append(&mut switch_pieces(
        theme,
        third_top,
        &third_heights,
        1,
        shown.dock.show_indicators,
    ));
    flats.append(&mut switch_pieces(
        theme,
        third_top,
        &third_heights,
        2,
        shown.dock.previews,
    ));
    ContentScene {
        flats,
        dividers,
        icons: Vec::new(),
        texts,
    }
}
