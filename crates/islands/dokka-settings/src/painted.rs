use dokka_components::nav_pill::NavHalf;

use super::state::Settings;

/// Todo lo que cambia lo que se ve: si no cambió, el shell no repinta.
#[derive(Debug, Clone, PartialEq)]
pub struct PaintedSettings {
    pub selected: usize,
    pub hovered: Option<usize>,
    pub nav_hovered: Option<NavHalf>,
    pub traffic_hovered: bool,
    pub window_focused: bool,
    pub query: String,
    pub search_focused: bool,
    pub can_previous: bool,
    pub can_next: bool,
}

impl PaintedSettings {
    /// Foto de lo dibujado, con la API pública de la isla.
    pub(crate) fn of(settings: &Settings) -> Self {
        Self {
            selected: settings.selected(),
            hovered: settings.hovered(),
            nav_hovered: settings.nav_hovered(),
            traffic_hovered: settings.traffic_hovered(),
            window_focused: settings.window_focused(),
            query: settings.query().to_owned(),
            search_focused: settings.search_focused(),
            can_previous: settings.can_previous(),
            can_next: settings.can_next(),
        }
    }
}
