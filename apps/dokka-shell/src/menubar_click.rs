//! Clic en la barra superior: icono, hora, fila y cerrar fuera.
//!
//! Abrir el menú, el calendario o la fila cierra los otros dos, el del dock
//! y las vistas previas; cerrar devuelve el foco. Al abrir red o idioma se
//! releen con la app de delante aún al frente; al abrir buscar se calculan
//! sus resultados. La salida del centro por piezas se lanza y se cancela
//! por aquí (`menubar_dismiss.rs`); este archivo solo decide el clic.

use std::error::Error;

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::menubar_actions::run_menubar_action;

/// Clic en la barra: icono, hora, fila, píldora o cerrar fuera. Abrir el menú,
/// el calendario o la fila cierra los otros dos, el del dock y las previas;
/// cerrar devuelve el foco. Al abrir red o idioma se releen con la app de
/// delante aún al frente; al abrir buscar se calculan sus resultados.
pub(crate) fn click_menubar(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    let width = surface.menubar_width();
    let screen_height = surface.menubar_monitor_height();
    let was_menu = session.menubar.is_menu_open();
    let was_calendar = session.menubar.is_calendar_open();
    let was_tray = session.menubar.is_tray_open();
    let was_closing = session.menubar.is_control_closing();
    let chosen = session.menubar.click(x, y, width, screen_height);
    if let Some(action) = chosen {
        return run_menubar_action(session, surface, action);
    }
    if session.menubar.is_control_closing() {
        // Salida en curso (empezada o seguida): el foco vuelve ya; la
        // ventana y las zonas se mantienen hasta el cierre real.
        surface.menubar_menu_closed();
        // Recién empezada por este clic (repetir su icono o fuera): lanzar
        // las animaciones una sola vez; si fallan, cerrar al instante.
        if !was_closing
            && let Some(order) = session.menubar.dismiss_order()
            && let Err(error) = session.renderer.start_control_dismiss(&order)
        {
            eprintln!("Dokka no pudo animar la salida del centro: {error}");
            session.menubar.close_tray();
            crate::menubar::reset_menubar_window(session, surface);
        }
        return Ok(());
    }
    if was_closing {
        // Se canceló (reabrir) o se cambió de panel: descartar lo animado
        // para reconstruir limpio antes de dibujar lo nuevo.
        crate::menubar_dismiss::cancel_control_dismiss(session);
    }
    if !was_tray && session.menubar.is_tray_open() {
        match session.menubar.tray_kind() {
            Some(dokka_menubar::TrayKind::Volume) => {
                crate::menubar_volume::refresh_audio_outputs(session);
            }
            Some(dokka_menubar::TrayKind::ControlCenter) => {
                crate::menubar_brightness::request_brightness(session);
            }
            Some(dokka_menubar::TrayKind::Network) => {
                crate::menubar_network::refresh_network(session);
            }
            Some(dokka_menubar::TrayKind::Language) => {
                crate::menubar_language::refresh_languages(session);
            }
            Some(dokka_menubar::TrayKind::Search) => {
                crate::menubar_search::refresh_search_results(session, surface);
            }
            _ => {}
        }
    }
    let now_menu = session.menubar.is_menu_open();
    let now_calendar = session.menubar.is_calendar_open();
    let now_tray = session.menubar.is_tray_open();
    let was_open = was_menu || was_calendar || was_tray;
    let now_open = now_menu || now_calendar || now_tray;
    if !was_open && now_open {
        // Se acaba de abrir: fuera el menú del dock y las previas.
        session.dock.close_menu();
        surface.menu_closed();
        session.previews.close();
        surface.menubar_menu_opened();
    } else if was_open
        && now_open
        && (was_menu != now_menu || was_calendar != now_calendar || was_tray != now_tray)
    {
        // Cambio directo entre menú, calendario y fila: ya tenía el foco,
        // salvo que venga de cancelar el fundido (ya se había devuelto).
        session.dock.close_menu();
        surface.menu_closed();
        session.previews.close();
        if was_closing {
            surface.menubar_menu_opened();
        }
    } else if was_open && !now_open {
        surface.menubar_menu_closed();
    }
    crate::menubar::reset_menubar_window(session, surface);
    Ok(())
}

/// Clic derecho en la barra: en una fila de segundo plano abre su menú;
/// en su botón abre el panel como el clic izquierdo.
pub(crate) fn right_click_menubar(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    let width = surface.menubar_width();
    if let Some(action) = session.menubar.background_right_clicked(x, y, width) {
        return run_menubar_action(session, surface, action);
    }
    Ok(())
}
