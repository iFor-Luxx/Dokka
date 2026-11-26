//! Clic sobre el Launchpad: la casilla enfoca sin cerrar, los puntos cambian
//! de página y el resto abre la app o cierra por vacío.

use dokka_components::app_grid::{AppGridItem, AppGridProps, app_at};
use dokka_components::page_dots::{PageDotsProps, dot_at};
use dokka_components::search_field::{SearchFieldProps, search_contains};
use dokka_scene::Size;

use crate::{Launchpad, LaunchpadAction};

/// Decide qué hacer con un clic en el punto.
pub(crate) fn click_launchpad(
    launchpad: &mut Launchpad,
    x: f32,
    y: f32,
    screen: Size,
) -> LaunchpadAction {
    let search = SearchFieldProps {
        screen,
        query: launchpad.query().to_owned(),
        focused: launchpad.is_focused(),
        cursor_visible: launchpad.is_cursor_visible(),
    };
    if search_contains(&search, x, y) {
        launchpad.focus_search();
        return LaunchpadAction::Stay;
    }
    let dots = PageDotsProps {
        screen,
        total: launchpad.page_count(screen),
        current: launchpad.current_page(screen),
    };
    if let Some(page) = dot_at(&dots, x, y) {
        return LaunchpadAction::ShowPage(page);
    }
    let index = app_index(launchpad, x, y, screen);
    match index.and_then(|at| launchpad.visible_apps(screen).get(at).cloned()) {
        Some(app) => LaunchpadAction::Open(app),
        None => LaunchpadAction::Close,
    }
}

pub(crate) fn app_index(launchpad: &Launchpad, x: f32, y: f32, screen: Size) -> Option<usize> {
    let apps = launchpad
        .visible_apps(screen)
        .into_iter()
        .map(|app| AppGridItem {
            name: app.name.clone(),
            image: None,
        })
        .collect();
    let props = AppGridProps {
        apps,
        highlighted: launchpad.highlighted(),
        screen,
    };
    app_at(&props, x, y)
}
