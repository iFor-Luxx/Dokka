//! Barra superior: app activa, hora con calendario y menú del icono.
//!
//! La isla decide y dibuja; aquí solo se conecta con la plataforma y con lo
//! que ya hace `context_menu.rs`, sin duplicarlo. Abrir el menú o el
//! calendario cierra el otro, el del dock y las vistas previas; abrir el del
//! dock o un panel lo cierra a él.

use std::error::Error;

use dokka_dock::MenuAction;
use dokka_menubar::{MenubarAction, clock_text};
use dokka_theme::menubar;
use dokka_win_control::{lock_screen, show_shutdown_dialog};
use dokka_win_surface::{Surface, SurfaceEvent, local_time};
use dokka_win_tracking::foreground_app;

use crate::context_menu::run_menu_action;
use crate::dock_session::DockSession;

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

/// Cierra el menú o el calendario si hay alguno abierto, devolviendo el foco.
pub(crate) fn dismiss_menubar(session: &mut DockSession, surface: &mut Surface) {
    if session.menubar.is_menu_open() {
        session.menubar.close_menu();
        surface.menubar_menu_closed();
    } else if session.menubar.is_calendar_open() {
        session.menubar.close_calendar();
        surface.menubar_menu_closed();
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
            return Ok(false);
        }
        SurfaceEvent::Tick => {
            refresh_foreground(session);
            refresh_clock(session);
            return Ok(false);
        }
        SurfaceEvent::MenubarMoved { x, y } => {
            let width = surface.menubar_width();
            session.menubar.pointer_moved(x, y, width);
        }
        SurfaceEvent::MenubarLeft => {
            session.menubar.pointer_left();
        }
        SurfaceEvent::MenubarClicked { x, y } => {
            click_menubar(session, surface, x, y)?;
        }
        SurfaceEvent::MenubarDismissed => {
            dismiss_menubar(session, surface);
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// Clic en la barra: icono, hora, píldora o cerrar fuera. Abrir el menú o el
/// calendario cierra el otro, el del dock y las previas; cerrar devuelve el foco.
fn click_menubar(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    let width = surface.menubar_width();
    let was_menu = session.menubar.is_menu_open();
    let was_calendar = session.menubar.is_calendar_open();
    let chosen = session.menubar.click(x, y, width);
    if let Some(action) = chosen {
        return run_menubar_action(session, surface, action);
    }
    let now_menu = session.menubar.is_menu_open();
    let now_calendar = session.menubar.is_calendar_open();
    if !was_menu && !was_calendar && (now_menu || now_calendar) {
        // Se acaba de abrir: fuera el menú del dock y las previas.
        session.dock.close_menu();
        surface.menu_closed();
        session.previews.close();
        surface.menubar_menu_opened();
    } else if (was_menu && now_calendar) || (was_calendar && now_menu) {
        // Cambio directo entre menú y calendario: ya tenía el foco.
        session.dock.close_menu();
        surface.menu_closed();
        session.previews.close();
    } else if (was_menu || was_calendar) && !now_menu && !now_calendar {
        surface.menubar_menu_closed();
    }
    Ok(())
}

/// Ejecuta la opción reutilizando `context_menu.rs` donde ya existe.
fn run_menubar_action(
    session: &mut DockSession,
    surface: &mut Surface,
    action: MenubarAction,
) -> Result<(), Box<dyn Error>> {
    match action {
        // Se actúa mientras la barra aún es la ventana activa, si no la
        // ventana nueva queda detrás; el resto devuelve el foco como antes.
        MenubarAction::OpenSettings => {
            surface.menubar_menu_dismissed();
            run_menu_action(session, surface, MenuAction::OpenSettings)?;
        }
        MenubarAction::ShowDesktop => {
            surface.menubar_menu_closed();
            run_menu_action(session, surface, MenuAction::ShowDesktop)?;
        }
        MenubarAction::TaskManager => {
            surface.menubar_menu_dismissed();
            run_menu_action(session, surface, MenuAction::TaskManager)?;
        }
        MenubarAction::LockScreen => {
            surface.menubar_menu_closed();
            lock_screen();
        }
        MenubarAction::Power => {
            surface.menubar_menu_closed();
            show_shutdown_dialog();
        }
        MenubarAction::Restart => {
            surface.menubar_menu_closed();
            run_menu_action(session, surface, MenuAction::Restart)?;
            surface.menubar_hide();
        }
        MenubarAction::Quit => {
            surface.menubar_menu_closed();
            run_menu_action(session, surface, MenuAction::Quit)?;
            surface.menubar_hide();
        }
    }
    Ok(())
}
