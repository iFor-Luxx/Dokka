//! Abre y cierra el Launchpad a pantalla completa.

use std::error::Error;

use dokka_contracts::{AppCatalog, AppLauncher};
use dokka_launchpad::{LaunchpadAction, launchpad_scene, merge_apps};
use dokka_scene::Size;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Abre o cierra al pulsar el icono del dock.
pub(crate) fn toggle_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if session.launchpad.is_open() {
        close_launchpad(session, surface);
    } else {
        open_launchpad(session, surface)?;
    }
    Ok(())
}

/// Muestra la pantalla completa con catálogo y fijadas.
pub(crate) fn open_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    session.dock.close_menu();
    session.previews.close();
    session.dock.pointer_left();
    surface.menu_closed();
    // Sin catálogo se muestran al menos las fijadas.
    let catalog = session.catalog.installed_apps().unwrap_or_default();
    let pinned: Vec<dokka_model::AppInfo> = session
        .state
        .apps()
        .iter()
        .filter(|app| app.is_pinned)
        .map(|app| app.info.clone())
        .collect();
    let apps = merge_apps(&catalog, &pinned, &session.explorer);
    session.launchpad.open(apps);
    surface.launchpad_show()?;
    surface.launchpad_opened()?;
    Ok(())
}

/// Oculta y devuelve el foco sin robar lo recién pulsado.
pub(crate) fn close_launchpad(session: &mut DockSession, surface: &mut Surface) {
    session.launchpad.close();
    // El foco se devuelve antes de ocultar: después ya no sería la ventana activa.
    surface.launchpad_closed();
    surface.launchpad_hide();
}

/// Resalta la app bajo el cursor.
pub(crate) fn move_over_launchpad(session: &mut DockSession, surface: &Surface, x: f32, y: f32) {
    let size = launchpad_size(surface);
    session.launchpad.pointer_moved(x, y, size);
}

/// Clic en la cuadrícula: abre la app, cambia de página o cierra por vacío.
pub(crate) fn click_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    let size = launchpad_size(surface);
    let action = session.launchpad.click(x, y, size);
    apply_launchpad_action(session, surface, action)
}

/// Letra escrita: filtra y vuelve a la primera página.
pub(crate) fn type_over_launchpad(session: &mut DockSession, character: char) {
    session.launchpad.type_character(character);
}

/// Retroceso: borra la última letra.
pub(crate) fn erase_over_launchpad(session: &mut DockSession) {
    session.launchpad.erase_character();
}

/// Enter abre la primera coincidencia.
pub(crate) fn confirm_over_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    let action = session.launchpad.confirm();
    apply_launchpad_action(session, surface, action)
}

/// Esc limpia lo escrito y, si ya está vacío, cierra.
pub(crate) fn escape_over_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    let action = session.launchpad.escape();
    apply_launchpad_action(session, surface, action)
}

/// Lo que pide la isla: abrir, cerrar, cambiar de página o seguir igual.
fn apply_launchpad_action(
    session: &mut DockSession,
    surface: &mut Surface,
    action: LaunchpadAction,
) -> Result<(), Box<dyn Error>> {
    match action {
        LaunchpadAction::Open(app) => {
            close_launchpad(session, surface);
            session.render(surface)?;
            session.launcher.launch(&app)?;
        }
        LaunchpadAction::Close => {
            close_launchpad(session, surface);
        }
        LaunchpadAction::ShowPage(page) => {
            session.launchpad.go_to_page(launchpad_size(surface), page);
        }
        LaunchpadAction::Stay => {}
    }
    Ok(())
}

/// La rueda cambia de página.
pub(crate) fn scroll_launchpad(session: &mut DockSession, surface: &Surface, delta: i16) {
    let size = launchpad_size(surface);
    session.launchpad.scroll(size, delta);
}

/// Dibuja la cuadrícula del Launchpad en su ventana propia.
pub(crate) fn render_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    let size = launchpad_size(surface);
    let images = session
        .launchpad
        .apps()
        .iter()
        .filter_map(|app| {
            let image = session.icons.icon_for(&app.executable)?;
            Some((app.id.clone(), image))
        })
        .collect();
    let scene = launchpad_scene(&session.theme, size, &mut session.launchpad, &images);
    session.renderer.draw_launchpad(&scene)?;
    Ok(())
}

fn launchpad_size(surface: &Surface) -> Size {
    let size = surface.launchpad_size();
    Size {
        width: size[0],
        height: size[1],
    }
}
