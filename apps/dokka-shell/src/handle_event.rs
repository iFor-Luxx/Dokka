//! Reparte cada evento de la ventana a la pieza que corresponde.

use std::error::Error;

use dokka_contracts::WindowControl;
use dokka_model::WindowId;
use dokka_previews::PreviewClick;
use dokka_win_surface::{Surface, SurfaceEvent};

use crate::context_menu::{open_context_menu, run_menu_action};
use crate::dock_session::DockSession;
use crate::floating_panels::preview_request;
use crate::media::send_media;
use crate::perform_click::perform_click;

pub(crate) fn handle_event(
    session: &mut DockSession,
    surface: &mut Surface,
    event: SurfaceEvent,
) -> Result<(), Box<dyn Error>> {
    match event {
        SurfaceEvent::PointerMoved { x, y } => {
            if session.dock.is_menu_open() {
                session.dock.move_over_menu(x, y);
            } else {
                session.dock.pointer_moved(x, y, session.state.apps());
                let request = preview_request(session, x, y);
                session.previews.pointer_moved(x, y, request);
            }
        }
        SurfaceEvent::PointerLeft => {
            if !session.dock.is_menu_open() {
                session.dock.pointer_left();
                session.previews.pointer_left();
            }
        }
        SurfaceEvent::Frame { seconds } => {
            session.dock.advance(seconds);
            session.previews.advance(seconds);
        }
        SurfaceEvent::Refresh => {
            if !session.sync()? {
                return Ok(());
            }
            // Un panel podría estar mostrando una ventana que ya no existe.
            session.previews.close();
            if session.dock.is_menu_open() {
                session.dock.close_menu();
                surface.menu_closed();
            }
            session.resize(surface)?;
        }
        SurfaceEvent::Tick => session.update_clock_and_trash(),
        SurfaceEvent::Dismissed => {
            if session.dock.is_menu_open() {
                session.dock.close_menu();
                surface.menu_closed();
            }
        }
        SurfaceEvent::Clicked { x, y, with_shift } => {
            click(session, surface, x, y, with_shift)?;
        }
        SurfaceEvent::RightClicked { x, y } => {
            session.previews.close();
            open_context_menu(session, surface, x, y)?;
        }
    }
    session.render(surface)
}

/// Los paneles están por encima: tienen prioridad sobre el dock.
fn click(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
    with_shift: bool,
) -> Result<(), Box<dyn Error>> {
    if session.dock.is_menu_open() {
        if let Some(action) = session.dock.click_menu(x, y) {
            return run_menu_action(session, surface, action);
        }
        // Un clic fuera del menú sobre el dock solo cierra el menú.
        if !session.dock.menu_contains(x, y) {
            session.dock.close_menu();
            surface.menu_closed();
        }
        return Ok(());
    }
    match session.previews.click(x, y) {
        Some(PreviewClick::Activate(window)) => {
            session.control.activate(WindowId::new(window))?;
        }
        Some(PreviewClick::Media(command)) => send_media(session, command),
        Some(PreviewClick::Handled) => {}
        None => {
            session.previews.close();
            perform_click(session, x, y, with_shift)?;
        }
    }
    Ok(())
}
