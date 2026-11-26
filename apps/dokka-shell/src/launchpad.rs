//! Abre y cierra el Launchpad a pantalla completa.

use std::error::Error;

use dokka_contracts::{AppCatalog, AppLauncher};
use dokka_launchpad::{LaunchpadAction, merge_apps};
use dokka_scene::Size;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Abre o cierra al pulsar el icono del dock. A mitad del cierre, reabre desde donde está.
pub(crate) fn toggle_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if session.launchpad.is_closing() {
        reopen_launchpad(session, surface)?;
    } else if session.launchpad.is_open() {
        begin_close_launchpad(session, surface);
    } else {
        open_launchpad(session, surface)?;
    }
    Ok(())
}

/// Muestra la pantalla completa con catálogo y fijadas, entrando con zoom.
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

/// Empieza el cierre animado: la app elegida se lanza y el foco se devuelve
/// ahora; la ventana se oculta cuando termina la animación.
pub(crate) fn begin_close_launchpad(session: &mut DockSession, surface: &mut Surface) {
    session.launchpad.begin_close();
    // El foco se devuelve antes de ocultar: después ya no sería la ventana activa.
    surface.launchpad_closed();
}

/// Oculta la ventana al terminar la animación de cierre.
pub(crate) fn finish_close_launchpad(session: &mut DockSession, surface: &mut Surface) {
    session.launchpad.close();
    surface.launchpad_hide();
}

/// Cancela el cierre y vuelve a abrir desde donde está.
pub(crate) fn reopen_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    session.launchpad.reopen();
    surface.launchpad_opened()?;
    Ok(())
}

/// Resalta la app bajo el cursor; indica si cambió para no redibujar.
pub(crate) fn move_over_launchpad(session: &mut DockSession, surface: &Surface, x: f32, y: f32) -> bool {
    if session.launchpad.is_closing() {
        return false;
    }
    let size = launchpad_size(surface);
    session.launchpad.pointer_moved(x, y, size)
}

/// Clic en la cuadrícula: abre la app, cambia de página o cierra por vacío.
/// Durante el cierre no se aceptan clics.
pub(crate) fn click_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    if session.launchpad.is_closing() {
        return Ok(());
    }
    let size = launchpad_size(surface);
    let action = session.launchpad.click(x, y, size);
    apply_launchpad_action(session, surface, action)
}

/// Letra escrita: filtra y vuelve a la primera página.
pub(crate) fn type_over_launchpad(session: &mut DockSession, character: char) {
    if session.launchpad.is_closing() {
        return;
    }
    session.launchpad.type_character(character);
}

/// Retroceso: borra la última letra.
pub(crate) fn erase_over_launchpad(session: &mut DockSession) {
    if session.launchpad.is_closing() {
        return;
    }
    session.launchpad.erase_character();
}

/// Enter abre la primera coincidencia.
pub(crate) fn confirm_over_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if session.launchpad.is_closing() {
        return Ok(());
    }
    let action = session.launchpad.confirm();
    apply_launchpad_action(session, surface, action)
}

/// Esc limpia lo escrito y quita el foco; con la casilla vacía y sin foco, cierra.
pub(crate) fn escape_over_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if session.launchpad.is_closing() {
        return Ok(());
    }
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
            begin_close_launchpad(session, surface);
            crate::session_render::render(session, surface)?;
            session.launcher.launch(&app)?;
        }
        LaunchpadAction::Close => {
            begin_close_launchpad(session, surface);
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
    if session.launchpad.is_closing() {
        return;
    }
    let size = launchpad_size(surface);
    session.launchpad.scroll(size, delta);
}

/// Flecha izquierda: retrocede de página o cede en la primera.
pub(crate) fn page_back_launchpad(session: &mut DockSession, surface: &Surface) {
    if session.launchpad.is_closing() {
        return;
    }
    let size = launchpad_size(surface);
    session.launchpad.step_backward(size);
}

/// Flecha derecha: avanza de página o cede en la última.
pub(crate) fn page_forward_launchpad(session: &mut DockSession, surface: &Surface) {
    if session.launchpad.is_closing() {
        return;
    }
    let size = launchpad_size(surface);
    session.launchpad.step_forward(size);
}

/// Tamaño lógico del Launchpad para la cuadrícula.
pub(crate) fn launchpad_size(surface: &Surface) -> Size {
    let size = surface.launchpad_size();
    Size {
        width: size[0],
        height: size[1],
    }
}
