//! Cierre de la barra con fundido del centro de control.
//!
//! El menú, el calendario y los demás paneles se cierran al instante como
//! hoy. El centro (sin galería) se desvanece en `DISMISS_SECONDS`
//! encogiéndose un poco hacia su icono: la isla guarda el progreso y cada
//! fotograma avanza hasta el cierre real. El foco vuelve al instante, sin
//! esperar al fundido; la ventana y las zonas activas se mantienen hasta que
//! termina para que no se recorte. Sin hilos ni temporizadores nuevos: valen
//! los fotogramas que ya pide `is_animating`.

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Cierra el menú, el calendario o la fila. El centro se desvanece y el foco
/// vuelve ya; lo demás se cierra al instante como hoy.
pub(crate) fn dismiss_menubar(session: &mut DockSession, surface: &mut Surface) {
    if session.menubar.is_menu_open() {
        session.menubar.close_menu();
        surface.menubar_menu_closed();
    } else if session.menubar.is_calendar_open() {
        session.menubar.close_calendar();
        surface.menubar_menu_closed();
    } else if session.menubar.is_tray_open() {
        if session.menubar.begin_control_close() {
            surface.menubar_menu_closed();
            return;
        }
        session.menubar.close_tray();
        surface.menubar_menu_closed();
    }
    crate::menubar::reset_menubar_window(session, surface);
}

/// Avanza un fotograma del fundido; al terminar hace el cierre real y recoge
/// la ventana. Sin cierre en curso no hace nada.
pub(crate) fn advance_menubar(session: &mut DockSession, surface: &mut Surface, seconds: f32) {
    if session.menubar.advance_control_frame(seconds) {
        crate::menubar::reset_menubar_window(session, surface);
    }
}
