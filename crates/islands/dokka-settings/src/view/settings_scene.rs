use dokka_components::nav_pill::{NavPillProps, nav_pill_divider, nav_pill_icons, nav_pill_panel};
use dokka_components::search_field::{search_label_at, search_magnifier_at, search_panel_at};
use dokka_components::settings_list::{
    SettingsListProps, row_badges, row_hover, row_icons, row_labels, row_selection, search_bounds,
    sidebar_panel,
};
use dokka_components::window_controls::traffic_dots;
use dokka_scene::{Divider, Panel, Rect, Scene, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::section_names;
use crate::Settings;

/// Escena fija de la ventana: fondo, barra lateral, filas, cabecera y tarjeta.
/// Cada capa visual se crea arriba, en orden de pintado: fondo, paneles,
/// planos, resaltado, líneas, iconos, puntos y textos. Nada cambia de forma al elegir
/// sección: solo se mueven la selección y los textos.
/// La barra es un panel flotante de cristal; el buscador, la selección y el
/// hover son planos sobre ella para que el cristal no los oscurezca. La
/// tarjeta es un plano sobre el fondo claro de la ventana: como cristal
/// muestrearía el fondo de pantalla y saldría azul oscuro.
pub fn settings_scene(theme: &Theme, settings: &mut Settings) -> Scene {
    let props = SettingsListProps {
        selected: settings.selected(),
        hovered: settings.hovered(),
    };
    let search = search_bounds();
    let search_drawing = settings.search_drawing(theme);
    let nav = settings.nav_drawings(theme);
    // Píldora de navegación: hoy sin lados disponibles, solo adorna.
    let pill = NavPillProps {
        bounds: Rect {
            x: measures::NAV_LEFT,
            y: measures::NAV_TOP,
            width: measures::NAV_PILL_WIDTH,
            height: measures::NAV_PILL_HEIGHT,
        },
        can_previous: false,
        can_next: false,
        hovered: None,
    };
    let mut glyphs = Vec::with_capacity(measures::SECTIONS);
    for index in 0..measures::SECTIONS {
        glyphs.push(settings.row_drawing(theme, index));
    }
    let mut icons = row_badges(theme);
    icons.extend(row_icons(theme, &glyphs));
    icons.push(search_magnifier_at(
        theme.settings.search_icon,
        search.x + measures::SEARCH_ICON_INSET,
        search.y + (search.height - measures::SEARCH_ICON_SIDE) / 2.0,
        measures::SEARCH_ICON_SIDE,
        &search_drawing,
    ));
    icons.extend(nav_pill_icons(theme, &nav, &pill));
    let (card_x, card_top, card_width) = measures::card_bounds();
    let names = section_names();
    let mut texts = row_labels(theme, &props, &names);
    texts.push(search_label_at(
        theme.settings.search_text,
        search.x + measures::SEARCH_TEXT_INSET,
        search.y + search.height / 2.0,
        measures::SEARCH_PLACEHOLDER,
        measures::SEARCH_TEXT_SIZE,
    ));
    texts.push(Text {
        x: measures::title_left(),
        center_y: measures::header_middle(),
        content: names[settings.selected()].clone(),
        size: measures::TITLE_SIZE,
        is_bold: true,
        color: theme.settings.title,
        align: TextAlign::Left,
    });
    texts.push(Text {
        x: card_x + card_width / 2.0,
        center_y: card_top + measures::CARD_HEIGHT / 2.0,
        content: measures::CARD_TEXT.to_owned(),
        size: measures::CARD_TEXT_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Center,
    });
    Scene {
        panel: Panel {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: measures::WIDTH,
                height: measures::HEIGHT,
            },
            corner_radius: measures::CORNER_RADIUS,
            border_width: measures::BORDER_WIDTH,
            tint: theme.settings.window,
            border: theme.settings.window_border,
        },
        panels: vec![
            sidebar_panel(theme),
            // Cada capa se inserta arriba: la píldora queda sobre el fondo.
            nav_pill_panel(theme, &pill),
        ],
        // Planos sobre la ventana: el cristal muestrearía el fondo de pantalla
        // y taparía su tinte. Siempre son cuatro para no reconstruir capas.
        flat_panels: vec![
            search_panel_at(
                theme.settings.search_background,
                theme.settings.search_border,
                search,
                measures::SEARCH_RADIUS,
                measures::SEARCH_BORDER_WIDTH,
            ),
            row_selection(theme, &props),
            row_hover(theme, &props),
            Panel {
                bounds: Rect {
                    x: card_x,
                    y: card_top,
                    width: card_width,
                    height: measures::CARD_HEIGHT,
                },
                corner_radius: measures::CARD_RADIUS,
                border_width: measures::CARD_BORDER_WIDTH,
                tint: theme.settings.card,
                border: theme.settings.card_border,
            },
        ],
        highlight: None,
        dividers: vec![
            Divider {
                bounds: Rect {
                    x: measures::HEADER_DIVIDER_LEFT,
                    y: measures::HEADER_DIVIDER_TOP,
                    width: measures::HEADER_DIVIDER_RIGHT - measures::HEADER_DIVIDER_LEFT,
                    height: measures::HEADER_DIVIDER_HEIGHT,
                },
                color: theme.settings.divider,
            },
            nav_pill_divider(theme, &pill),
        ],
        icons,
        dots: traffic_dots(theme),
        texts,
        thumbnails: Vec::new(),
        labels: Vec::new(),
        opacity: 1.0,
        scale: 1.0,
        blur: 0.0,
        overlay: None,
        submenu: None,
    }
}
