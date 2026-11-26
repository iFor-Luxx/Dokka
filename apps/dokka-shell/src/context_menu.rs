//! Menús del clic derecho: sobre una app o sobre la línea separadora.

use std::error::Error;

use dokka_contracts::AppLauncher;
use dokka_dock::MenuAction;
use dokka_win_apps::system_place;
use dokka_win_control::show_desktop;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::pinned_apps::toggle_pin;

/// Abre el menú de ese punto. Otro clic derecho lo mueve al nuevo sitio.
pub(crate) fn open_context_menu(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    if session.dock.is_menu_open() {
        session.dock.close_menu();
    }
    if session.dock.open_menu(x, y, session.state.apps()) {
        session.previews.close();
        session.dock.pointer_left();
        surface.menu_opened()?;
    } else {
        surface.menu_closed();
    }
    Ok(())
}

/// Ejecuta la opción elegida y cierra el menú.
pub(crate) fn run_menu_action(
    session: &mut DockSession,
    surface: &mut Surface,
    action: MenuAction,
) -> Result<(), Box<dyn Error>> {
    // Primero se cierra y se devuelve el foco; si no, la ventana lanzada quedaría detrás.
    session.dock.close_menu();
    surface.menu_closed();
    match action {
        MenuAction::ShowDesktop => show_desktop(),
        MenuAction::TaskManager => session
            .launcher
            .launch(&system_place("Administrador de tareas", "taskmgr.exe"))?,
        MenuAction::TaskbarSettings => session
            .launcher
            .launch(&system_place("Barra de tareas", "ms-settings:taskbar"))?,
        MenuAction::Quit => surface.close(),
        MenuAction::Pin(info) => {
            toggle_pin(session, info, false)?;
            session.resize(surface)?;
        }
        MenuAction::Unpin(info) => {
            toggle_pin(session, info, true)?;
            session.resize(surface)?;
        }
    }
    Ok(())
}
