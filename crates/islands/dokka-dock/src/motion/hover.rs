use dokka_model::{DockApp, DockPosition};

use crate::Dock;
use crate::logic::{is_vertical, place_dock};
use crate::view::dock_style as style;

/// Margen alrededor de la zona activa antes de dar la lupa por terminada.
const LEAVE_MARGIN: f32 = 12.0;

impl Dock {
    pub fn pointer_moved(&mut self, x: f32, y: f32, apps: &[DockApp]) {
        let bar = place_dock(self, apps).bar;
        // Ya dentro, la zona se amplía para alcanzar los iconos agrandados.
        let (reach, side) = if self.is_hovered {
            let rise = style::slot(self) * (style::maximum_scale(self) - 1.0);
            (rise + LEAVE_MARGIN, LEAVE_MARGIN)
        } else {
            (0.0, 0.0)
        };
        let inside = if is_vertical(self) {
            let (edge_low, edge_high) = match self.preferences.position {
                DockPosition::Left => (bar.x, bar.x + bar.width + reach),
                _ => (bar.x - reach, bar.x + bar.width),
            };
            y >= bar.y - side && y <= bar.y + bar.height + side && x >= edge_low && x <= edge_high
        } else {
            y >= bar.y - reach && x >= bar.x - side && x <= bar.x + bar.width + side
        };

        self.is_hovered = inside;
        self.cursor_inside = true;
        self.hide.set_cursor_inside(true);
        if inside {
            self.cursor_x = x;
            self.cursor_y = y;
        }
        let target = if inside && self.preferences.magnification {
            1.0
        } else {
            0.0
        };
        self.hover.set_target(target);
    }

    pub fn pointer_left(&mut self) {
        self.is_hovered = false;
        self.cursor_inside = false;
        self.hide.set_cursor_inside(false);
        self.hover.set_target(0.0);
    }

    /// Indica si el cursor sigue sobre el dock para el auto-ocultado.
    pub fn note_cursor_inside(&mut self, inside: bool) {
        self.cursor_inside = inside;
        self.hide.set_cursor_inside(inside);
    }

    /// Si hay menú, vista previa o Launchpad no debe esconderse.
    pub fn set_hide_busy(&mut self, busy: bool) {
        self.hide.set_busy(busy);
    }

    pub fn hide_eased(&self) -> f32 {
        self.hide.eased()
    }

    pub fn is_hide_animating(&self) -> bool {
        self.hide.is_animating()
    }

    /// Avanza las animaciones; indica si queda algo moviéndose.
    pub fn advance(&mut self, seconds: f32) -> bool {
        self.hover.advance(seconds);
        self.bounces.retain(|_, bounce| bounce.advance(seconds));
        self.hide.advance(seconds);
        self.is_animating()
    }

    pub fn is_animating(&self) -> bool {
        !self.hover.is_settled() || !self.bounces.is_empty() || self.hide.is_animating()
    }

    /// Los iconos sobresalen de la barra mientras hay lupa o saltos.
    pub(crate) fn needs_headroom(&self) -> bool {
        self.is_hovered || self.is_animating() || self.hover.value() > 0.0
    }
}
