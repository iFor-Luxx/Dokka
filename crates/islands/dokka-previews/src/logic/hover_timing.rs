use dokka_theme::popup as measures;

use crate::{PreviewContent, PreviewRequest, Previews, motion::Reveal};

/// Espera del panel de una app desde cero: evita destellos al pasar de largo.
const PANEL_FIRST_SECONDS: f32 = 0.45;
/// Espera al cambiar entre paneles o para el calendario, como hoy.
const PANEL_SWITCH_SECONDS: f32 = 0.12;
/// Margen para volver antes de que se cierre.
const CLOSE_AFTER_SECONDS: f32 = 0.25;

impl Previews {
    pub fn pointer_moved(&mut self, x: f32, y: f32, request: Option<PreviewRequest>) {
        if self.keeps(x, y) {
            self.pending = None;
            self.closing = None;
            self.reveal.cancel_exit();
            self.hovered = self.open.as_ref().and_then(|open| {
                self.bounds()
                    .and_then(|bounds| super::hovered_at(open, bounds, x, y))
            });
            self.nav_hovered = self.open.as_ref().and_then(|open| {
                self.bounds()
                    .and_then(|bounds| super::nav_hovered_at(open, bounds, x, y))
            });
            let hovered = self.hovered;
            self.update_large(hovered);
            return;
        }
        match request {
            Some(request) if self.is_open_for(&request) => {
                self.pending = None;
                self.closing = None;
                self.reveal.cancel_exit();
                // Sobre el icono no hay mini bajo el cursor.
                self.hovered = None;
                self.nav_hovered = None;
                self.update_large(None);
            }
            Some(request) => {
                self.hovered = None;
                self.nav_hovered = None;
                self.update_large(None);
                let waited = match &self.pending {
                    Some((old, waited)) if old.content == request.content => *waited,
                    _ => 0.0,
                };
                self.pending = Some((request, waited));
            }
            None => self.pointer_left(),
        }
    }

    pub fn pointer_left(&mut self) {
        self.pending = None;
        self.hovered = None;
        self.nav_hovered = None;
        self.update_large(None);
        if self.open.is_some() && self.closing.is_none() && !self.reveal.is_leaving() {
            self.closing = Some(0.0);
        }
    }

    pub fn close(&mut self) {
        self.open = None;
        self.pending = None;
        self.closing = None;
        self.hovered = None;
        self.large = None;
        self.large_wait = None;
        self.nav_hovered = None;
        self.month_offset = 0;
        self.reveal = Reveal::hidden();
    }

    pub fn advance(&mut self, seconds: f32) {
        if let Some(waited) = &mut self.closing {
            *waited += seconds;
            if *waited >= CLOSE_AFTER_SECONDS {
                self.closing = None;
                self.reveal.start_exit();
            }
        }
        let fire = if let Some((request, waited)) = &mut self.pending {
            *waited += seconds;
            *waited >= Self::open_delay(request, self.open.is_some())
        } else {
            false
        };
        if fire {
            let request = self.pending.take().map(|(request, _)| request);
            self.open = None;
            self.closing = None;
            self.month_offset = 0;
            self.nav_hovered = None;
            self.large = None;
            self.large_wait = None;
            self.reveal = Reveal::hidden();
            self.open = request;
        }
        self.advance_large(seconds);
        if self.open.is_some() {
            self.reveal.advance(
                seconds,
                measures::POPUP_ENTER_SECONDS,
                measures::POPUP_EXIT_SECONDS,
            );
            if self.reveal.has_exited() {
                self.open = None;
                self.hovered = None;
                self.large = None;
                self.large_wait = None;
                self.nav_hovered = None;
                self.month_offset = 0;
                self.reveal = Reveal::hidden();
            }
        }
    }

    /// `true` mientras haya una espera o una animación en marcha.
    /// La espera del segundo de la vista grande también mantiene los fotogramas.
    pub fn is_waiting(&self) -> bool {
        self.pending.is_some()
            || self.closing.is_some()
            || self.large_wait.is_some()
            || (self.open.is_some() && self.reveal.is_animating())
    }

    /// `true` si hay panel abierto o esperando a abrirse.
    pub fn has_panel(&self) -> bool {
        self.open.is_some() || self.pending.is_some()
    }

    /// `true` si el cursor sigue sobre el panel o sobre su icono.
    /// La red de seguridad usa el cursor real cuando el aviso se pierde.
    pub fn still_inside(&self, x: f32, y: f32, request: Option<&PreviewRequest>) -> bool {
        if self.contains(x, y) {
            return true;
        }
        let Some(request) = request else {
            return false;
        };
        self.is_open_for(request)
            || self
                .pending
                .as_ref()
                .is_some_and(|(pending, _)| pending.content == request.content)
    }

    fn is_open_for(&self, request: &PreviewRequest) -> bool {
        self.open
            .as_ref()
            .is_some_and(|open| open.content == request.content)
    }

    /// Espera del panel: desde cero tarda más; cambiando o para el
    /// calendario mantiene la de hoy.
    fn open_delay(request: &PreviewRequest, switching: bool) -> f32 {
        match &request.content {
            PreviewContent::Calendar => PANEL_SWITCH_SECONDS,
            PreviewContent::Windows(_) if switching => PANEL_SWITCH_SECONDS,
            PreviewContent::Windows(_) => PANEL_FIRST_SECONDS,
        }
    }

    /// El panel y el hueco que lo separa del dock.
    fn keeps(&self, x: f32, y: f32) -> bool {
        self.area()
            .is_some_and(|a| x >= a.x && x <= a.x + a.width && y >= a.y && y <= a.y + a.height)
    }
}
