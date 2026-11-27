//! Trae una ventana al frente desde cualquier sitio de Dokka.
//!
//! Miniatura, icono, vista previa y menú pasan por aquí: así una minimizada
//! sale siempre del dock con el genio, y no solo desde su miniatura.

use std::error::Error;

use dokka_contracts::WindowControl;
use dokka_model::WindowId;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

pub(crate) fn activate_window(
    session: &mut DockSession,
    surface: &mut Surface,
    window: WindowId,
) -> Result<(), Box<dyn Error>> {
    // Preferencias es propia: se muestra directamente, sin vuelo.
    if session.is_settings_window(window, surface) {
        return crate::settings::open_or_focus_settings(session, surface);
    }
    let minimized = session
        .state
        .apps()
        .iter()
        .flat_map(|app| app.windows.iter())
        .any(|known| known.id == window && known.is_minimized);
    if minimized {
        let position = session.preferences.dock.position;
        let flying = {
            let (genie, dock, state) = (&mut session.genie, &session.dock, &session.state);
            genie.on_restore_click(dock, state.apps(), position, &*surface, window)
        };
        if flying {
            // La vista previa taparía el vuelo; la ventana aparece al llegar.
            session.previews.close();
            return Ok(());
        }
    }
    session.control.activate(window)?;
    Ok(())
}
