//! Desinstalación desde Preferencias: estado y apertura.
//!
//! La isla solo muestra si responde y emite `Uninstall`; el shell conecta con
//! `InstalledDokka`. Si algo falla, se registra y Dokka sigue igual: no se
//! cierra, lo hace el desinstalador.

use dokka_contracts::UninstallControl as _;

use super::dock_session::DockSession;

/// Pasa a la isla si el botón responde (Dokka instalado).
pub(crate) fn sync_uninstall(session: &mut DockSession) {
    session
        .settings
        .set_uninstall_available(session.uninstall.can_uninstall());
}

/// Pide el estado al entrar en "Actualización de Dokka".
pub(crate) fn request_uninstall(session: &mut DockSession) {
    sync_uninstall(session);
}

/// Abre `Desinstalar.exe --uninstall` en su hilo. Falla sola.
pub(crate) fn open_uninstaller(session: &DockSession) {
    if let Err(error) = session.uninstall.open_uninstaller() {
        eprintln!("Dokka no pudo abrir el desinstalador: {error}");
    }
}
