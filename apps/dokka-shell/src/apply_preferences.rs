//! Guarda y aplica en el acto lo ajustado en Preferencias.

use dokka_model::Preferences;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

pub(crate) fn apply_preferences(
    session: &mut DockSession,
    _surface: &mut Surface,
    preferences: Preferences,
) {
    if session.preferences != preferences {
        session.preferences = preferences;
    }
}

pub(crate) fn set_taskbar_auto(session: &mut DockSession, _surface: &mut Surface, auto: bool) {
    session.set_taskbar_auto(auto);
}
