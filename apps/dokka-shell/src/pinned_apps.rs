//! Une las apps fijadas del estado con el archivo donde se guardan.

use std::error::Error;

use dokka_config::PinnedAppsFile;
use dokka_contracts::PinnedAppsStore;
use dokka_events::Event;
use dokka_model::AppInfo;

use crate::dock_session::DockSession;

/// Fija el explorador y las apps guardadas.
pub(crate) fn load_pinned(session: &mut DockSession, explorer: AppInfo) {
    session.state.apply(Event::AppPinned(explorer));
    match PinnedAppsFile::in_user_folder().and_then(|file| file.load()) {
        Ok(apps) => {
            for app in apps {
                session.state.apply(Event::AppPinned(app));
            }
        }
        Err(error) => eprintln!("No se pudieron leer las apps fijadas: {error}"),
    }
}

/// Fija o quita una app y guarda la lista.
pub(crate) fn toggle_pin(
    session: &mut DockSession,
    app: AppInfo,
    is_pinned: bool,
) -> Result<(), Box<dyn Error>> {
    let event = if is_pinned {
        Event::AppUnpinned(app.id)
    } else {
        Event::AppPinned(app)
    };
    session.state.apply(event);

    // El explorador siempre está fijado: no hace falta guardarlo.
    let pinned: Vec<AppInfo> = session
        .state
        .apps()
        .iter()
        .filter(|app| app.is_pinned && app.info.id != session.explorer)
        .map(|app| app.info.clone())
        .collect();
    PinnedAppsFile::in_user_folder()?.save(&pinned)?;
    Ok(())
}
