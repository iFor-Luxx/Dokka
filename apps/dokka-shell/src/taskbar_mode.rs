//! Preferencia de la barra: si se oculta sola y si está oculta ahora.
//!
//! Guarda en disco y aplica en el acto. Los menús y Preferencias comparten
//! estas ayudas para no duplicar lo que ya hace el menú del dock.

use std::error::Error;

use dokka_config::{TaskbarMode, TaskbarModeFile};
use dokka_win_control::set_taskbar_visible;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Carga si la barra se oculta sola; devuelve si hay que ocultarla ya.
pub(crate) fn load_taskbar_mode(session: &mut DockSession) -> bool {
    let mode = match TaskbarModeFile::in_user_folder().and_then(|file| file.load()) {
        Ok(mode) => mode,
        Err(error) => {
            eprintln!("No se pudo leer el modo de la barra: {error}");
            TaskbarMode::Manual
        }
    };
    let auto = mode == TaskbarMode::Automatic;
    session.set_taskbar_auto(auto);
    if auto {
        session.set_taskbar_hidden(true);
        return true;
    }
    false
}

/// Guarda la preferencia en disco.
pub(crate) fn persist_taskbar_mode(auto: bool) -> Result<(), Box<dyn Error>> {
    let mode = if auto {
        TaskbarMode::Automatic
    } else {
        TaskbarMode::Manual
    };
    TaskbarModeFile::in_user_folder()?.save(mode)?;
    Ok(())
}

/// Oculta la barra de Windows ya y recoloca el dock.
pub(crate) fn hide_taskbar_now(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    set_taskbar_visible(false);
    session.set_taskbar_hidden(true);
    session.resize(surface)?;
    Ok(())
}

/// Muestra la barra de Windows ya y recoloca el dock.
pub(crate) fn show_taskbar_now(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    set_taskbar_visible(true);
    session.set_taskbar_hidden(false);
    session.resize(surface)?;
    Ok(())
}

/// Vuelve a ocultar la barra si Windows la sacó sola (al registrarse iconos o
/// barras, o al reiniciarse el Explorador). Ocultar una ya oculta no hace nada.
pub(crate) fn keep_taskbar_hidden(session: &DockSession) {
    if session.taskbar_hidden() {
        set_taskbar_visible(false);
    }
}
