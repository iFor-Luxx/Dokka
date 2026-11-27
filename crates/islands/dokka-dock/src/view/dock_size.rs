use dokka_model::DockApp;
use dokka_scene::{Rect, Size};

use super::dock_style as style;
use crate::Dock;
use crate::logic::{is_vertical, list_entries, place_dock};

impl Dock {
    /// Tamaño de la ventana: barra, espacio para crecer y sitio para paneles.
    /// Abajo es idéntico al de hoy; en laterales se intercambian los ejes.
    pub fn surface_size(&self, apps: &[DockApp]) -> Size {
        let (entries, _) = list_entries(apps, &self.explorer);
        let length = style::bar_width(self, entries.len()) + style::side_room(self) * 2.0;
        let cross = style::headroom(self) + style::height(self);
        if is_vertical(self) {
            Size {
                width: cross,
                height: length,
            }
        } else {
            Size {
                width: length,
                height: cross,
            }
        }
    }

    /// Parte del dock que se ve y recibe el mouse. El resto deja pasar los clics:
    /// solo la barra más lo que crece con la lupa, sin el margen del submenú.
    /// Escondido es solo la franja fina del borde.
    pub fn active_area(&self, apps: &[DockApp]) -> Rect {
        if self.hide.is_hidden() {
            return self.hidden_strip(apps);
        }
        let bar = place_dock(self, apps).bar;
        if !self.needs_headroom() {
            return bar;
        }
        let rise = style::rise_space(self);
        let magnify = style::magnify_room(self);
        let (entries, _) = list_entries(apps, &self.explorer);
        if is_vertical(self) {
            Rect {
                x: bar.x
                    - if self.preferences.position == dokka_model::DockPosition::Right {
                        rise
                    } else {
                        0.0
                    },
                y: style::side_room(self) - magnify,
                width: rise + bar.width,
                height: style::bar_width(self, entries.len()) + magnify * 2.0,
            }
        } else {
            Rect {
                x: style::side_room(self) - magnify,
                y: bar.y - rise,
                width: style::bar_width(self, entries.len()) + magnify * 2.0,
                height: rise + bar.height,
            }
        }
    }

    fn hidden_strip(&self, apps: &[DockApp]) -> Rect {
        let size = self.surface_size(apps);
        let strip = dokka_theme::dock::HIDE_STRIP;
        match self.preferences.position {
            dokka_model::DockPosition::Left => Rect {
                x: size.width - strip,
                y: 0.0,
                width: strip,
                height: size.height,
            },
            dokka_model::DockPosition::Right => Rect {
                x: 0.0,
                y: 0.0,
                width: strip,
                height: size.height,
            },
            _ => Rect {
                x: 0.0,
                y: 0.0,
                width: size.width,
                height: strip,
            },
        }
    }
}
