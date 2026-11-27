use dokka_components::nav_pill::{
    nav_pill_divider, nav_pill_highlight, nav_pill_icons, nav_pill_panel,
};
use dokka_components::search_field::{
    search_magnifier_at, search_panel_at, settings_search_cursor, settings_search_text,
};
use dokka_components::settings_list::{
    filtered_badges, filtered_empty, filtered_hover, filtered_icons, filtered_labels,
    filtered_selection, search_bounds, sidebar_panel,
};
use dokka_components::window_controls::traffic_icons;
use dokka_scene::{Divider, Panel, Rect, Scene, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::section_names;
use crate::Settings;

/// Escena fija de la ventana: fondo, barra lateral, filas visibles, cabecera
/// y tarjeta. Cada capa visual se crea arriba, en orden de pintado: fondo,
/// paneles, planos, resaltado, líneas, iconos, puntos y textos. Cambiar de
/// sección, filtrar o mover el cursor no cambia la forma: las capas de sobra
/// se ocultan sin reconstruir y la reserva cubre las vueltas.
/// La barra es un panel flotante de cristal; el buscador, la selección, los
/// hovers y la tarjeta van como planos sobre ella para que el cristal no los
/// oscurezca. La tarjeta es un plano sobre el fondo claro de la ventana: como
/// cristal muestrearía el fondo de pantalla y saldría azul oscuro. El
/// resaltado de la píldora va como plano, igual que en el calendario: como
/// cristal muestrearía el fondo y taparía su sombra suave. Cada semáforo es
/// un solo icono con su círculo y, con el cursor encima, su símbolo; sin foco
/// van en gris. El cursor de la casilla siempre está, de ancho cero cuando no
/// hay foco.
pub fn settings_scene(theme: &Theme, settings: &mut Settings) -> Scene {
    let order = settings.filtered_order();
    let selected = settings.selected();
    let pill = settings.pill_props();
    let search = search_bounds();
    let search_drawing = settings.search_drawing(theme);
    let nav = settings.nav_drawings(theme);
    let traffic = settings.traffic_drawings(theme);
    let focused = settings.window_focused();
    let hovered = settings.traffic_hovered();
    let mut badge_drawings = Vec::with_capacity(order.len());
    let mut glyphs = Vec::with_capacity(order.len());
    for index in &order {
        badge_drawings.push(settings.badge_drawing(theme, *index));
        glyphs.push(settings.row_drawing(theme, *index));
    }
    let mut icons = filtered_badges(theme, &order, &badge_drawings);
    icons.extend(filtered_icons(theme, &order, &glyphs));
    icons.push(search_magnifier_at(
        theme.settings.search_icon,
        search.x + measures::SEARCH_ICON_INSET,
        search.y + (search.height - measures::SEARCH_ICON_SIDE) / 2.0,
        measures::SEARCH_ICON_SIDE,
        &search_drawing,
    ));
    icons.extend(nav_pill_icons(theme, &nav, &pill));
    icons.extend(traffic_icons(theme, &traffic, focused, hovered));
    let (card_x, card_top, card_width) = measures::card_bounds();
    let names = section_names();
    let mut texts = filtered_labels(theme, selected, &order, &names);
    texts.push(settings_search_text(
        theme.settings.search_text,
        search,
        settings.query(),
    ));
    if order.is_empty() {
        texts.push(filtered_empty(theme));
    }
    texts.push(Text {
        x: measures::title_left(),
        center_y: measures::header_middle(),
        content: names[selected].clone(),
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
        // y taparía su tinte. Siempre son cinco para no reconstruir capas: el
        // resaltado de la píldora existe siempre, de tamaño cero sin hover.
        flat_panels: vec![
            search_panel_at(
                theme.settings.search_background,
                theme.settings.search_border,
                search,
                measures::SEARCH_RADIUS,
                measures::SEARCH_BORDER_WIDTH,
            ),
            filtered_selection(theme, selected, &order),
            filtered_hover(theme, selected, settings.hovered(), &order),
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
            nav_pill_highlight(theme, &pill),
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
            settings_search_cursor(
                theme.settings.search_text,
                search,
                settings.query(),
                settings.search_focused(),
            ),
        ],
        icons,
        dots: Vec::new(),
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
