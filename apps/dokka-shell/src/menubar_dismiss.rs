//! Cierre de la barra con salida del centro por piezas.
//!
//! El menú, el calendario y los demás paneles se cierran al instante como
//! hoy. El centro (sin galería) vuela por piezas hacia su icono con
//! animaciones de Composition: la isla guarda el destino y el tiempo, la
//! plataforma las lanza una sola vez y al llegar al tope se hace el cierre
//! real. El foco vuelve al instante, sin esperar a la salida; la ventana y
//! las zonas activas se mantienen hasta que termina para que no se recorte.
//! Mientras dura no se repinta la barra para no pisar lo animado. Si algo de
//! la animación falla, se cierra sin animación; el resto de Dokka sigue.

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Cierra el menú, el calendario o la fila. El centro sale por piezas y el
/// foco vuelve ya; lo demás se cierra al instante como hoy.
pub(crate) fn dismiss_menubar(session: &mut DockSession, surface: &mut Surface) {
    if session.menubar.is_menu_open() {
        session.menubar.close_menu();
        surface.menubar_menu_closed();
    } else if session.menubar.is_calendar_open() {
        session.menubar.close_calendar();
        surface.menubar_menu_closed();
    } else if session.menubar.is_tray_open() {
        if session.menubar.begin_control_close(surface.menubar_width()) {
            surface.menubar_menu_closed();
            start_control_dismiss(session);
            if !session.menubar.is_tray_open() {
                // La animación falló y ya se cerró al instante: recoger.
                crate::menubar::reset_menubar_window(session, surface);
            }
            return;
        }
        session.menubar.close_tray();
        surface.menubar_menu_closed();
    }
    crate::menubar::reset_menubar_window(session, surface);
}

/// Lanza las animaciones por piezas una sola vez. Si fallan, cierra al
/// instante como hoy y el resto sigue.
fn start_control_dismiss(session: &mut DockSession) {
    let Some(order) = session.menubar.dismiss_order() else {
        return;
    };
    if let Err(error) = session.renderer.start_control_dismiss(&order) {
        eprintln!("Dokka no pudo animar la salida del centro: {error}");
        session.menubar.close_tray();
    }
}

/// Descarta las animaciones y deja la barra para reconstruir limpio:
/// ninguna capa reutilizada queda encogida o invisible.
pub(crate) fn cancel_control_dismiss(session: &mut DockSession) {
    session.renderer.cancel_control_dismiss();
}

/// Avanza el tiempo hacia el cierre real y entonces recoge la ventana.
/// Sin cierre en curso no hace nada; no repinta ni anima por fotograma.
pub(crate) fn advance_menubar(session: &mut DockSession, surface: &mut Surface, seconds: f32) {
    if session.menubar.advance_control_frame(seconds) {
        crate::menubar::reset_menubar_window(session, surface);
    }
}
