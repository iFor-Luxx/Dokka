//! Guarda y aplica en el acto lo ajustado en Preferencias.
//!
//! Lee con `PreferencesFile` (lo crea otro agente en `dokka-config` con
//! `PreferencesStore`: `load`/`save`) y lo aplica al dock, a las vistas
//! previas y a la barra. Un error al leer o guardar se registra y se sigue:
//! nunca cierra Dokka.

use dokka_config::PreferencesFile;
use dokka_contracts::PreferencesStore;
use dokka_model::{HiddenControls, Preferences};
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::menubar_background::apply_background_apps;
use crate::taskbar_mode::{hide_taskbar_now, persist_taskbar_mode, show_taskbar_now};

/// Lee lo guardado y lo deja en la sesión, el dock y la barra.
/// Llamar una vez al arrancar, antes de dibujar. Sin archivo, fábrica.
pub(crate) fn load_preferences(session: &mut DockSession) {
    // Si no se pueden leer, valen los de fábrica que ya tiene la sesión.
    let stored = match PreferencesFile::in_user_folder().and_then(|file| file.load()) {
        Ok(preferences) => preferences,
        Err(error) => {
            eprintln!("No se pudieron leer las preferencias: {error}");
            session.preferences
        }
    };
    session.preferences = stored;
    session.dock.set_preferences(stored.dock);
    session.dock.set_clock_24(stored.general.hour_24);
    session.refresh_occlusion();
    session.menubar.set_shown(stored.menubar);
    session
        .menubar
        .set_week_start(stored.general.week_starts_monday);
    session
        .previews
        .set_week_start(stored.general.week_starts_monday);
    apply_background_apps(session, stored.menubar.background_apps);
    if !stored.dock.previews {
        session.previews.close();
    }
}

pub(crate) fn apply_preferences(
    session: &mut DockSession,
    surface: &mut Surface,
    preferences: Preferences,
) {
    if session.preferences == preferences {
        return;
    }
    let saved = PreferencesFile::in_user_folder().and_then(|file| file.save(&preferences));
    if let Err(error) = saved {
        eprintln!("No se pudieron guardar las preferencias: {error}");
    }
    session.preferences = preferences;
    session.dock.set_preferences(preferences.dock);
    session.dock.set_clock_24(preferences.general.hour_24);
    session.refresh_occlusion();
    session.menubar.set_shown(preferences.menubar);
    session
        .menubar
        .set_week_start(preferences.general.week_starts_monday);
    session
        .previews
        .set_week_start(preferences.general.week_starts_monday);
    apply_background_apps(session, preferences.menubar.background_apps);
    if !preferences.dock.previews {
        session.previews.close();
    }
    if let Err(error) = session.resize(surface) {
        eprintln!("Dokka no pudo recolocar el dock con las preferencias: {error}");
    }
}

/// Guarda los controles ocultos del centro por el mismo camino que
/// Preferencias (mismo archivo) y lo aplica en el acto: la barra reacomoda
/// sin cerrar y la copia de la ventana de Preferencias (abierta o no) queda
/// actualizada para que un cambio posterior allí no los devuelva.
pub(crate) fn apply_hidden_controls(session: &mut DockSession, hidden: HiddenControls) {
    if session.preferences.menubar.hidden_controls == hidden {
        return;
    }
    let mut preferences = session.preferences;
    preferences.menubar.hidden_controls = hidden;
    let saved = PreferencesFile::in_user_folder().and_then(|file| file.save(&preferences));
    if let Err(error) = saved {
        eprintln!("No se pudieron guardar las preferencias: {error}");
    }
    session.preferences = preferences;
    session.menubar.set_shown(preferences.menubar);
    // La isla de Preferencias guarda su propia copia al abrir; se sincroniza
    // también si está abierta para no pisar lo oculto desde el centro.
    session
        .settings
        .set_values(preferences, session.taskbar_auto());
}

/// Guarda el modo de la barra y la oculta o muestra en el acto.
/// Reutiliza lo que ya hace el menú del dock, sin duplicarlo.
pub(crate) fn set_taskbar_auto(session: &mut DockSession, surface: &mut Surface, auto: bool) {
    if let Err(error) = persist_taskbar_mode(auto) {
        eprintln!("No se pudo guardar el modo de la barra: {error}");
    }
    session.set_taskbar_auto(auto);
    let applied = if auto {
        hide_taskbar_now(session, surface)
    } else {
        show_taskbar_now(session, surface)
    };
    if let Err(error) = applied {
        eprintln!("Dokka no pudo cambiar la barra de Windows: {error}");
    }
}
