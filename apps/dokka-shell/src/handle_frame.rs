//! Fotograma de animación: avanza el dock, el genio y los paneles.

use dokka_contracts::WindowControl;
use std::error::Error;

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Avanza un fotograma y restaura las ventanas que el genio ya llevó a su sitio.
pub(crate) fn handle_frame(
    session: &mut DockSession,
    surface: &mut Surface,
    seconds: f32,
) -> Result<(), Box<dyn Error>> {
    session.dock.advance(seconds);
    // El genio avanza en su propio reloj y pide restaurar al llegar.
    for window in session.genie.on_frame(std::time::Instant::now()) {
        if session.is_settings_window(window, surface) {
            if let Err(error) = crate::settings::open_or_focus_settings(session, surface) {
                eprintln!("Dokka no pudo restaurar Preferencias: {error}");
            }
        } else if let Err(error) = session.control.activate(window) {
            eprintln!("Dokka no pudo restaurar la ventana: {error}");
        }
    }
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
