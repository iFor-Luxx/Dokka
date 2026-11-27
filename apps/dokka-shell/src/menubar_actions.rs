//! Acciones de la barra: ejecuta lo que la isla pide sin duplicar el dock.
//!
//! La isla decide y dibuja; aquí solo se conecta con la plataforma y con lo
//! que ya hace `context_menu.rs`. Abrir con el lanzador usa `SystemLauncher`
//! con `system_place`, sin contratos nuevos. Los ajustes y las apps nuevas
//! usan `menubar_menu_dismissed` para quedar delante; volumen y música
//! mantienen su panel abierto.

use std::error::Error;

use dokka_contracts::AppLauncher;
use dokka_dock::MenuAction;
use dokka_menubar::MenubarAction;
use dokka_win_apps::system_place;
use dokka_win_control::{lock_screen, show_shutdown_dialog};
use dokka_win_media::MediaAction;
use dokka_win_surface::Surface;

use crate::context_menu::run_menu_action;
use crate::dock_session::DockSession;

/// Ejecuta la opción reutilizando `context_menu.rs` donde ya existe.
pub(crate) fn run_menubar_action(
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
        MenubarAction::VolumeSetLevel(level) => {
            crate::menubar_volume::set_level(session, level)?;
        }
        MenubarAction::VolumeToggleMute => {
            crate::menubar_volume::toggle_mute(session)?;
        }
        MenubarAction::OpenSoundSettings => {
            crate::menubar_volume::open_sound_settings(session, surface)?;
        }
        MenubarAction::OpenNetworkSettings => {
            crate::menubar_network::open_network_settings(session, surface)?;
        }
        MenubarAction::ChooseLanguage(id) => {
            crate::menubar_language::choose_language(session, surface, id)?;
        }
        MenubarAction::OpenLanguageSettings => {
            crate::menubar_language::open_language_settings(session, surface)?;
        }
        MenubarAction::MusicPrevious => {
            crate::menubar_music::send_music(session, MediaAction::Previous);
        }
        MenubarAction::MusicToggle => {
            crate::menubar_music::send_music(session, MediaAction::Toggle);
        }
        MenubarAction::MusicNext => {
            crate::menubar_music::send_music(session, MediaAction::Next);
        }
        MenubarAction::TaskView => {
            surface.menubar_menu_closed();
            open_place(
                session,
                "Vista de tareas",
                "shell:::{3080F90E-D7AD-11D9-BD98-0000947B0257}",
            )?;
        }
        MenubarAction::Project => {
            surface.menubar_menu_dismissed();
            open_place(session, "Proyectar", "displayswitch.exe")?;
        }
        MenubarAction::Calculator => {
            surface.menubar_menu_dismissed();
            open_place(session, "Calculadora", "calc.exe")?;
        }
        MenubarAction::ScreenClip => {
            surface.menubar_menu_dismissed();
            open_place(session, "Recorte de pantalla", "ms-screenclip:")?;
        }
        MenubarAction::Clock => {
            surface.menubar_menu_dismissed();
            open_place(session, "Reloj", "ms-clock:")?;
        }
        MenubarAction::SearchOpen(index) => {
            crate::menubar_search::launch_search_result(session, surface, index)?;
        }
    }
    Ok(())
}

/// Abre una dirección o programa de Windows con el lanzador del shell.
fn open_place(session: &DockSession, name: &str, address: &str) -> Result<(), Box<dyn Error>> {
    session.launcher.launch(&system_place(name, address))?;
    Ok(())
}
