//! Huella de la barra ya dibujada: el shell solo repinta si cambia.
//!
//! Vive aquí para no exponer los tipos del componente. La carátula se
//! compara por identidad (`Arc::ptr_eq`): el monitor reutiliza el mismo
//! `Arc` mientras no cambia la canción, así que comparar su contenido
//! (64 KiB por `Tick`) sería trabajo inútil.

use std::sync::Arc;

use dokka_components::language_panel::LanguageEntry;
use dokka_components::language_panel::LanguageHit;
use dokka_components::music_panel::MusicTrack;
use dokka_components::network_panel::NetworkHit;
use dokka_components::volume_panel::VolumePanelHit;
use dokka_model::NetworkKind;
use dokka_model::MenubarPreferences;
use dokka_scene::Bitmap;

use super::logic::TrayKind;
use dokka_components::nav_pill::NavHalf;

/// Huella de la barra ya dibujada: app, hora, hoy, idioma, aperturas,
/// resaltados, mes, píldora y los datos que muestra cada panel.
#[derive(Debug, Clone)]
pub struct MenubarPainted {
    pub app: String,
    pub clock: String,
    pub today: (u16, u16, u16),
    pub language: String,
    pub shown: MenubarPreferences,
    pub menu_open: bool,
    pub icon_hovered: bool,
    pub highlighted: Option<usize>,
    pub calendar_open: bool,
    pub month_offset: i32,
    pub weeks_seen: u16,
    pub nav_hovered: Option<NavHalf>,
    pub clock_hovered: bool,
    pub tray_open: Option<TrayKind>,
    pub tray_hovered: Option<TrayKind>,
    pub volume_level: f32,
    pub volume_muted: bool,
    pub volume_hovered: Option<VolumePanelHit>,
    pub network_kind: NetworkKind,
    pub network_name: String,
    pub network_online: bool,
    pub network_hovered: Option<NetworkHit>,
    pub languages: Vec<LanguageEntry>,
    pub language_hovered: Option<LanguageHit>,
    pub track: Option<MusicTrack>,
    pub artwork: Option<Arc<Bitmap>>,
    pub search_query: String,
    pub search_results: Vec<String>,
    pub search_highlighted: Option<usize>,
    pub search_cursor: bool,
}

impl PartialEq for MenubarPainted {
    fn eq(&self, other: &Self) -> bool {
        self.app == other.app
            && self.clock == other.clock
            && self.today == other.today
            && self.language == other.language
            && self.shown == other.shown
            && self.menu_open == other.menu_open
            && self.icon_hovered == other.icon_hovered
            && self.highlighted == other.highlighted
            && self.calendar_open == other.calendar_open
            && self.month_offset == other.month_offset
            && self.weeks_seen == other.weeks_seen
            && self.nav_hovered == other.nav_hovered
            && self.clock_hovered == other.clock_hovered
            && self.tray_open == other.tray_open
            && self.tray_hovered == other.tray_hovered
            && self.volume_level == other.volume_level
            && self.volume_muted == other.volume_muted
            && self.volume_hovered == other.volume_hovered
            && self.network_kind == other.network_kind
            && self.network_name == other.network_name
            && self.network_online == other.network_online
            && self.network_hovered == other.network_hovered
            && self.languages == other.languages
            && self.language_hovered == other.language_hovered
            && self.track == other.track
            && same_artwork(&self.artwork, &other.artwork)
            && self.search_query == other.search_query
            && self.search_results == other.search_results
            && self.search_highlighted == other.search_highlighted
            && self.search_cursor == other.search_cursor
    }
}

/// `true` si es la misma carátula: el mismo objeto en memoria.
/// Un `Arc` distinto, aunque con los mismos píxeles, pide repintar.
fn same_artwork(first: &Option<Arc<Bitmap>>, second: &Option<Arc<Bitmap>>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => Arc::ptr_eq(first, second),
        (None, None) => true,
        _ => false,
    }
}
