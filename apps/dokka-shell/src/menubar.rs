//! Barra superior: app activa, hora con calendario, menú del icono y fila
//! derecha con sus paneles reales (volumen, red, música, idioma, buscar y
//! centro de control).
//!
//! La isla decide y dibuja; aquí solo se conecta con la plataforma y con lo
//! que ya hace `context_menu.rs`, sin duplicarlo. Las acciones viven en
//! `menubar_actions.rs` para que este archivo no pase de 300 líneas. Abrir
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
    ));
    session.menubar.set_today(now.year, now.month, now.day);
}

/// Dibuja la barra en su ventana si algo cambió (mira `menubar_painted`).
pub(crate) fn render_menubar(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    let width = surface.menubar_width();
    let finger = session.menubar.painted();
    if session.menubar_painted.as_ref() == Some(&finger) {
        return Ok(());
    }
    let scene = session.menubar.scene(width);
    session.renderer.draw_menubar(&scene)?;
    let areas: Vec<[f32; 4]> = session
        .menubar
        .active_areas(width)
        .iter()
        .map(|area| [area.x, area.y, area.width, area.height])
        .collect();
    surface.menubar_set_active_areas(&areas);
    session.menubar_painted = Some(finger);
    Ok(())
}

/// Cierra el menú, el calendario o la fila si hay algo abierto, devolviendo el foco.
pub(crate) fn dismiss_menubar(session: &mut DockSession, surface: &mut Surface) {
    if session.menubar.is_menu_open() {
        session.menubar.close_menu();
        surface.menubar_menu_closed();
    } else if session.menubar.is_calendar_open() {
        session.menubar.close_calendar();
        surface.menubar_menu_closed();
    } else if session.menubar.is_tray_open() {
        session.menubar.close_tray();
        surface.menubar_menu_closed();
    }
}

/// Datos de los paneles en cada `Tick`: volumen y música siempre; red e
/// idioma cada pocos `Tick` y con la fila cerrada (el idioma activo es el
/// de la ventana al frente, antes de que la barra tome el foco).
fn refresh_panels(session: &mut DockSession) {
    crate::menubar_volume::refresh_volume(session);
    crate::menubar_music::refresh_music(session);
    session.menubar.tick_search_cursor();
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
            return Ok(false);
        }
        SurfaceEvent::MenubarMoved { x, y } => {
            let width = surface.menubar_width();
            if let Some(action) = session.menubar.pointer_moved(x, y, width) {
                run_menubar_action(session, surface, action)?;
            }
        }
        SurfaceEvent::MenubarPressed { x, y } => {
            if let Some(action) = session.menubar.pressed(x, y) {
                run_menubar_action(session, surface, action)?;
            }
        }
        SurfaceEvent::MenubarLeft => {
            session.menubar.pointer_left();
        }
        SurfaceEvent::MenubarClicked { x, y } => {
            click_menubar(session, surface, x, y)?;
        }
        SurfaceEvent::MenubarRightClicked { .. } => {}
        SurfaceEvent::MenubarTyped { character } => {
            crate::menubar_search::type_search(session, surface, character);
        }
        SurfaceEvent::MenubarBackspace => {
            crate::menubar_search::erase_search(session, surface);
        }
        SurfaceEvent::MenubarEnter => {
            crate::menubar_search::confirm_search(session, surface)?;
        }
        SurfaceEvent::MenubarUp => {
            crate::menubar_search::step_search_up(session);
        }
        SurfaceEvent::MenubarDown => {
            crate::menubar_search::step_search_down(session);
        }
        SurfaceEvent::MenubarDismissed => {
            dismiss_menubar(session, surface);
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// Clic en la barra: icono, hora, fila, píldora o cerrar fuera. Abrir el menú,
/// el calendario o la fila cierra los otros dos, el del dock y las previas;
/// cerrar devuelve el foco. Al abrir red o idioma se releen con la app de
/// delante aún al frente; al abrir buscar se calculan sus resultados.
fn click_menubar(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    let width = surface.menubar_width();
    let was_menu = session.menubar.is_menu_open();
    let was_calendar = session.menubar.is_calendar_open();
    let was_tray = session.menubar.is_tray_open();
    let chosen = session.menubar.click(x, y, width);
    if let Some(action) = chosen {
        return run_menubar_action(session, surface, action);
    }
    if !was_tray && session.menubar.is_tray_open() {
        match session.menubar.tray_kind() {
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
        // Cambio directo entre menú, calendario y fila: ya tenía el foco.
        session.dock.close_menu();
        surface.menu_closed();
        session.previews.close();
    } else if was_open && !now_open {
        surface.menubar_menu_closed();
    }
    Ok(())
}
