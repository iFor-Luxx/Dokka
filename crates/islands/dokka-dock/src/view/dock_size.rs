use dokka_model::DockApp;
use dokka_scene::{Rect, Size};

use super::dock_style as style;
use crate::Dock;
use crate::logic::{list_entries, place_dock};

impl Dock {
    /// Tamaño de la ventana: barra más el espacio para crecer y saltar.
    pub fn surface_size(&self, apps: &[DockApp]) -> Size {
        let (entries, _) = list_entries(apps, &self.explorer);
        Size {
            width: style::bar_width(entries.len()) + style::side_room() * 2.0,
            height: style::headroom() + style::HEIGHT,
        }
    }

    /// Parte de la ventana que se ve y recibe el mouse. El resto deja pasar los clics.
    pub fn active_area(&self, apps: &[DockApp]) -> Rect {
        if !self.needs_headroom() {
            return place_dock(self, apps).bar;
        }
        let size = self.surface_size(apps);
        Rect {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }
}
