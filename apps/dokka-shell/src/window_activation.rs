//! Trae una ventana al frente desde cualquier sitio de Dokka.
//!
//! Miniatura, icono, vista previa y menú pasan por aquí.

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
    // Preferencias es propia: se muestra directamente.
    if session.is_settings_window(window, surface) {
        return crate::settings::open_or_focus_settings(session, surface);
    }
    session.control.activate(window)?;
    Ok(())
}
