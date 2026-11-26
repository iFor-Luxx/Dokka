use dokka_model::DockApp;
use dokka_scene::{Rect, Size};

use super::dock_style as style;
use crate::Dock;
use crate::logic::{list_entries, place_dock};

impl Dock {
    /// Tamaño de la ventana: barra, espacio para crecer y sitio para paneles.
    pub fn surface_size(&self, apps: &[DockApp]) -> Size {
        let (entries, _) = list_entries(apps, &self.explorer);
        Size {
            width: style::bar_width(entries.len()) + style::side_room() * 2.0,
            height: style::headroom() + style::HEIGHT,
        }
    }

    /// Parte del dock que se ve y recibe el mouse. El resto deja pasar los clics.
    pub fn active_area(&self, apps: &[DockApp]) -> Rect {
        let bar = place_dock(self, apps).bar;
        if !self.needs_headroom() {
            return bar;
        }
        let rise = style::rise_space();
        Rect {
            x: 0.0,
            y: bar.y - rise,
            width: self.surface_size(apps).width,
            height: rise + bar.height,
        }
    }
}
