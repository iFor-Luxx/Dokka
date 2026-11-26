//! Acciones del menú de apps e iconos del sistema.

use std::error::Error;

use dokka_contracts::{AppActions, AppLauncher, RecycleBin, WindowControl};
use dokka_dock::MenuAction;
use dokka_win_apps::system_place;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Ejecuta una acción de app o icono del sistema. `true` si la atendió.
pub(crate) fn run_app_menu_action(
    session: &mut DockSession,
    surface: &mut Surface,
    action: MenuAction,
) -> Result<bool, Box<dyn Error>> {
    match action {
        MenuAction::ActivateWindow(window) => {
            session.control.activate(window)?;
        }
        MenuAction::OpenApp(info) => {
            session.launcher.launch(&info)?;
        }
        MenuAction::RunAsAdministrator(info) => {
            session.app_actions.run_as_administrator(&info)?;
        }
        MenuAction::ShowInFolder(info) => {
            session.app_actions.show_in_folder(&info)?;
        }
        MenuAction::CloseFrontWindow(windows) => {
            if let Some(front) = session.control.front_window(&windows) {
                session.control.close(front)?;
            }
        }
        MenuAction::QuitApp(windows) => {
            // Cierra como su X, una por una; nunca mata el proceso.
            for window in windows {
                let _ = session.control.close(window);
            }
        }
        MenuAction::OpenDownloads => {
            session
                .launcher
                .launch(&system_place("Descargas", "shell:Downloads"))?;
        }
        MenuAction::OpenDocuments => {
            session
                .launcher
                .launch(&system_place("Documentos", "shell:Personal"))?;
        }
        MenuAction::OpenDesktop => {
            session
                .launcher
                .launch(&system_place("Escritorio", "shell:Desktop"))?;
        }
        MenuAction::OpenPictures => {
            session
                .launcher
                .launch(&system_place("Imágenes", "shell:My Pictures"))?;
        }
        MenuAction::OpenTrash => {
            session
                .launcher
                .launch(&system_place("Papelera", "shell:RecycleBinFolder"))?;
        }
        MenuAction::EmptyTrash => {
            session.recycle_bin.empty()?;
        }
        MenuAction::OpenLaunchpad => {
            crate::launchpad::open_launchpad(session, surface)?;
        }
        _ => return Ok(false),
    }
    Ok(true)
}
