use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as Session,
    GlobalSystemMediaTransportControlsSessionManager as Manager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as Status,
};
use windows::core::Result;

use crate::{MediaAction, MediaSession};

pub(crate) fn read_sessions(manager: &Manager) -> Vec<MediaSession> {
    // Una sesión que falle (por ejemplo, al cerrarse) no oculta las demás.
    each_session(manager)
        .into_iter()
        .filter_map(|session| describe(&session).ok())
        .collect()
}

pub(crate) fn read_current_source(manager: &Manager) -> Option<String> {
    // Sin sesión actual (nada sonando) no hay error: simplemente no hay actual.
    let session = manager.GetCurrentSession().ok()?;
    session.SourceAppUserModelId().ok().map(|id| id.to_string())
}

pub(crate) fn perform(manager: &Manager, source: &str, action: MediaAction) -> Result<()> {
    for session in each_session(manager) {
        if session.SourceAppUserModelId()? == source {
            let request = match action {
                MediaAction::Previous => session.TrySkipPreviousAsync()?,
                MediaAction::Toggle => session.TryTogglePlayPauseAsync()?,
                MediaAction::Next => session.TrySkipNextAsync()?,
            };
            request.join()?;
        }
    }
    Ok(())
}

fn each_session(manager: &Manager) -> Vec<Session> {
    let Ok(sessions) = manager.GetSessions() else {
        return Vec::new();
    };
    let count = sessions.Size().unwrap_or(0);
    (0..count)
        .filter_map(|index| sessions.GetAt(index).ok())
        .collect()
}

fn describe(session: &Session) -> Result<MediaSession> {
    let properties = session.TryGetMediaPropertiesAsync()?.join()?;
    let status = session.GetPlaybackInfo()?.PlaybackStatus()?;
    Ok(MediaSession {
        source: session.SourceAppUserModelId()?.to_string(),
        title: properties.Title()?.to_string(),
        artist: properties.Artist()?.to_string(),
        is_playing: status == Status::Playing,
    })
}
