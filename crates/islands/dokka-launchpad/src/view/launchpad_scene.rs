use std::collections::HashMap;
use std::sync::Arc;

use dokka_components::app_grid::{
    AppGridItem, AppGridProps, app_highlight, app_icons, app_texts, empty_label,
};
use dokka_components::page_dots::{PageDotsProps, dot_halo, page_dots};
use dokka_components::search_field::{
    SearchFieldProps, search_cursor, search_label, search_magnifier, search_panel,
};
use dokka_model::AppId;
use dokka_scene::{Bitmap, Panel, Rect, Scene, Size};
use dokka_theme::Theme;

use crate::Launchpad;

/// Escena a pantalla completa: fondo, casilla de búsqueda, iconos, nombres y
/// puntos de página. Cada capa visual se crea arriba, en orden de pintado:
/// fondo, paneles, resaltado, iconos, puntos y textos.
///
/// Durante el deslizamiento la página destino ya está dibujada y se desplaza
/// hasta su sitio; la casilla y los puntos se quedan quietos. La cantidad de
/// iconos y textos no cambia a mitad del movimiento: las capas solo se
/// actualizan.
///
/// El zoom de apertura y cierre lo aplica la plataforma sobre esta escena:
/// el fondo se desvanece y el resto además escala y se desenfoca desde el centro.
pub fn launchpad_scene(
    theme: &Theme,
    screen: Size,
    launchpad: &mut Launchpad,
    images: &HashMap<AppId, Arc<Bitmap>>,
) -> Scene {
    let visible = launchpad.visible_apps(screen);
    let show_empty = visible.is_empty();
    let offset = launchpad.motion_offset(screen.width);
    let items = visible
        .iter()
        .map(|app| AppGridItem {
            name: app.name.clone(),
            image: images
                .get(&app.id)
                .cloned()
                .map(dokka_scene::IconImage::Pixels),
        })
        .collect();
    let grid = AppGridProps {
        apps: items,
        highlighted: launchpad.highlighted(),
        screen,
    };
    let search = SearchFieldProps {
        screen,
        query: launchpad.query().to_owned(),
        focused: launchpad.is_focused(),
        cursor_visible: launchpad.is_cursor_visible(),
    };
    let dots = PageDotsProps {
        screen,
        total: launchpad.page_count(screen),
        current: launchpad.current_page(screen),
    };
    let drawing = launchpad.magnifier(theme.launchpad.search_text);
    let mut icons = app_icons(theme, &grid);
    if offset != 0.0 {
        for icon in &mut icons {
            icon.x += offset;
        }
    }
    icons.push(search_magnifier(theme, &search, &drawing));
    let mut texts = app_texts(theme, &grid);
    if show_empty {
        texts.push(empty_label(theme, screen));
    }
    if offset != 0.0 {
        for text in &mut texts {
            text.x += offset;
        }
    }
    texts.push(search_label(theme, &search));
    let dividers = vec![search_cursor(theme, &search)];
    let mut highlight = app_highlight(theme, &grid);
    highlight.bounds.x += offset;
    Scene {
        panel: Panel {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: screen.width,
                height: screen.height,
            },
            corner_radius: 0.0,
            border_width: 0.0,
            tint: theme.launchpad.background,
            border: theme.launchpad.background,
        },
        panels: vec![search_panel(theme, &search), dot_halo(theme, &dots)],
        icons,
        dots: page_dots(theme, &dots),
        dividers,
        thumbnails: Vec::new(),
        labels: Vec::new(),
        texts,
        highlight: Some(highlight),
        overlay: None,
        submenu: None,
        opacity: launchpad.zoom.opacity(),
        scale: launchpad.zoom.scale(),
        blur: launchpad.zoom.blur(),
    }
}
