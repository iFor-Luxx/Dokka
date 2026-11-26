//! Une la reproducción que informa Windows con el panel abierto.

use dokka_previews::{MediaCommand, NowPlaying};
use dokka_win_media::{MediaAction, MediaSession};

use crate::dock_session::DockSession;

/// Indica al panel si la app que muestra está reproduciendo algo.
pub(crate) fn update_media(session: &mut DockSession) {
    let now = open_session(session).map(|media| NowPlaying {
        is_playing: media.is_playing,
    });
    session.previews.set_media(now);
}

pub(crate) fn send_media(session: &mut DockSession, command: MediaCommand) {
    let Some(source) = open_session(session).map(|media| media.source) else {
        return;
    };
    let action = match command {
        MediaCommand::Previous => MediaAction::Previous,
        MediaCommand::Toggle => MediaAction::Toggle,
        MediaCommand::Next => MediaAction::Next,
    };
    session.media.send(&source, action);
}

/// Sesión multimedia de la app cuyo panel está abierto.
fn open_session(session: &mut DockSession) -> Option<MediaSession> {
    let open = session.previews.open_app()?;
    let app = session
        .state
        .apps()
        .iter()
        .find(|app| app.info.id.as_str() == open)?;
    session
        .media
        .sessions()
        .iter()
        .find(|media| media.belongs_to(&app.info.executable))
        .cloned()
}
