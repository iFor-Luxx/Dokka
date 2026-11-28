//! Barra superior: app activa, hora con calendario, menú del icono y fila
//! derecha con sus paneles reales (volumen, red, música, segundo plano,
//! idioma, buscar y centro de control).
//!
//! La isla decide y dibuja; aquí solo se conecta con la plataforma y con lo
//! que ya hace `context_menu.rs`, sin duplicarlo. Las acciones viven en
//! `menubar_actions.rs`, el clic en `menubar_click.rs` y el cierre con su
//! salida por piezas en `menubar_dismiss.rs`. Abrir
//! el menú, el calendario o la fila cierra los otros dos, el del dock y las
//! vistas previas; abrir el del dock o un panel los cierra a ellos.

use std::error::Error;

use dokka_menubar::clock_text;
use dokka_theme::menubar;
use dokka_win_surface::{Surface, SurfaceEvent, local_time};
use dokka_win_tracking::foreground_app;

use crate::dock_session::DockSession;
use crate::menubar_actions::run_menubar_action;

/// Muestra la barra con las medidas del tema. Llamar una vez al arrancar.
pub(crate) fn setup_menubar(surface: &mut Surface) -> Result<(), Box<dyn Error>> {
    surface.menubar_show(menubar::HEIGHT, menubar::WINDOW_HEIGHT)?;
    Ok(())
}

/// Nombre de la app al frente (`NameCache`); si no hay ninguna, conserva el último.
pub(crate) fn refresh_foreground(session: &mut DockSession) {
    let Some(info) = foreground_app() else {
        return;
    };
    let name = session
        .names
        .name_for(&info.executable)
        .unwrap_or(info.name);
    session.menubar.set_app(name);
}

/// Hora de la barra y hoy del calendario, desde el reloj local.
pub(crate) fn refresh_clock(session: &mut DockSession) {
    let now = local_time();
    session.menubar.set_clock(clock_text(
        now.weekday,
        now.day,
        now.month,
        now.hour,
        now.minute,
        session.preferences.general.hour_24,
    ));
    session.menubar.set_today(now.year, now.month, now.day);
}

/// Dibuja la barra en su ventana si algo cambió (mira `menubar_painted`).
/// Con la salida por piezas en curso no repinta: Composition la anima sin
/// que Dokka pise opacidad, escala ni posición (ni el `Tick` ni la música).
pub(crate) fn render_menubar(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if session.menubar.is_control_closing() {
        return Ok(());
    }
    let width = surface.menubar_width();
    let screen_height = surface.menubar_monitor_height();
    let finger = session.menubar.painted();
    if session.menubar_painted.as_ref() == Some(&finger) {
        return Ok(());
    }
    let scene = session.menubar.scene(width, screen_height);
    session.renderer.draw_menubar(&scene)?;
    let areas: Vec<[f32; 4]> = session
        .menubar
        .active_areas(width, screen_height)
        .iter()
        .map(|area| [area.x, area.y, area.width, area.height])
        .collect();
    surface.menubar_set_active_areas(&areas);
    session.menubar_painted = Some(finger);
    Ok(())
}

/// Con la galería abierta la ventana necesita todo el alto de la pantalla
/// (como el buscador); al cerrar vuelve a su alto para los demás paneles.
pub(crate) fn reset_menubar_window(session: &DockSession, surface: &mut Surface) {
    if session.menubar.is_gallery_open() {
        let height = session
            .menubar
            .gallery_window_height(surface.menubar_monitor_height());
        surface.menubar_set_window_height(height);
        return;
    }
    if session.menubar.tray_kind() != Some(dokka_menubar::TrayKind::Search) {
        surface.menubar_set_window_height(menubar::WINDOW_HEIGHT);
    }
}

/// Datos de los paneles en cada `Tick`: volumen, brillo (solo el último valor
/// conocido, sin DDC) y música siempre; red e idioma cada pocos `Tick` y con
/// la fila cerrada (el idioma activo es el de la ventana al frente, antes de
/// que la barra tome el foco).
fn refresh_panels(session: &mut DockSession) {
    crate::menubar_volume::refresh_volume(session);
    crate::menubar_brightness::refresh_brightness(session);
    crate::menubar_music::refresh_music(session);
    crate::menubar_background::refresh_background_apps(session);
    session.menubar.tick_search_cursor();
    session.menubar.tick_gallery_cursor();
    session.menubar_ticks += 1;
    if session.menubar_ticks.is_multiple_of(3) {
        crate::menubar_network::refresh_network(session);
        // Con la fila abierta el foco es de Dokka y el activo saldría mal.
        if !session.menubar.is_tray_open() {
            crate::menubar_language::refresh_languages(session);
        }
    }
}

/// Atiende un evento de la plataforma. Devuelve si lo consumió.
/// `Refresh` y `Tick` solo actualizan datos y dejan seguir al resto.
pub(crate) fn handle_menubar_event(
    session: &mut DockSession,
    surface: &mut Surface,
    event: SurfaceEvent,
) -> Result<bool, Box<dyn Error>> {
    match event {
        SurfaceEvent::Refresh => {
            refresh_foreground(session);
            crate::menubar_search::refresh_search_results(session, surface);
            return Ok(false);
        }
        SurfaceEvent::Tick => {
            refresh_foreground(session);
            refresh_clock(session);
            refresh_panels(session);
            // Solo el centro se cierra solo: por el mismo camino que al
            // perder el foco (devuelve el foco igual, sin robarlo).
            if session.menubar.tick_control_idle() {
                crate::menubar_dismiss::dismiss_menubar(session, surface);
            }
            return Ok(false);
        }
        SurfaceEvent::MenubarMoved { x, y } => {
            let width = surface.menubar_width();
            if let Some(action) = session.menubar.pointer_moved(x, y, width) {
                run_menubar_action(session, surface, action)?;
            }
            session.menubar.update_control_cursor(x, y);
        }
        SurfaceEvent::MenubarPressed { x, y } => {
            if let Some(action) = session.menubar.pressed(x, y) {
                run_menubar_action(session, surface, action)?;
            }
            session.menubar.note_control_used();
            session.menubar.update_control_cursor(x, y);
        }
        SurfaceEvent::MenubarLeft => {
            session.menubar.pointer_left();
        }
        SurfaceEvent::MenubarClicked { x, y } => {
            crate::menubar_click::click_menubar(session, surface, x, y)?;
            session.menubar.note_control_used();
            session.menubar.update_control_cursor(x, y);
        }
        SurfaceEvent::MenubarRightClicked { x, y } => {
            crate::menubar_click::right_click_menubar(session, surface, x, y)?;
            session.menubar.note_control_used();
        }
        SurfaceEvent::MenubarTyped { character } => {
            session.menubar.note_control_used();
            crate::menubar_gallery::typed(session, surface, character);
        }
        SurfaceEvent::MenubarBackspace => {
            session.menubar.note_control_used();
            crate::menubar_gallery::erased(session, surface);
        }
        SurfaceEvent::MenubarWheel { x, y, delta } => {
            session.menubar.note_control_used();
            crate::menubar_gallery::scrolled(session, surface, x, y, delta);
        }
        SurfaceEvent::MenubarEnter => {
            session.menubar.note_control_used();
            crate::menubar_search::confirm_search(session, surface)?;
        }
        SurfaceEvent::MenubarUp => {
            session.menubar.note_control_used();
            crate::menubar_search::step_search_up(session);
        }
        SurfaceEvent::MenubarDown => {
            session.menubar.note_control_used();
            crate::menubar_search::step_search_down(session);
        }
        SurfaceEvent::MenubarDismissed => {
            crate::menubar_dismiss::dismiss_menubar(session, surface);
        }
        _ => return Ok(false),
    }
    Ok(true)
}
