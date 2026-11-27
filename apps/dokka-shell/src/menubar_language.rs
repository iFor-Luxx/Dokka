//! Idioma de la barra: lista real, código y activación.
//!
//! `SystemKeyboard` marca como activo el de la ventana al frente, así que
//! la lista se lee antes de que la barra tome el foco (o con la fila
//! cerrada); elegir publica el cambio en la app que estaba delante.

use std::error::Error;

use dokka_components::language_panel::LanguageEntry;
use dokka_contracts::KeyboardLanguages;
use dokka_win_control::SystemKeyboard;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Idiomas instalados y código corto para la barra.
/// El shell solo la llama con la fila cerrada o al abrir el panel: el
/// idioma activo es el de la ventana que estaba al frente.
pub(crate) fn refresh_languages(session: &mut DockSession) {
    match SystemKeyboard.languages() {
        Ok(languages) => {
            let code = languages
                .iter()
                .find(|language| language.is_active)
                .map(|language| language.code.clone())
                .unwrap_or_else(|| "ES".to_owned());
            let entries = languages
                .iter()
                .map(|language| LanguageEntry {
                    id: language.id,
                    name: language.name.clone(),
                    is_active: language.is_active,
                })
                .collect();
            session.menubar.set_languages(entries, code);
        }
        Err(error) => eprintln!("Dokka no pudo leer los idiomas: {error}"),
    }
}

/// Activa el idioma en la app que estaba al frente: primero se cierra la
/// fila y se devuelve el foco, y después se publica el cambio.
pub(crate) fn choose_language(
    session: &mut DockSession,
    surface: &mut Surface,
    id: u64,
) -> Result<(), Box<dyn Error>> {
    session.menubar.close_tray();
    surface.menubar_menu_closed();
    if let Err(error) = SystemKeyboard.activate_language(id) {
        eprintln!("Dokka no pudo cambiar el idioma: {error}");
    }
    refresh_languages(session);
    Ok(())
}

/// Abre los ajustes de idioma por delante, sin devolver el foco antes.
pub(crate) fn open_language_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    use dokka_contracts::AppLauncher as _;
    use dokka_win_apps::system_place;

    surface.menubar_menu_dismissed();
    session.launcher.launch(&system_place(
        "Configuración de idioma",
        "ms-settings:regionlanguage",
    ))?;
    Ok(())
}
