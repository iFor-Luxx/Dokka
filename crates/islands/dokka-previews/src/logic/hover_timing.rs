use dokka_theme::popup as measures;

use crate::{PreviewRequest, Previews, motion::Reveal};

/// Todos los paneles se muestran casi al instante; la espera mínima evita destellos al pasar de largo.
const OPEN_AFTER_SECONDS: f32 = 0.12;
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
            return;
        }
        match request {
            Some(request) if self.is_open_for(&request) => {
                self.pending = None;
                self.closing = None;
                self.reveal.cancel_exit();
            }
            Some(request) => {
                self.hovered = None;
                self.nav_hovered = None;
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
        if self.open.is_some() && self.closing.is_none() && !self.reveal.is_leaving() {
            self.closing = Some(0.0);
        }
    }

    pub fn close(&mut self) {
        self.open = None;
        self.pending = None;
        self.closing = None;
        self.hovered = None;
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
        if let Some((_, waited)) = &mut self.pending {
            *waited += seconds;
            if *waited >= OPEN_AFTER_SECONDS {
                let request = self.pending.take().map(|(request, _)| request);
                self.open = None;
                self.closing = None;
                self.month_offset = 0;
                self.nav_hovered = None;
                self.reveal = Reveal::hidden();
                self.open = request;
            }
        }
        if self.open.is_some() {
            self.reveal.advance(
                seconds,
                measures::POPUP_ENTER_SECONDS,
                measures::POPUP_EXIT_SECONDS,
            );
            if self.reveal.has_exited() {
                self.open = None;
                self.hovered = None;
                self.nav_hovered = None;
                self.month_offset = 0;
                self.reveal = Reveal::hidden();
            }
        }
    }

    /// `true` mientras haya una espera o una animación en marcha.
    pub fn is_waiting(&self) -> bool {
        self.pending.is_some()
            || self.closing.is_some()
            || (self.open.is_some() && self.reveal.is_animating())
    }

    fn is_open_for(&self, request: &PreviewRequest) -> bool {
        self.open
            .as_ref()
            .is_some_and(|open| open.content == request.content)
    }

    /// El panel y el hueco que lo separa del dock.
    fn keeps(&self, x: f32, y: f32) -> bool {
        self.area()
            .is_some_and(|a| x >= a.x && x <= a.x + a.width && y >= a.y && y <= a.y + a.height)
    }
}
