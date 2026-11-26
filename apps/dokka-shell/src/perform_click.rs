//! Lleva a Windows la acción que el dock decidió para un clic.

use std::error::Error;

use dokka_contracts::{AppLauncher, WindowControl};
use dokka_dock::ClickAction;
use dokka_win_control::foreground_window;

use crate::dock_session::DockSession;

pub(crate) fn perform_click(
    session: &mut DockSession,
    x: f32,
    y: f32,
    with_shift: bool,
) -> Result<(), Box<dyn Error>> {
    let apps = session.state.apps();
    let Some((app_id, action)) = session
        .dock
        .click(x, y, apps, foreground_window(), with_shift)
    else {
        return Ok(());
    };

    match action {
        ClickAction::Activate(window) => session.control.activate(window)?,
        ClickAction::Minimize(windows) => {
            for window in windows {
                session.control.minimize(window)?;
            }
        }
        ClickAction::Launch => {
            if let Some(app) = apps.iter().find(|app| app.info.id == app_id) {
                session.launcher.launch(&app.info)?;
                session.dock.start_bounce(app_id, apps);
            }
        }
    }
    Ok(())
}
