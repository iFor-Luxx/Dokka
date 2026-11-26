use dokka_components::preview_card::NowPlaying;

use crate::PreviewRequest;

/// Lo que los paneles recuerdan entre eventos.
#[derive(Debug, Default)]
pub struct Previews {
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
}

impl Previews {
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
