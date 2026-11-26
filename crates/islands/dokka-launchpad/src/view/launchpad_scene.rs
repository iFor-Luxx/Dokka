use std::collections::HashMap;
use std::sync::Arc;

use dokka_components::app_grid::{AppGridItem, AppGridProps, app_highlight, app_icons, app_texts};
use dokka_components::page_dots::{PageDotsProps, page_dots};
use dokka_components::search_field::{
    SearchFieldProps, search_label, search_magnifier, search_panel,
};
use dokka_model::AppId;
use dokka_scene::{Bitmap, Panel, Rect, Scene, Size};
use dokka_theme::Theme;

use crate::Launchpad;

/// Escena a pantalla completa: fondo, casilla de búsqueda, iconos, nombres y
/// puntos de página. Cada capa visual se crea arriba, en orden de pintado:
/// fondo, paneles, resaltado, iconos, puntos y textos.
pub fn launchpad_scene(
    theme: &Theme,
    screen: Size,
    launchpad: &mut Launchpad,
    images: &HashMap<AppId, Arc<Bitmap>>,
) -> Scene {
    let visible = launchpad.visible_apps(screen);
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
    };
    let dots = PageDotsProps {
        screen,
        total: launchpad.page_count(screen),
        current: launchpad.current_page(screen),
    };
    let drawing = launchpad.magnifier(theme.launchpad.search_text);
    let mut icons = app_icons(theme, &grid);
    icons.push(search_magnifier(theme, &search, &drawing));
    let mut texts = app_texts(theme, &grid);
    texts.push(search_label(theme, &search));
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
        panels: vec![search_panel(theme, &search)],
        icons,
        dots: page_dots(theme, &dots),
        dividers: Vec::new(),
        thumbnails: Vec::new(),
        labels: Vec::new(),
        texts,
        highlight: Some(app_highlight(theme, &grid)),
        overlay: None,
        submenu: None,
    }
}
