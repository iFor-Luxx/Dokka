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
        DockAction::App(app_id, action) => perform_app_action(session, app_id, action)?,
        // La miniatura de Preferencias la restaura el shell mismo: la ventana
        // es propia del mismo hilo y `ShowWindow` directo la trae al frente sin
        // bloquearse con mensajes entre el mismo hilo.
        DockAction::Restore(window) => {
            if session.is_settings_window(window, surface) {
                crate::settings::open_or_focus_settings(session, surface)?;
            } else {
                let position = session.preferences.dock.position;
                let used = {
                    let (genie, dock, state) = (&mut session.genie, &session.dock, &session.state);
                    genie.on_restore_click(dock, state.apps(), position, &*surface, window)
                };
                if !used {
                    session.control.activate(window)?;
                }
            }
        }
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
    app_id: AppId,
    action: ClickAction,
) -> Result<(), Box<dyn Error>> {
    match action {
        // Preferencias nunca tiene entrada de app a la izquierda, así que no
        // llega aquí; si llegara, activar como ventana normal no bloquea.
        ClickAction::Activate(window) => {
            session.control.activate(window)?;
        }
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
