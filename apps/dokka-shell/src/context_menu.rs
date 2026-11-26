//! Menú del clic derecho sobre una app.

use std::error::Error;

use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::pinned_apps::toggle_pin;

pub(crate) fn open_context_menu(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    let Some(app) = session.dock.pin_target(x, y, session.state.apps()) else {
        return Ok(());
    };
    let (info, is_pinned) = (app.info.clone(), app.is_pinned);
    let option = if is_pinned {
        "Quitar del dock"
    } else {
        "Fijar en el dock"
    };

    let chosen = surface.show_menu(&[option])?;
    // El menú se queda con el mouse: la lupa no recibe el aviso de salida.
    session.dock.pointer_left();
    if chosen.is_some() {
        toggle_pin(session, info, is_pinned)?;
        session.resize(surface)?;
    }
    Ok(())
}
