use dokka_model::DockApp;

use super::place_dock;
use crate::Dock;

/// Distancia a cada lado de la línea que aún cuenta como pulsarla.
const REACH: f32 = 8.0;

impl Dock {
    /// `true` si el punto está sobre la línea separadora.
    pub fn is_on_divider(&self, x: f32, y: f32, apps: &[DockApp]) -> bool {
        let placement = place_dock(self, apps);
        let line = placement.divider.x + placement.divider.width / 2.0;
        let bar = placement.bar;
        (x - line).abs() <= REACH && y >= bar.y && y <= bar.y + bar.height
    }
}
