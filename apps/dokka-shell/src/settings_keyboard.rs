//! Foco y teclado de la ventana de Preferencias: la isla decide.

use crate::dock_session::DockSession;

/// Foco ganado o perdido: sin foco los semáforos se ven grises.
pub(crate) fn focus_over_settings(session: &mut DockSession, focused: bool) {
    if session.settings.is_open() {
        session.settings.set_window_focused(focused);
    }
}

/// Letra escrita con la casilla enfocada: filtra las filas.
pub(crate) fn type_over_settings(session: &mut DockSession, character: char) {
    if session.settings.is_open() {
        session.settings.type_character(character);
    }
}

/// Retroceso con la casilla enfocada: borra la última letra.
pub(crate) fn erase_over_settings(session: &mut DockSession) {
    if session.settings.is_open() {
        session.settings.erase_character();
    }
}

/// Enter con búsqueda: elige la primera coincidencia.
pub(crate) fn confirm_over_settings(session: &mut DockSession) {
    if session.settings.is_open() {
        session.settings.confirm_search();
    }
}

/// Esc: limpia lo escrito y quita el foco.
pub(crate) fn escape_over_settings(session: &mut DockSession) {
    if session.settings.is_open() {
        session.settings.escape_search();
    }
}
