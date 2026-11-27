//! Otras secciones con ajustes: barra superior y barra de tareas.
//!
//! Un interruptor por fila, con la misma geometría que el golpe.

use dokka_theme::Theme;

use super::super::logic::{SECTION_MENUBAR, SECTION_TASKBAR, card_heights, card_tops, section_cards};
use super::super::pointer::display_preferences;
use super::super::state::Settings;
use super::content::{ContentScene, coming_soon, group_pieces, switch_pieces};

/// Contenido de la barra superior: un interruptor por icono.
pub(crate) fn menubar_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let shown = display_preferences(settings.values());
    let tops = card_tops(SECTION_MENUBAR);
    let Some(top) = tops.into_iter().next() else {
        return coming_soon(theme);
    };
    let cards = section_cards(SECTION_MENUBAR);
    let Some(labels) = cards.into_iter().next() else {
        return coming_soon(theme);
    };
    let heights = card_heights(SECTION_MENUBAR, 0);
    let dim = [false; 7];
    let (panel, dividers, texts) = group_pieces(theme, top, &labels, &heights, &dim);
    let mut flats = vec![panel];
    let states = [
        shown.menubar.volume,
        shown.menubar.network,
        shown.menubar.music,
        shown.menubar.language,
        shown.menubar.search,
        shown.menubar.control_center,
        shown.menubar.background_apps,
    ];
    for (index, on) in states.iter().enumerate() {
        flats.append(&mut switch_pieces(theme, top, &heights, index, *on));
    }
    ContentScene {
        flats,
        dividers,
        texts,
    }
}

/// Contenido de la barra de tareas: ocultar sola la barra de Windows.
pub(crate) fn taskbar_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let tops = card_tops(SECTION_TASKBAR);
    let Some(top) = tops.into_iter().next() else {
        return coming_soon(theme);
    };
    let cards = section_cards(SECTION_TASKBAR);
    let Some(labels) = cards.into_iter().next() else {
        return coming_soon(theme);
    };
    let heights = card_heights(SECTION_TASKBAR, 0);
    let (panel, dividers, texts) = group_pieces(theme, top, &labels, &heights, &[false]);
    let mut flats = vec![panel];
    flats.append(&mut switch_pieces(
        theme,
        top,
        &heights,
        0,
        settings.taskbar_auto(),
    ));
    ContentScene {
        flats,
        dividers,
        texts,
    }
}
