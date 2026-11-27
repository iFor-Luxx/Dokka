//! Red de seguridad de los paneles: si el aviso de salida se pierde,
//! el cursor real manda.
//!
//! Mientras hay panel (abierto o pendiente) cada fotograma, cada segundo y
//! cada aviso del vigilante comprueban dónde está de verdad el cursor: si ya
//! no está sobre el panel ni sobre su icono, sigue el mismo camino que
//! `PointerLeft`, con su margen para volver.

use std::error::Error;

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::floating_panels::preview_request;

/// Dibuja y ajusta la vigilancia del cursor: solo suena con panel.
/// También fija si el auto-ocultado debe esperar (menú, vista o Launchpad)
/// y, al cerrar algo con el cursor fuera, confirma con el cursor real.
pub(crate) fn render_and_watch(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    let busy =
        session.dock.is_menu_open() || session.previews.has_panel() || session.launchpad.is_open();
    session.dock.set_hide_busy(busy);
    // Al cerrar con el cursor fuera, el aviso de salida pudo perderse con el
    // menú abierto: el cursor real manda, sin sondeo continuo en reposo.
    if !busy && surface.cursor_over_dock().is_none() {
        session.dock.note_cursor_inside(false);
    }
    crate::session_render::render(session, surface)?;
    surface.set_preview_watch(session.previews.has_panel());
    Ok(())
}

/// Si hay panel y el cursor ya no está sobre él ni sobre su icono,
/// lo trata como salida (mismo camino que `PointerLeft`, con su margen).
pub(crate) fn check_preview_cursor(session: &mut DockSession, surface: &Surface) {
    if !session.previews.has_panel() {
        return;
    }
    let Some((x, y)) = surface.cursor_over_dock() else {
        session.previews.pointer_left();
        return;
    };
    let request = preview_request(session, x, y);
    if !session.previews.still_inside(x, y, request.as_ref()) {
        session.previews.pointer_left();
    }
}

/// Cierra el panel si el cursor no está sobre él ni sobre su icono.
/// No lo quita de debajo del cursor (botones de música o fijar en uso).
pub(crate) fn close_if_cursor_away(session: &mut DockSession, surface: &Surface) {
    if !session.previews.has_panel() {
        return;
    }
    if let Some((x, y)) = surface.cursor_over_dock() {
        let request = preview_request(session, x, y);
        if session.previews.still_inside(x, y, request.as_ref()) {
            return;
        }
    }
    session.previews.close();
}
