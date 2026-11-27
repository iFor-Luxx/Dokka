//! Música de la barra: canción actual, carátula y botones.
//!
//! La isla guarda lo que suena y decide el botón; aquí se leen
//! `MediaMonitor` (canción y carátula) cada `Tick` y se envía la orden
//! con su fuente. La pausa adelanta el icono al instante dentro del
//! propio monitor.

use dokka_win_media::MediaAction;

use crate::dock_session::DockSession;

/// Lo que el sistema marca como sesión actual, sin esperar.
/// Guarda su fuente para enviarle las órdenes y pasa su carátula a la isla.
pub(crate) fn refresh_music(session: &mut DockSession) {
    let current = session.media.current();
    let artwork = session.media.artwork();
    session.menubar_music_source = current.as_ref().and_then(|info| {
        session
            .media
            .sessions()
            .iter()
            .find(|media| media.title == info.title && media.artist == info.artist)
            .map(|media| media.source.clone())
    });
    session.menubar.set_now_playing(current, artwork);
}

/// Envía la orden a la fuente guardada; sin fuente no hay nada que tocar.
pub(crate) fn send_music(session: &mut DockSession, action: MediaAction) {
    let Some(source) = session.menubar_music_source.clone() else {
        return;
    };
    session.media.send(&source, action);
    // La pausa ya adelantó el icono; se relee para pintar lo real.
    let current = session.media.current();
    let artwork = session.media.artwork();
    session.menubar.set_now_playing(current, artwork);
}
