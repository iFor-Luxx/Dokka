use dokka_model::DockApp;

use crate::Dock;
use crate::logic::place_dock;
use crate::view::style;

/// Margen alrededor de la zona activa antes de dar la lupa por terminada.
const LEAVE_MARGIN: f32 = 12.0;

impl Dock {
    pub fn pointer_moved(&mut self, x: f32, y: f32, apps: &[DockApp]) {
        let bar = place_dock(self, apps).bar;
        // Ya dentro, la zona se amplía para alcanzar los iconos agrandados.
        let (reach, side) = if self.is_hovered {
            let rise = style::SLOT * (style::MAXIMUM_SCALE - 1.0);
            (rise + LEAVE_MARGIN, LEAVE_MARGIN)
        } else {
            (0.0, 0.0)
        };
        let inside = y >= bar.y - reach && x >= bar.x - side && x <= bar.x + bar.width + side;

        self.is_hovered = inside;
        if inside {
            self.cursor_x = x;
        }
        self.hover.set_target(if inside { 1.0 } else { 0.0 });
    }

    pub fn pointer_left(&mut self) {
        self.is_hovered = false;
        self.hover.set_target(0.0);
    }

    /// Avanza las animaciones; indica si queda algo moviéndose.
    pub fn advance(&mut self, seconds: f32) -> bool {
        self.hover.advance(seconds);
        self.bounces.retain(|_, bounce| bounce.advance(seconds));
        self.is_animating()
    }

    pub fn is_animating(&self) -> bool {
        !self.hover.is_settled() || !self.bounces.is_empty()
    }

    /// Los iconos sobresalen de la barra mientras hay lupa o saltos.
    pub(crate) fn needs_headroom(&self) -> bool {
        self.is_hovered || self.is_animating() || self.hover.value() > 0.0
    }
}
