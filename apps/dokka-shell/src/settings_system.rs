//! Datos del equipo para "Acerca de este PC" y "Almacenamiento": pedir,
//! pasar y abrir.
//!
//! La isla no habla con Windows: el shell pide el resumen al entrar en
//! "Acerca" y el desglose al entrar en "Almacenamiento", los pasa cuando los
//! hilos los leen y abre las páginas de los botones. Falla sola: si no se
//! lee, el campo queda en "No disponible" o "Calculando…".

use dokka_contracts::SystemInfo as _;
use dokka_settings::{AboutButton, GeneralPage};

use super::dock_session::DockSession;

/// Pide el resumen del equipo una vez y pasa lo que ya haya al abrir "Acerca".
pub(crate) fn request_about(session: &mut DockSession) {
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
    if let Some(summary) = session.system_info.summary()
        && session.settings.system_summary() != Some(summary.clone())
    {
        session.settings.set_system_info(Some(summary));
    }
}

/// Pasa lo ya calculado y pide recalcular en el hilo. Solo calcula al
/// abrir el apartado, nunca en bucle: mientras tanto la isla sigue mostrando
/// el último resultado en memoria.
pub(crate) fn request_storage(session: &mut DockSession) {
    if let Some(breakdown) = session.system_info.storage() {
        session.settings.set_storage(Some(breakdown));
    }
    session.system_info.request_storage();
}

/// Si se está en "Almacenamiento" y el hilo ya calculó algo nuevo, pasa el
/// último valor a la isla para que repinte sin pedir en bucle.
pub(crate) fn poll_storage(session: &mut DockSession) {
    if session.settings.general() != GeneralPage::Storage {
        return;
    }
    if let Some(breakdown) = session.system_info.storage()
        && session.settings.storage_breakdown() != Some(breakdown.clone())
    {
        session.settings.set_storage(Some(breakdown));
    }
}

/// Detiene el cálculo al salir del apartado; conserva lo ya calculado.
pub(crate) fn cancel_storage(session: &DockSession) {
    session.system_info.cancel_storage();
}

/// Vacía la papelera con su confirmación y recalcula el apartado.
pub(crate) fn empty_trash(session: &mut DockSession) {
    use dokka_contracts::RecycleBin as _;
    if session.recycle_bin.empty().is_err() {
        return;
    }
    session.system_info.request_storage();
}

/// Abre la página del sistema del botón pulsado. Falla sola.
pub(crate) fn open_about_page(session: &DockSession, button: AboutButton) {
    use dokka_contracts::SystemPage;
    let page = match button {
        AboutButton::DisplaySettings => SystemPage::Display,
        AboutButton::StorageSettings => SystemPage::Storage,
        AboutButton::SystemReport => SystemPage::Report,
    };
    if let Err(error) = session.system_info.open_page(page) {
        eprintln!("Dokka no pudo abrir la página del sistema: {error}");
    }
}

/// Rueda sobre Preferencias: baja la ficha del apartado de General.
pub(crate) fn scroll_settings(session: &mut DockSession, delta: i16) {
    if session.settings.is_open() {
        session.settings.scroll_wheel(delta);
    }
}
