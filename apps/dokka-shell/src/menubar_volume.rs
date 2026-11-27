//! Volumen de la barra: lectura cada `Tick` y ejecución de sus acciones.
//!
//! La isla guarda el nivel y decide el gesto; aquí solo se lee la
//! plataforma con `SystemVolume` y se aplica lo que devuelve.

use std::error::Error;

use dokka_contracts::VolumeControl;
use dokka_win_media::SystemVolume;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Nivel y silencio actuales; sin dispositivo, se conserva lo último.
pub(crate) fn refresh_volume(session: &mut DockSession) {
    match SystemVolume.volume() {
        Ok(state) => session.menubar.set_volume(state.level, state.is_muted),
        Err(error) => eprintln!("Dokka no pudo leer el volumen: {error}"),
    }
}

/// Fija el nivel elegido en el deslizador y lo relee para pintar lo real.
pub(crate) fn set_level(session: &mut DockSession, level: f32) -> Result<(), Box<dyn Error>> {
    if let Err(error) = SystemVolume.set_level(level) {
        eprintln!("Dokka no pudo cambiar el volumen: {error}");
    }
    refresh_volume(session);
    Ok(())
}

/// Cambia el silencio según lo que muestra la propia barra.
pub(crate) fn toggle_mute(session: &mut DockSession) -> Result<(), Box<dyn Error>> {
    let muted = !session.menubar.painted().volume_muted;
    if let Err(error) = SystemVolume.set_muted(muted) {
        eprintln!("Dokka no pudo cambiar el silencio: {error}");
    }
    refresh_volume(session);
    Ok(())
}

/// Abre los ajustes de sonido por delante, sin devolver el foco antes.
pub(crate) fn open_sound_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    use dokka_contracts::AppLauncher as _;
    use dokka_win_apps::system_place;

    surface.menubar_menu_dismissed();
    session.launcher.launch(&system_place(
        "Configuración de sonido",
        "ms-settings:sound",
    ))?;
    Ok(())
}
