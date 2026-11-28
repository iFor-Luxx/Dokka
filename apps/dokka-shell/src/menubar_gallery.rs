//! Teclado y rueda de la barra cuando la galería de controles está abierta:
//! el shell solo reparte; decide la isla.

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Con la galería se filtra ella; si no, el buscador como antes.
pub(crate) fn typed(session: &mut DockSession, surface: &mut Surface, character: char) {
    if session.menubar.is_gallery_open() {
        let width = surface.menubar_width();
        let screen_height = surface.menubar_monitor_height();
        session
            .menubar
            .gallery_typed(character, width, screen_height);
    } else {
        crate::menubar_search::type_search(session, surface, character);
    }
}

/// Retroceso: igual reparto que al escribir.
pub(crate) fn erased(session: &mut DockSession, surface: &mut Surface) {
    if session.menubar.is_gallery_open() {
        let width = surface.menubar_width();
        let screen_height = surface.menubar_monitor_height();
        session.menubar.gallery_erased(width, screen_height);
    } else {
        crate::menubar_search::erase_search(session, surface);
    }
}

/// Rueda sobre la galería.
pub(crate) fn scrolled(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
    delta: i16,
) {
    let width = surface.menubar_width();
    let screen_height = surface.menubar_monitor_height();
    session
        .menubar
        .gallery_scroll(x, y, delta, width, screen_height);
}
