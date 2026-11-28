use std::sync::Arc;

use dokka_components::nav_pill::{NavHalf, NavPillIcons};
use dokka_components::preview_card::{MediaIcons, NowPlaying, WindowButtonIcons};
use dokka_theme::Theme;

use crate::{PreviewRequest, motion::Reveal};

/// Lo que los paneles recuerdan entre eventos.
#[derive(Debug)]
pub struct Previews {
    pub(crate) theme: Arc<Theme>,
    pub(crate) media_icons: MediaIcons,
    pub(crate) window_buttons: WindowButtonIcons,
    /// Chevrones de la píldora, creados una vez: el mes cambia sin reconstruirlos.
    pub(crate) nav_pill: NavPillIcons,
    /// Mitad de la píldora bajo el cursor, si hay alguna.
    pub(crate) nav_hovered: Option<NavHalf>,
    pub(crate) open: Option<PreviewRequest>,
    /// Panel que espera a que el cursor se detenga, y cuánto lleva esperando.
    pub(crate) pending: Option<(PreviewRequest, f32)>,
    /// Tiempo desde que el cursor se alejó del panel abierto.
    pub(crate) closing: Option<f32>,
    /// Canción de la app cuyo panel está abierto.
    pub(crate) media: Option<NowPlaying>,
    /// Ventanas fijadas (siempre encima) del panel abierto, leídas de Windows.
    pub(crate) pinned: Vec<u64>,
    /// Ventana bajo el cursor, si está sobre una mini.
    pub(crate) hovered: Option<u64>,
    /// Ventana con vista grande visible, tras esperar su segundo.
    pub(crate) large: Option<u64>,
    /// Ventana esperando su segundo para mostrarse grande, y cuánto lleva.
    pub(crate) large_wait: Option<(u64, f32)>,
    /// Meses de diferencia con el actual en el calendario.
    pub(crate) month_offset: i32,
    /// Año, mes y día de hoy.
    pub(crate) today: (u16, u16, u16),
    /// La semana del calendario empieza en lunes; si no, en domingo.
    pub(crate) week_monday: bool,
    /// Cuánto se ve el panel abierto.
    pub(crate) reveal: Reveal,
}

impl Previews {
    pub fn new(theme: Arc<Theme>) -> Self {
        Self {
            media_icons: MediaIcons::new(&theme),
            window_buttons: WindowButtonIcons::new(&theme),
            nav_pill: NavPillIcons::new(&theme),
            theme,
            open: None,
            pending: None,
            closing: None,
            media: None,
            pinned: Vec::new(),
            hovered: None,
            large: None,
            large_wait: None,
            nav_hovered: None,
            month_offset: 0,
            today: (0, 1, 1),
            week_monday: false,
            reveal: Reveal::hidden(),
        }
    }

    pub fn set_today(&mut self, year: u16, month: u16, day: u16) {
        self.today = (year, month, day);
    }

    /// Guarda si la semana del calendario empieza en lunes (Dokka).
    pub fn set_week_start(&mut self, monday_first: bool) {
        self.week_monday = monday_first;
    }

    /// App cuyas ventanas se están mostrando.
    pub fn open_app(&self) -> Option<&str> {
        self.open.as_ref()?.app.as_deref()
    }

    pub fn set_media(&mut self, media: Option<NowPlaying>) {
        self.media = media;
    }

    /// Ventanas fijadas del panel abierto, leídas de la propia ventana.
    pub fn set_pinned(&mut self, pinned: Vec<u64>) {
        self.pinned = pinned;
    }

    /// Ventanas del panel abierto, si son ventanas.
    pub fn open_windows(&self) -> Vec<u64> {
        match &self.open {
            Some(request) => match &request.content {
                crate::PreviewContent::Windows(windows) => {
                    windows.iter().map(|item| item.window).collect()
                }
                crate::PreviewContent::Calendar => Vec::new(),
            },
            None => Vec::new(),
        }
    }
}
