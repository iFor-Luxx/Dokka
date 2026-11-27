use dokka_components::nav_pill::{
    nav_pill_divider, nav_pill_highlight, nav_pill_icons, nav_pill_panel,
};
use dokka_components::search_field::{
    search_magnifier_at, search_panel_at, settings_search_cursor, settings_search_text,
};
use dokka_components::settings_list::{
    filtered_badges, filtered_empty, filtered_hover, filtered_icons, filtered_labels,
    filtered_selection, search_bounds, sidebar_panel_for,
};
use dokka_components::window_controls::traffic_icons;
use dokka_scene::{Divider, Scene, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::section_names;
use super::content::content_scene;
use crate::Settings;

/// Escena fija de la ventana: fondo, barra lateral, filas visibles, cabecera
/// y contenido. Cada capa visual se crea arriba, en orden de pintado: fondo,
/// paneles, planos, resaltado, líneas, iconos, puntos y textos. Cambiar de
/// sección, filtrar o mover el cursor no cambia la forma salvo el contenido:
/// las tarjetas traen planos, líneas, puntos y textos de más o de menos según
/// la sección y `SceneLayers` reconstruye entonces (si el estilo cambia o
/// crece la cantidad); los textos e iconos sobrantes se ocultan sin
/// reconstruir y la reserva cubre las vueltas.
/// La barra es un panel flotante de cristal; el buscador, la selección, los
/// hovers y el contenido van como planos sobre ella para que el cristal no
/// los oscurezca. Las tarjetas del contenido son planos claros sobre el fondo
/// de la ventana: como cristal muestrearían el fondo de pantalla y saldrían
/// azul oscuro. El resaltado de la píldora va como plano, igual que en el
/// calendario: como cristal muestrearía el fondo y taparía su sombra suave.
/// La fila arrastrada de la barra va como plano bajo sus iconos y textos:
/// los planos se pintan antes y el texto nunca queda tapado. Cada semáforo
/// es un solo icono con su círculo y, con el cursor encima, su símbolo; sin
/// foco van en gris. El cursor de la casilla siempre está, de ancho cero
/// cuando no hay foco. Con el zoom verde la ventana crece y el contenido se
/// ensancha: la barra lateral conserva su ancho y los controles, pegados al
/// margen derecho, se adaptan solos porque cada medida sale del ancho actual.
pub fn settings_scene(theme: &Theme, settings: &mut Settings) -> Scene {
    let order = settings.filtered_order();
    let selected = settings.selected();
    let pill = settings.pill_props();
    let search = search_bounds();
    let search_drawing = settings.search_drawing(theme);
    let grip_drawing = settings.grip_drawing(theme);
    let nav = settings.nav_drawings(theme);
    let traffic = settings.traffic_drawings(theme);
    let focused = settings.window_focused();
    let hovered = settings.traffic_hovered();
    let window_width = settings.window_width();
    let window_height = settings.window_height();
    let header_right = settings.header_right();
    let mut badge_drawings = Vec::with_capacity(order.len());
    let mut glyphs = Vec::with_capacity(order.len());
    for index in &order {
        badge_drawings.push(settings.badge_drawing(theme, *index));
        glyphs.push(settings.row_drawing(theme, *index));
    }
    let content = content_scene(theme, settings, &grip_drawing);
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
    icons.extend(content.icons);
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
    texts.extend(content.texts);
    let mut flats = vec![
        search_panel_at(
            theme.settings.search_background,
            theme.settings.search_border,
            search,
            measures::SEARCH_RADIUS,
            measures::SEARCH_BORDER_WIDTH,
        ),
        filtered_selection(theme, selected, &order),
        filtered_hover(theme, selected, settings.hovered(), &order),
        nav_pill_highlight(theme, &pill),
    ];
    flats.extend(content.flats);
    let mut dividers: Vec<Divider> = vec![
        Divider {
            bounds: dokka_scene::Rect {
                x: measures::HEADER_DIVIDER_LEFT,
                y: measures::HEADER_DIVIDER_TOP,
                width: header_right - measures::HEADER_DIVIDER_LEFT,
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
    ];
    dividers.extend(content.dividers);
    Scene {
        panel: dokka_scene::Panel {
            bounds: dokka_scene::Rect {
                x: 0.0,
                y: 0.0,
                width: window_width,
                height: window_height,
            },
            corner_radius: measures::CORNER_RADIUS,
            border_width: measures::BORDER_WIDTH,
            tint: theme.settings.window,
            border: theme.settings.window_border,
        },
        panels: vec![
            sidebar_panel_for(theme, window_height),
            // Cada capa se inserta arriba: la píldora queda sobre el fondo.
            nav_pill_panel(theme, &pill),
        ],
        // Planos sobre la ventana: el cristal muestrearía el fondo de pantalla
        // y taparía su tinte. Las tarjetas, los deslizadores (barra, relleno
        // desde la fábrica, marca y tirador en cápsula), el segmentado (fondo
        // y pastilla), los interruptores (pista y tirador), el agarre
        // reordenable y su resaltado y el resaltado de la píldora van aquí.
        // Los tiradores son planos porque los puntos son círculos. El contenido
        // cambia de tamaño entre secciones: `SceneLayers` reconstruye si crece
        // o cambia el estilo; al arrastrar la barra solo se mueven, sin
        // reconstruir: la tarjeta, el resaltado, los interruptores, las líneas,
        // los agarres y los textos son siempre los mismos.
        flat_panels: flats,
        highlight: None,
        dividers,
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
