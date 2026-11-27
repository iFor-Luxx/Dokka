//! Red de la barra: lectura cada pocos `Tick` y ajustes.
//!
//! La isla guarda la conexión y decide el clic; aquí solo se consulta
//! `SystemNetwork` y se abre la página de ajustes.

use std::error::Error;

use dokka_contracts::NetworkStatus;
use dokka_win_control::SystemNetwork;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Conexión en uso; sin perfil hay apagado, sin que sea error.
pub(crate) fn refresh_network(session: &mut DockSession) {
    match SystemNetwork.network() {
        Ok(network) => session.menubar.set_network(network),
        Err(error) => eprintln!("Dokka no pudo leer la red: {error}"),
    }
}

/// Abre los ajustes de red por delante, sin devolver el foco antes.
pub(crate) fn open_network_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    use dokka_contracts::AppLauncher as _;
    use dokka_win_apps::system_place;

    surface.menubar_menu_dismissed();
    session
        .launcher
        .launch(&system_place("Configuración de red", "ms-settings:network"))?;
    Ok(())
}
