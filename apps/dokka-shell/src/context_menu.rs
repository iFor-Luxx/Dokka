//! Menús del clic derecho: sobre una app o sobre la línea separadora.

use std::error::Error;

use dokka_contracts::AppLauncher;
use dokka_win_apps::system_place;
use dokka_win_control::show_desktop;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::pinned_apps::toggle_pin;

pub(crate) fn open_context_menu(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    if session.dock.is_on_divider(x, y, session.state.apps()) {
        return open_dock_menu(session, surface);
    }
    open_app_menu(session, surface, x, y)
}

fn open_app_menu(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    let Some(app) = session.dock.pin_target(x, y, session.state.apps()) else {
        return Ok(());
    };
    let (info, is_pinned) = (app.info.clone(), app.is_pinned);
    let option = if is_pinned {
        "Quitar del dock"
    } else {
        "Fijar en el dock"
    };

    if choose(session, surface, &[option])?.is_some() {
        toggle_pin(session, info, is_pinned)?;
        session.resize(surface)?;
    }
    Ok(())
}

fn open_dock_menu(session: &mut DockSession, surface: &mut Surface) -> Result<(), Box<dyn Error>> {
    let options = [
        "Mostrar el escritorio",
        "Administrador de tareas",
        "Configuración de la barra de tareas",
        "Salir de Dokka",
    ];
    match choose(session, surface, &options)? {
        Some(0) => show_desktop(),
        Some(1) => session
            .launcher
            .launch(&system_place("Administrador de tareas", "taskmgr.exe"))?,
        Some(2) => session
            .launcher
            .launch(&system_place("Barra de tareas", "ms-settings:taskbar"))?,
        Some(3) => surface.close(),
        _ => {}
    }
    Ok(())
}

fn choose(
    session: &mut DockSession,
    surface: &Surface,
    options: &[&str],
) -> Result<Option<usize>, Box<dyn Error>> {
    let chosen = surface.show_menu(options)?;
    // El menú se queda con el mouse: la lupa no recibe el aviso de salida.
    session.dock.pointer_left();
    Ok(chosen)
}
