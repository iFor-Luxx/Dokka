use std::sync::Arc;

use dokka_components::background_panel::{BackgroundApp, BackgroundHit};
use dokka_components::control_center_panel::ControlCenterIcons;
use dokka_components::language_panel::LanguageHit;
use dokka_components::language_panel::{LanguageEntry, LanguageIcons};
use dokka_components::menu_panel::{MenuChecks, MenuChevrons};
use dokka_components::music_panel::{MusicPanelIcons, MusicTrack};
use dokka_components::nav_pill::{NavHalf, NavPillIcons};
use dokka_components::network_panel::NetworkHit;
use dokka_components::network_panel::NetworkIcons;
use dokka_components::search_panel::{SearchPanelIcons, SearchPanelItem};
use dokka_components::volume_panel::VolumePanelHit;
use dokka_components::volume_panel::VolumePanelIcons;
use dokka_icons::{TrayIcons, menubar_icon};
use dokka_model::{AudioOutput, MenubarPreferences, NetworkKind, NetworkState, NowPlayingInfo, VolumeState};
use dokka_scene::{Bitmap, Drawing};
use dokka_theme::Theme;

use super::logic::{OpenCalendar, OpenMenu, TrayKind};
use super::painted::MenubarPainted;

/// Lo que la barra recuerda entre eventos: nombre, hora, idioma, menú,
/// calendario, fila derecha con sus datos y resaltados.
#[derive(Debug)]
pub struct Menubar {
    pub(crate) theme: Arc<Theme>,
    pub(crate) icon: Arc<Drawing>,
    pub(crate) chevrons: MenuChevrons,
    pub(crate) checks: MenuChecks,
    pub(crate) nav_pill: NavPillIcons,
    pub(crate) tray_icons: TrayIcons,
    pub(crate) volume_icons: VolumePanelIcons,
    pub(crate) network_icons: NetworkIcons,
    pub(crate) language_icons: LanguageIcons,
    pub(crate) music_icons: MusicPanelIcons,
    pub(crate) control_icons: ControlCenterIcons,
    pub(crate) search_icons: SearchPanelIcons,
    pub(crate) app_name: String,
    pub(crate) clock: String,
    pub(crate) icon_hovered: bool,
    pub(crate) clock_hovered: bool,
    pub(crate) nav_hovered: Option<NavHalf>,
    pub(crate) menu: Option<OpenMenu>,
    pub(crate) calendar: Option<OpenCalendar>,
    pub(crate) today: (u16, u16, u16),
    pub(crate) language_code: String,
    pub(crate) tray: Option<super::logic::OpenTray>,
    pub(crate) tray_hovered: Option<TrayKind>,
    pub(crate) volume: VolumeState,
    pub(crate) volume_hovered: Option<VolumePanelHit>,
    pub(crate) audio_outputs: Vec<AudioOutput>,
    pub(crate) network: NetworkState,
    pub(crate) network_hovered: Option<NetworkHit>,
    pub(crate) languages: Vec<LanguageEntry>,
    pub(crate) language_hovered: Option<LanguageHit>,
    pub(crate) track: Option<MusicTrack>,
    pub(crate) artwork: Option<Arc<Bitmap>>,
    pub(crate) now_playing: NowPlayingInfo,
    pub(crate) background_apps: Vec<BackgroundApp>,
    pub(crate) background_hovered: Option<BackgroundHit>,
    pub(crate) search_query: String,
    pub(crate) search_results: Vec<SearchPanelItem>,
    pub(crate) search_highlighted: Option<usize>,
    pub(crate) search_cursor: bool,
    pub(crate) dragging_slider: bool,
    pub(crate) shown: MenubarPreferences,
}

impl Menubar {
    pub fn new(theme: Arc<Theme>) -> Self {
        let icon = Arc::new(menubar_icon(theme.menubar.text));
        Self {
            volume_icons: VolumePanelIcons::new(&theme),
            network_icons: NetworkIcons::new(&theme),
            language_icons: LanguageIcons::new(&theme),
            music_icons: MusicPanelIcons::new(&theme),
            control_icons: ControlCenterIcons::new(&theme),
            search_icons: SearchPanelIcons::new(&theme),
            chevrons: MenuChevrons::new(&theme),
            checks: MenuChecks::new(&theme),
            nav_pill: NavPillIcons::new(&theme),
            tray_icons: TrayIcons::new(&theme),
            theme,
            icon,
            app_name: String::new(),
            clock: String::new(),
            icon_hovered: false,
            clock_hovered: false,
            nav_hovered: None,
            menu: None,
            calendar: None,
            today: (0, 1, 1),
            language_code: "ES".to_owned(),
            tray: None,
            tray_hovered: None,
            volume: VolumeState {
                level: 0.0,
                is_muted: false,
            },
            volume_hovered: None,
            audio_outputs: Vec::new(),
            network: NetworkState {
                kind: NetworkKind::Offline,
                name: String::new(),
                has_internet: false,
            },
            network_hovered: None,
            languages: Vec::new(),
            language_hovered: None,
            track: None,
            artwork: None,
            now_playing: NowPlayingInfo {
                title: String::new(),
                artist: String::new(),
                is_playing: false,
            },
            background_apps: Vec::new(),
            background_hovered: None,
            search_query: String::new(),
            search_results: Vec::new(),
            search_highlighted: None,
            search_cursor: false,
            dragging_slider: false,
            shown: MenubarPreferences::default(),
        }
    }

    pub fn set_app(&mut self, name: String) {
        self.app_name = name;
    }

    pub fn set_clock(&mut self, text: String) {
        self.clock = text;
    }

    pub fn set_today(&mut self, year: u16, month: u16, day: u16) {
        self.today = (year, month, day);
    }

    /// Código corto del idioma ("ES"): el shell lo rellena con
    /// `KeyboardLanguage.code` cada pocos `Tick`.
    pub fn set_language_code(&mut self, code: String) {
        self.language_code = code;
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn clock_text(&self) -> &str {
        &self.clock
    }

    pub fn is_menu_open(&self) -> bool {
        self.menu.is_some()
    }

    pub fn is_calendar_open(&self) -> bool {
        self.calendar.is_some()
    }

    pub fn is_tray_open(&self) -> bool {
        self.tray.is_some()
    }

    pub fn tray_kind(&self) -> Option<TrayKind> {
        self.tray.map(|open| open.kind)
    }

    pub fn is_icon_hovered(&self) -> bool {
        self.icon_hovered
    }

    pub(crate) fn calendar_month_offset(&self) -> i32 {
        self.calendar.map(|open| open.month_offset).unwrap_or(0)
    }

    pub(crate) fn calendar_weeks_seen(&self) -> u16 {
        self.calendar.map(|open| open.calendar_weeks).unwrap_or(0)
    }

    pub fn highlighted(&self) -> Option<usize> {
        self.menu.as_ref().and_then(|menu| menu.highlighted)
    }

    /// Huella de lo dibujado: el shell solo repinta si cambia.
    pub fn painted(&self) -> MenubarPainted {
        MenubarPainted {
            app: self.app_name.clone(),
            clock: self.clock.clone(),
            today: self.today,
            language: self.language_code.clone(),
            shown: self.shown,
            menu_open: self.menu.is_some(),
            icon_hovered: self.icon_hovered,
            highlighted: self.highlighted(),
            calendar_open: self.calendar.is_some(),
            month_offset: self.calendar_month_offset(),
            weeks_seen: self.calendar_weeks_seen(),
            nav_hovered: self.nav_hovered,
            clock_hovered: self.clock_hovered,
            tray_open: self.tray_kind(),
            tray_hovered: self.tray_hovered,
            volume_level: self.volume.level,
            volume_muted: self.volume.is_muted,
            volume_hovered: self.volume_hovered,
            audio_outputs: self.audio_outputs.clone(),
            network_kind: self.network.kind,
            network_name: self.network.name.clone(),
            network_online: self.network.has_internet,
            network_hovered: self.network_hovered,
            languages: self.languages.clone(),
            language_hovered: self.language_hovered,
            track: self.track.clone(),
            artwork: self.artwork.clone(),
            background_ids: self.background_apps.iter().map(|app| app.id).collect(),
            background_names: self
                .background_apps
                .iter()
                .map(|app| app.name.clone())
                .collect(),
            background_icons: self
                .background_apps
                .iter()
                .map(|app| app.icon.clone())
                .collect(),
            background_hovered: self.background_hovered,
            search_query: self.search_query.clone(),
            search_results: self
                .search_results
                .iter()
                .map(|item| item.name.clone())
                .collect(),
            search_highlighted: self.search_highlighted,
            search_cursor: self.search_cursor,
        }
    }

    pub fn close_menu(&mut self) {
        self.menu = None;
        self.icon_hovered = false;
    }

    pub fn close_calendar(&mut self) {
        self.calendar = None;
        self.nav_hovered = None;
        self.clock_hovered = false;
    }

    pub fn close_tray(&mut self) {
        self.tray = None;
        self.tray_hovered = None;
        self.volume_hovered = None;
        self.network_hovered = None;
        self.language_hovered = None;
        self.background_hovered = None;
        self.dragging_slider = false;
    }
}
