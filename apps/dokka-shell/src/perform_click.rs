//! Lleva a Windows la acción que el dock decidió para un clic.

use std::error::Error;

use dokka_contracts::{AppLauncher, WindowControl};
use dokka_dock::{ClickAction, DockAction};
use dokka_model::AppId;
use dokka_win_apps::system_place;
use dokka_win_control::{foreground_window, open_start_menu};

use crate::dock_session::DockSession;

pub(crate) fn perform_click(
    session: &mut DockSession,
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

    match action {
        DockAction::App(app_id, action) => perform_app_action(session, app_id, action)?,
        DockAction::Restore(window) => session.control.activate(window)?,
        // Provisional hasta tener nuestro propio Launchpad.
        DockAction::Launchpad => open_start_menu(),
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
    app_id: AppId,
    action: ClickAction,
) -> Result<(), Box<dyn Error>> {
    match action {
        ClickAction::Activate(window) => session.control.activate(window)?,
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
