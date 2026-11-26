//! Acciones de cerrar y fijar desde las vistas previas.

use dokka_contracts::WindowControl;
use dokka_model::WindowId;

use crate::dock_session::DockSession;

/// Pide a la ventana que se cierre como su propia X.
/// Si ya no existe, no hace nada: el refresco cerrará el panel.
pub(crate) fn close_preview_window(session: &mut DockSession, window: u64) {
    let _ = session.control.close(WindowId::new(window));
}

/// Fija la ventana por encima o la devuelve a su orden.
/// Lee el estado de la propia ventana, así vale tras reiniciar Dokka.
pub(crate) fn toggle_pin_preview_window(session: &mut DockSession, window: u64) {
    let id = WindowId::new(window);
    let topmost = session.control.is_topmost(id).unwrap_or(false);
    let _ = session.control.set_topmost(id, !topmost);
}

/// Lee de Windows qué ventanas del panel están fijadas.
pub(crate) fn update_pinned(session: &mut DockSession) {
    let pinned = session
        .previews
        .open_windows()
        .into_iter()
        .filter(|window| {
            session
                .control
                .is_topmost(WindowId::new(*window))
                .unwrap_or(false)
        })
        .collect();
    session.previews.set_pinned(pinned);
}
