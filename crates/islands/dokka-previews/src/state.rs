use std::sync::Arc;

use dokka_components::calendar_panel::CalendarArrows;
use dokka_components::preview_card::{MediaIcons, NowPlaying};
use dokka_theme::Theme;

use crate::{PreviewRequest, motion::Reveal};

/// Lo que los paneles recuerdan entre eventos.
#[derive(Debug)]
pub struct Previews {
    pub(crate) theme: Arc<Theme>,
    pub(crate) media_icons: MediaIcons,
    /// Flechas del calendario, creadas una vez: el mes cambia sin reconstruirlas.
    pub(crate) calendar_arrows: CalendarArrows,
    pub(crate) open: Option<PreviewRequest>,
    /// Panel que espera a que el cursor se detenga, y cuánto lleva esperando.
    pub(crate) pending: Option<(PreviewRequest, f32)>,
    /// Tiempo desde que el cursor se alejó del panel abierto.
    pub(crate) closing: Option<f32>,
    /// Canción de la app cuyo panel está abierto.
    pub(crate) media: Option<NowPlaying>,
    /// Meses de diferencia con el actual en el calendario.
    pub(crate) month_offset: i32,
    /// Año, mes y día de hoy.
    pub(crate) today: (u16, u16, u16),
    /// Cuánto se ve el panel abierto.
    pub(crate) reveal: Reveal,
}

impl Previews {
    pub fn new(theme: Arc<Theme>) -> Self {
        Self {
            media_icons: MediaIcons::new(&theme),
            calendar_arrows: CalendarArrows::new(&theme),
            theme,
            open: None,
            pending: None,
            closing: None,
            media: None,
            month_offset: 0,
            today: (0, 1, 1),
            reveal: Reveal::hidden(),
        }
    }

    pub fn set_today(&mut self, year: u16, month: u16, day: u16) {
        self.today = (year, month, day);
    }

    /// App cuyas ventanas se están mostrando.
    pub fn open_app(&self) -> Option<&str> {
        self.open.as_ref()?.app.as_deref()
    }

    pub fn set_media(&mut self, media: Option<NowPlaying>) {
        self.media = media;
    }
}
