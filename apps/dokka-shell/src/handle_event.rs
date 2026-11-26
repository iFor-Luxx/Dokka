//! Reparte cada evento de la ventana a la pieza que corresponde.

use std::error::Error;

use dokka_win_surface::{Surface, SurfaceEvent};

use crate::dock_session::DockSession;
use crate::perform_click::perform_click;

pub(crate) fn handle_event(
    session: &mut DockSession,
    surface: &mut Surface,
    event: SurfaceEvent,
) -> Result<(), Box<dyn Error>> {
    match event {
        SurfaceEvent::PointerMoved { x, y } => {
            session.dock.pointer_moved(x, y, session.state.apps());
        }
        SurfaceEvent::PointerLeft => session.dock.pointer_left(),
        SurfaceEvent::Frame { seconds } => {
            session.dock.advance(seconds);
        }
        SurfaceEvent::Refresh => {
            if !session.sync()? {
                return Ok(());
            }
            session.resize(surface)?;
        }
        SurfaceEvent::Clicked { x, y, with_shift } => {
            perform_click(session, x, y, with_shift)?;
        }
    }
    session.render(surface)
}
