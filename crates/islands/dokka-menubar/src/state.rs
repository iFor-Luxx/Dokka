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
use dokka_icons::{
    TrayIcons, control_center_calculator_icon, control_center_clock_icon,
    control_center_desktop_icon, control_center_lock_icon, control_center_play_icon,
    control_center_screen_clip_icon, control_center_sun_icon, control_center_task_view_icon,
    control_center_volume_icon, control_center_wifi_icon, menubar_icon,
};
use dokka_model::{
    AudioOutput, MenubarPreferences, NetworkKind, NetworkState, NowPlayingInfo, VolumeState,
};
use dokka_scene::{Bitmap, Drawing};
use dokka_theme::Theme;

use super::logic::{OpenCalendar, OpenMenu, TrayKind};

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
    /// `true` si lo arrastrado es el brillo (si no, el volumen del centro).
    pub(crate) dragging_brightness: bool,
    /// `Tick` seguidos sin uso con el cursor fuera del centro.
    pub(crate) control_idle_ticks: u32,
    /// `true` si el cursor está sobre el panel del centro.
    pub(crate) control_cursor_inside: bool,
    /// Brillo DDC de 0 a 1; `None` sin DDC: sin tarjeta de Pantalla.
    pub(crate) brightness: Option<f32>,
    /// `true` en modo editar: cada pieza lleva su "−" y no hace su acción;
    /// la burbuja desaparece. Vive aquí; el componente solo la recibe.
    pub(crate) control_editing: bool,
    /// Galería del modo editar: insignias blancas de cada categoría (una vez,
    /// como el resto de dibujos), categoría elegida (0: todas), búsqueda y
    /// desplazamiento de la columna y de las secciones. El componente solo
    /// la recibe.
    pub(crate) gallery_badges: Vec<Arc<dokka_scene::Drawing>>,
    pub(crate) gallery_category: usize,
    pub(crate) gallery_query: String,
    pub(crate) gallery_list_scroll: f32,
    pub(crate) gallery_grid_scroll: f32,
    pub(crate) gallery_cursor: bool,
    pub(crate) shown: MenubarPreferences,
    /// Salida del centro por piezas en curso, si la hay: destino y tiempo
    /// para el cierre real. Mientras existe el panel sigue abierto y la
    /// plataforma lo anima sin repintar.
    pub(crate) control_dismiss: Option<super::logic::ClosingCenter>,
    /// La semana del calendario empieza en lunes; si no, en domingo.
    pub(crate) week_monday: bool,
}

impl Menubar {
    pub fn new(theme: Arc<Theme>) -> Self {
        let icon = Arc::new(menubar_icon(theme.menubar.text));
        // Insignias blancas de la galería, en el orden de sus categorías.
        let badge = theme.tray.circle_active_icon;
        let gallery_badges = [
            control_center_task_view_icon(badge),
            control_center_play_icon(badge),
            control_center_screen_clip_icon(badge),
            control_center_wifi_icon(badge),
            control_center_desktop_icon(badge),
            control_center_lock_icon(badge),
            control_center_sun_icon(badge),
            control_center_clock_icon(badge),
            control_center_volume_icon(badge),
            control_center_calculator_icon(badge),
        ]
        .into_iter()
        .map(Arc::new)
        .collect();
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
            dragging_brightness: false,
            control_idle_ticks: 0,
            control_cursor_inside: false,
            brightness: None,
            control_editing: false,
            gallery_badges,
            gallery_category: 0,
            gallery_query: String::new(),
            gallery_list_scroll: 0.0,
            gallery_grid_scroll: 0.0,
            gallery_cursor: false,
            shown: MenubarPreferences::default(),
            control_dismiss: None,
            week_monday: false,
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

    /// Guarda si la semana del calendario empieza en lunes (Dokka).
    pub fn set_week_start(&mut self, monday_first: bool) {
        self.week_monday = monday_first;
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

    pub(crate) fn calendar_month_offset(&self) -> i32 {
        self.calendar.map(|open| open.month_offset).unwrap_or(0)
    }

    pub fn highlighted(&self) -> Option<usize> {
        self.menu.as_ref().and_then(|menu| menu.highlighted)
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
        self.control_dismiss = None;
        self.tray_hovered = None;
        self.volume_hovered = None;
        self.network_hovered = None;
        self.language_hovered = None;
        self.background_hovered = None;
        self.dragging_slider = false;
        self.dragging_brightness = false;
        self.control_idle_ticks = 0;
        self.control_cursor_inside = false;
        // Cerrar sale del modo editar: al reabrir no hay "−" ni burbuja a medias.
        self.control_editing = false;
    }
}
