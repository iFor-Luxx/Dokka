//! Lleva a Windows la acción que el dock decidió para un clic.

use std::error::Error;

use dokka_contracts::{AppLauncher, WindowControl};
use dokka_dock::{ClickAction, DockAction};
use dokka_model::AppId;
use dokka_win_apps::system_place;
use dokka_win_control::foreground_window;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::launchpad::{begin_close_launchpad, toggle_launchpad};
use crate::window_activation::activate_window;

pub(crate) fn perform_click(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
    with_shift: bool,
) -> Result<(), Box<dyn Error>> {
    let apps = session.state.apps();
    let Some(action) = session
        .dock
        .click(x, y, apps, foreground_window(), with_shift)
    else {
        return Ok(());
    };

    // Cualquier otra cosa del dock cierra primero el Launchpad.
    if action != DockAction::Launchpad && session.launchpad.is_open() {
        begin_close_launchpad(session, surface);
    }
    match action {
        DockAction::App(app_id, action) => perform_app_action(session, surface, app_id, action)?,
        DockAction::Restore(window) => activate_window(session, surface, window)?,
        DockAction::Launchpad => toggle_launchpad(session, surface)?,
        DockAction::Calendar => open_place(session, "Calendario", "outlookcal:")?,
        DockAction::Clock => open_place(session, "Reloj", "ms-clock:")?,
        DockAction::Trash => open_place(session, "Papelera", "shell:RecycleBinFolder")?,
    }
    Ok(())
}

fn open_place(session: &DockSession, name: &str, address: &str) -> Result<(), Box<dyn Error>> {
    session.launcher.launch(&system_place(name, address))?;
    Ok(())
}

fn perform_app_action(
    session: &mut DockSession,
    surface: &mut Surface,
    app_id: AppId,
    action: ClickAction,
) -> Result<(), Box<dyn Error>> {
    match action {
        ClickAction::Activate(window) => activate_window(session, surface, window)?,
        ClickAction::Minimize(windows) => {
            for window in windows {
                session.control.minimize(window)?;
            }
        }
        ClickAction::Launch => {
            let apps = session.state.apps();
            if let Some(app) = apps.iter().find(|app| app.info.id == app_id) {
                session.launcher.launch(&app.info)?;
                session.dock.start_bounce(app_id, apps);
            }
        }
    }
    Ok(())
}
