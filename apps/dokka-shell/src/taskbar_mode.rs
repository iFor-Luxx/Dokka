//! Lee la preferencia de la barra y la deja en la sesión.

use dokka_config::{TaskbarMode, TaskbarModeFile};

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
