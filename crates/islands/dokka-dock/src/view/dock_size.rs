use dokka_model::DockApp;
use dokka_scene::{Rect, Size};

use super::dock_style as style;
use crate::Dock;
use crate::logic::{bar_width, place_dock};

impl Dock {
    /// Tamaño de la ventana: barra más el espacio para crecer y saltar.
    pub fn surface_size(&self, app_count: usize) -> Size {
        let bar = bar_width(app_count, style::SLOT, style::GAP, style::PADDING);
        Size {
            width: bar + style::side_room() * 2.0,
            height: style::headroom() + style::HEIGHT,
        }
    }

    /// Parte de la ventana que se ve y recibe el mouse. El resto deja pasar los clics.
    pub fn active_area(&self, apps: &[DockApp]) -> Rect {
        if !self.needs_headroom() {
            return place_dock(self, apps).bar;
        }
        let size = self.surface_size(apps.len());
        Rect {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }
}
