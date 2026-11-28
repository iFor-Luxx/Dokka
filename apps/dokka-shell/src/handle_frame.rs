//! Fotograma de animación: avanza el dock y los paneles.

use std::error::Error;

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Avanza un fotograma.
pub(crate) fn handle_frame(
    session: &mut DockSession,
    surface: &mut Surface,
    seconds: f32,
) -> Result<(), Box<dyn Error>> {
    session.dock.advance(seconds);
    // La salida del centro solo cuenta el tiempo hasta su cierre real:
    // la animan las capas de Composition sin repintar por fotograma.
    crate::menubar_dismiss::advance_menubar(session, surface, seconds);
    // El auto-ocultado mueve la ventana: solo mientras anima.
    if session.dock.is_hide_animating() {
        if let Err(error) = session.apply_hide(surface) {
            eprintln!("Dokka no pudo deslizar el dock: {error}");
        }
        // Al volver, el cursor ya está sobre la franja: actualiza la lupa.
        if let Some((x, y)) = surface.cursor_over_dock()
            && !session.dock.is_menu_open()
            && !session.previews.has_panel()
        {
            session.dock.pointer_moved(x, y, session.state.apps());
        }
    }
    if session.launchpad.is_open() {
        session.launchpad.advance(seconds);
        if session.launchpad.close_finished() {
            crate::launchpad::finish_close_launchpad(session, surface);
        }
    }
    if !session.launchpad.is_open() {
        // Si el aviso de salida se perdió, el cursor real manda.
        crate::preview_safety::check_preview_cursor(session, surface);
        session.previews.advance(seconds);
    }
    Ok(())
}
