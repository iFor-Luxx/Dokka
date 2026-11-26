use crate::{PreviewContent, PreviewRequest, Previews};

/// Las ventanas se muestran casi al instante; la espera mínima evita destellos al pasar de largo.
const WINDOWS_AFTER_SECONDS: f32 = 0.12;
/// El calendario espera más: primero se ve la etiqueta con la fecha.
const CALENDAR_AFTER_SECONDS: f32 = 0.4;
/// Margen para volver antes de que se cierre.
const CLOSE_AFTER_SECONDS: f32 = 0.25;

impl Previews {
    pub fn pointer_moved(&mut self, x: f32, y: f32, request: Option<PreviewRequest>) {
        if self.keeps(x, y) {
            self.pending = None;
            self.closing = None;
            return;
        }
        match request {
            Some(request) if self.is_open_for(&request) => {
                self.pending = None;
                self.closing = None;
            }
            Some(request) => {
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
        if self.open.is_some() && self.closing.is_none() {
            self.closing = Some(0.0);
        }
    }

    pub fn close(&mut self) {
        self.open = None;
        self.pending = None;
        self.closing = None;
        self.month_offset = 0;
    }

    pub fn advance(&mut self, seconds: f32) {
        if let Some(waited) = &mut self.closing {
            *waited += seconds;
            if *waited >= CLOSE_AFTER_SECONDS {
                self.close();
            }
        }
        if let Some((request, waited)) = &mut self.pending {
            *waited += seconds;
            let delay = match request.content {
                PreviewContent::Windows(_) => WINDOWS_AFTER_SECONDS,
                PreviewContent::Calendar => CALENDAR_AFTER_SECONDS,
            };
            if *waited >= delay {
                let request = self.pending.take().map(|(request, _)| request);
                self.close();
                self.open = request;
            }
        }
    }

    /// `true` mientras haya una espera en marcha.
    pub fn is_waiting(&self) -> bool {
        self.pending.is_some() || self.closing.is_some()
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
