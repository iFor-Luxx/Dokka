//! Apps en segundo plano de la barra: arranca la bandeja y la relee.
//!
//! La isla guarda la lista y decide el clic; aquí solo se mantiene el
//! `TrayHost` como `Option` en la sesión. Hoy `start()` devuelve error:
//! se registra una vez y la parte queda apagada, sin afectar al resto.

use dokka_components::background_panel::BackgroundApp;
use dokka_contracts::SystemTray;
use dokka_theme::background as measures;
use dokka_win_tray::TrayHost;

use crate::dock_session::DockSession;

/// Arranca o suelta la bandeja según Preferencias. Un error se avisa una
/// sola vez y la parte queda apagada.
pub(crate) fn apply_background_apps(session: &mut DockSession, enabled: bool) {
    if !enabled {
        session.background_host = None;
        session.background_revision = 0;
        session.menubar.set_background_apps(Vec::new());
        return;
    }
    if session.background_host.is_some() {
        return;
    }
    match TrayHost::start() {
        Ok(host) => {
            session.background_host = Some(host);
            session.background_revision = 0;
        }
        Err(error) => {
            session.background_host = None;
            if !session.background_error_logged {
                eprintln!("Dokka no pudo escuchar la bandeja: {error}");
                session.background_error_logged = true;
            }
        }
    }
}

/// Pasa la lista a la isla solo si cambió la revisión del host.
pub(crate) fn refresh_background_apps(session: &mut DockSession) {
    let Some(host) = session.background_host.as_ref() else {
        return;
    };
    let revision = host.revision();
    if revision == session.background_revision {
        return;
    }
    session.background_revision = revision;
    let mut apps = Vec::new();
    for item in host.items() {
        let icon = host.icon(item.id);
        apps.push(BackgroundApp::new(item.id, display_name(&item.title), icon));
    }
    session.menubar.set_background_apps(apps);
}

/// Activa la app de la bandeja por delante, sin devolver el foco antes.
pub(crate) fn activate_background_app(session: &mut DockSession, id: dokka_model::TrayItemId) {
    let Some(host) = session.background_host.as_ref() else {
        return;
    };
    if let Err(error) = host.activate(id) {
        eprintln!("Dokka no pudo activar la app en segundo plano: {error}");
    }
}

/// Abre el menú propio de la app de la bandeja, sin devolver el foco antes.
pub(crate) fn open_background_menu(session: &mut DockSession, id: dokka_model::TrayItemId) {
    let Some(host) = session.background_host.as_ref() else {
        return;
    };
    if let Err(error) = host.open_menu(id) {
        eprintln!("Dokka no pudo abrir el menú en segundo plano: {error}");
    }
}

/// Primera línea recortada con "…"; si está vacío, "Aplicación".
fn display_name(title: &str) -> String {
    let first = title.lines().next().unwrap_or("").trim();
    if first.is_empty() {
        return measures::FALLBACK_NAME.to_owned();
    }
    if first.chars().count() <= measures::MAX_NAME_CHARACTERS {
        return first.to_owned();
    }
    let kept: String = first
        .chars()
        .take(measures::MAX_NAME_CHARACTERS - 1)
        .collect();
    format!("{}…", kept.trim_end())
}
