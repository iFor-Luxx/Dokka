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
use super::header_veil::{HeaderRequest, header_veil as veil_of};
use crate::Settings;

/// Escena fija de la ventana: fondo, barra lateral, filas visibles, franja
/// de la cabecera y contenido. Cada capa visual se crea arriba, en orden de
/// pintado: fondo, paneles, planos, resaltado, líneas, iconos, puntos, textos
/// y, entre el contenido y la cabecera, la franja difuminada con la píldora
/// y el título por encima y nítidos. Cambiar de
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
    let content = content_scene(theme, &mut *settings, &grip_drawing);
    let mut icons = filtered_badges(theme, &order, &badge_drawings);
    icons.extend(filtered_icons(theme, &order, &glyphs));
    icons.push(search_magnifier_at(
        theme.settings.search_icon,
        search.x + measures::SEARCH_ICON_INSET,
        search.y + (search.height - measures::SEARCH_ICON_SIDE) / 2.0,
        measures::SEARCH_ICON_SIDE,
        &search_drawing,
    ));
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
    let title = Text {
        x: measures::title_left(),
        center_y: measures::header_middle(),
        content: settings.title(),
        size: measures::TITLE_SIZE,
        is_bold: true,
        color: theme.settings.title,
        align: TextAlign::Left,
    };
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
    ];
    flats.extend(content.flats);
    let header_divider = Divider {
        bounds: dokka_scene::Rect {
            x: measures::HEADER_DIVIDER_LEFT,
            y: measures::HEADER_DIVIDER_TOP,
            width: header_right - measures::HEADER_DIVIDER_LEFT,
            height: measures::HEADER_DIVIDER_HEIGHT,
        },
        color: theme.settings.divider,
    };
    let cursors = vec![settings_search_cursor(
        theme.settings.search_text,
        search,
        settings.query(),
        settings.search_focused(),
    )];
    let mut dividers: Vec<Divider> = Vec::new();
    dividers.extend(content.dividers);
    // La cabecera va sobre la franja difuminada: la píldora, su resaltado,
    // sus líneas, sus iconos y el título quedan nítidos por encima. Sin
    // desplazar la franja es invisible y se ve como hoy.
    let veil = veil_of(
        theme,
        HeaderRequest {
            pill_panel: nav_pill_panel(theme, &pill),
            pill_flat: nav_pill_highlight(theme, &pill),
            header_divider,
            pill_divider: nav_pill_divider(theme, &pill),
            pill_icons: nav_pill_icons(theme, &nav, &pill),
            title,
            scroll: settings.scroll_offset(),
            window_width,
        },
    );
    let mut scene = Scene {
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
        panel_shadow: Some(dokka_scene::Shadow {
            color: theme.tray.shadow,
            blur: measures::SHADOW_BLUR,
            offset: (0.0, measures::SHADOW_OFFSET_Y),
        }),
        panels: vec![
            sidebar_panel_for(theme, window_height),
        ],
        // Planos sobre la ventana: el cristal muestrearía el fondo de pantalla
        // y taparía su tinte. Las tarjetas, los deslizadores (barra, relleno
        // desde la fábrica, marca y tirador en cápsula), el segmentado (fondo
        // y pastilla), los interruptores (pista y tirador) y el agarre
        // reordenable con su resaltado van aquí. El resaltado de la píldora
        // va sobre la franja, con su cristal: lo lleva la propia franja.
        // Los tiradores son planos porque los puntos son círculos. El contenido
        // cambia de tamaño entre secciones: `SceneLayers` reconstruye si crece
        // o cambia el estilo; al arrastrar la barra solo se mueven, sin
        // reconstruir: la tarjeta, el resaltado, los interruptores, las líneas,
        // los agarres y los textos son siempre los mismos.
        flat_panels: flats,
        highlight: None,
        dividers,
        cursors,
        icons,
        dots: Vec::new(),
        texts,
        thumbnails: Vec::new(),
        labels: Vec::new(),
        opacity: 1.0,
        scale: 1.0,
        blur: 0.0,
        header_veil: Some(veil),
        overlay: None,
        submenu: None,
    };
    // La ventana exterior añade el aire de la sombra por cada lado: todo el
    // contenido se desplaza por él en este único sitio (golpes y hover usan
    // coordenadas de contenido, el shell les quita el margen antes).
    let (ox, oy) = measures::content_origin();
    shift_scene(&mut scene, ox, oy);
    scene
}

/// Desplaza toda la escena por el aire de la sombra.
fn shift_scene(scene: &mut Scene, dx: f32, dy: f32) {
    scene.panel.bounds.x += dx;
    scene.panel.bounds.y += dy;
    for panel in &mut scene.panels {
        panel.bounds.x += dx;
        panel.bounds.y += dy;
    }
    for panel in &mut scene.flat_panels {
        panel.bounds.x += dx;
        panel.bounds.y += dy;
    }
    if let Some(highlight) = &mut scene.highlight {
        highlight.bounds.x += dx;
        highlight.bounds.y += dy;
    }
    for divider in &mut scene.dividers {
        divider.bounds.x += dx;
        divider.bounds.y += dy;
    }
    for cursor in &mut scene.cursors {
        cursor.text_x += dx;
        cursor.max_x += dx;
        cursor.center_y += dy;
    }
    for icon in &mut scene.icons {
        icon.x += dx;
        icon.y += dy;
    }
    for dot in &mut scene.dots {
        dot.x += dx;
        dot.y += dy;
    }
    for text in &mut scene.texts {
        text.x += dx;
        text.center_y += dy;
    }
    if let Some(veil) = &mut scene.header_veil {
        veil.bounds.x += dx;
        veil.bounds.y += dy;
        for panel in veil.panels.iter_mut().chain(veil.flats.iter_mut()) {
            panel.bounds.x += dx;
            panel.bounds.y += dy;
        }
        for divider in &mut veil.dividers {
            divider.bounds.x += dx;
            divider.bounds.y += dy;
        }
        for icon in &mut veil.icons {
            icon.x += dx;
            icon.y += dy;
        }
        for text in &mut veil.texts {
            text.x += dx;
            text.center_y += dy;
        }
    }
}
