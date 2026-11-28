//! Datos del equipo para "Acerca de este PC": pedir, pasar y abrir.
//!
//! La isla no habla con Windows: el shell pide el resumen al entrar en
//! "Acerca", lo pasa cuando el hilo lo lee y abre las páginas de los botones.
//! Falla sola: si no se lee, el campo queda en "No disponible".

use dokka_settings::{AboutButton, GeneralPage};

use super::dock_session::DockSession;

/// Pide el resumen del equipo una vez y pasa lo que ya haya al abrir "Acerca".
pub(crate) fn request_about(session: &mut DockSession) {
    use dokka_contracts::SystemInfo as _;
    if let Some(summary) = session.system_info.summary() {
        session.settings.set_system_info(Some(summary));
    } else {
        session.system_info.request_refresh();
    }
}

/// Si se está en "Acerca" y el hilo ya leyó, pasa el último valor a la isla.
pub(crate) fn poll_about(session: &mut DockSession) {
    if session.settings.general() != GeneralPage::About {
        return;
    }
    use dokka_contracts::SystemInfo as _;
    if let Some(summary) = session.system_info.summary()
        && session.settings.system_summary() != Some(summary.clone())
    {
        session.settings.set_system_info(Some(summary));
    }
}

/// Abre la página del sistema del botón pulsado. Falla sola.
pub(crate) fn open_about_page(session: &DockSession, button: AboutButton) {
    use dokka_contracts::{SystemInfo as _, SystemPage};
    let page = match button {
        AboutButton::DisplaySettings => SystemPage::Display,
        AboutButton::StorageSettings => SystemPage::Storage,
        AboutButton::SystemReport => SystemPage::Report,
    };
    if let Err(error) = session.system_info.open_page(page) {
        eprintln!("Dokka no pudo abrir la página del sistema: {error}");
    }
}

/// Rueda sobre Preferencias: baja la ficha "Acerca de este PC".
pub(crate) fn scroll_settings(session: &mut DockSession, delta: i16) {
    if session.settings.is_open() {
        session.settings.scroll_wheel(delta);
    }
}
