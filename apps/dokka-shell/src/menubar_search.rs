//! Buscador de la barra: filtra el Escritorio y las fijadas como el Launchpad.
//!
//! La isla guarda lo escrito y lo resaltado; aquí se mezclan el catálogo
//! con las fijadas (`merge_apps`), se filtra con `filter_search_order` y
//! se resuelven los iconos con `IconCache`. Enter o clic abre la app.

use std::error::Error;

use dokka_components::search_panel::{SearchPanelItem, filter_search_order};
use dokka_contracts::{AppCatalog, AppLauncher};
use dokka_launchpad::merge_apps;
use dokka_model::AppInfo;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Escritorio más fijadas, en el mismo orden que el Launchpad.
fn search_apps(session: &mut DockSession) -> Vec<AppInfo> {
    let catalog = session.catalog.installed_apps().unwrap_or_default();
    let pinned: Vec<AppInfo> = session
        .state
        .apps()
        .iter()
        .filter(|app| app.is_pinned)
        .map(|app| app.info.clone())
        .collect();
    merge_apps(&catalog, &pinned, &session.explorer)
}

/// Recalcula los resultados con lo escrito; solo con el buscador abierto.
pub(crate) fn refresh_search_results(session: &mut DockSession, surface: &Surface) {
    if session.menubar.tray_kind() != Some(dokka_menubar::TrayKind::Search) {
        return;
    }
    let apps = search_apps(session);
    let names: Vec<String> = apps.iter().map(|app| app.name.clone()).collect();
    let query = session.menubar.search_query().to_owned();
    let order = filter_search_order(&names, &query);
    let items: Vec<SearchPanelItem> = order
        .into_iter()
        .filter_map(|index| apps.get(index))
        .map(|app| {
            let image = session.icons.icon_for(&app.executable);
            SearchPanelItem::new(app.name.clone(), image)
        })
        .collect();
    let width = surface.menubar_width();
    session.menubar.set_search_results(items, width);
}

/// Letra escrita: filtra de nuevo con lo escrito.
pub(crate) fn type_search(session: &mut DockSession, surface: &Surface, character: char) {
    session.menubar.search_typed(character);
    refresh_search_results(session, surface);
}

/// Retroceso: borra y filtra de nuevo.
pub(crate) fn erase_search(session: &mut DockSession, surface: &Surface) {
    session.menubar.search_erased();
    refresh_search_results(session, surface);
}

/// Flechas: mueven el resaltado entre lo visible.
pub(crate) fn step_search_up(session: &mut DockSession) {
    session.menubar.search_up();
}

/// Flechas: mueven el resaltado entre lo visible.
pub(crate) fn step_search_down(session: &mut DockSession) {
    session.menubar.search_down();
}

/// Enter abre lo resaltado (o lo primero).
pub(crate) fn confirm_search(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if let Some(action) = session.menubar.search_confirmed() {
        return crate::menubar::run_menubar_action(session, surface, action);
    }
    Ok(())
}

/// Abre la app del resultado visible con este índice, por delante.
pub(crate) fn launch_search_result(
    session: &mut DockSession,
    surface: &mut Surface,
    index: usize,
) -> Result<(), Box<dyn Error>> {
    let apps = search_apps(session);
    let names: Vec<String> = apps.iter().map(|app| app.name.clone()).collect();
    let query = session.menubar.search_query().to_owned();
    let order = filter_search_order(&names, &query);
    let Some(app) = order.get(index).and_then(|slot| apps.get(*slot)) else {
        return Ok(());
    };
    let app = app.clone();
    session.menubar.close_tray();
    surface.menubar_menu_dismissed();
    session.launcher.launch(&app)?;
    Ok(())
}
