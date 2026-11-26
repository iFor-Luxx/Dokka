use std::collections::HashMap;
use std::sync::Arc;

use dokka_components::app_grid::{AppGridItem, AppGridProps, app_highlight, app_icons, app_texts};
use dokka_model::AppId;
use dokka_scene::{Bitmap, Panel, Rect, Scene, Size};
use dokka_theme::Theme;

/// Escena a pantalla completa: fondo oscurecido, iconos y nombres.
pub fn launchpad_scene(
    theme: &Theme,
    screen: Size,
    apps: &[dokka_model::AppInfo],
    images: &HashMap<AppId, Arc<Bitmap>>,
    highlighted: Option<usize>,
) -> Scene {
    let items = apps
        .iter()
        .map(|app| AppGridItem {
            name: app.name.clone(),
            image: images
                .get(&app.id)
                .cloned()
                .map(dokka_scene::IconImage::Pixels),
        })
        .collect();
    let props = AppGridProps {
        apps: items,
        highlighted,
        screen,
    };
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
        icons: app_icons(theme, &props),
        dots: Vec::new(),
        dividers: Vec::new(),
        thumbnails: Vec::new(),
        labels: Vec::new(),
        texts: app_texts(theme, &props),
        highlight: app_highlight(theme, &props),
        overlay: None,
    }
}
