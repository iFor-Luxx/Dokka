use dokka_model::DockApp;
use dokka_scene::{Rect, Size};

use super::dock_style as style;
use crate::Dock;
use crate::logic::{home_touch, is_vertical, list_entries, place_dock, position_or_bottom};

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
    /// Escondido es solo la zona de toque de la línea, pegada al borde.
    /// Con una app ocupando, la zona visible se alarga hasta el borde para
    /// que el cursor suba desde la línea sin salirse.
    pub fn active_area(&self, apps: &[DockApp]) -> Rect {
        if self.hide.is_hidden() {
            let size = self.surface_size(apps);
            return home_touch(size, position_or_bottom(self));
        }
        let bar = self.visible_area(apps);
        if self.hide.is_occluded() && self.preferences.auto_hide {
            return self.extended_to_edge(bar);
        }
        bar
    }

    fn visible_area(&self, apps: &[DockApp]) -> Rect {
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

    /// Alarga la zona visible hasta el borde de la pantalla para no perder
    /// el cursor entre la línea y la barra. Solo el margen del tema.
    fn extended_to_edge(&self, area: Rect) -> Rect {
        let margin = style::bottom_margin(self);
        match position_or_bottom(self) {
            dokka_model::DockPosition::Left => Rect {
                x: area.x - margin,
                y: area.y,
                width: area.width + margin,
                height: area.height,
            },
            dokka_model::DockPosition::Right => Rect {
                x: area.x,
                y: area.y,
                width: area.width + margin,
                height: area.height,
            },
            _ => Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: area.height + margin,
            },
        }
    }
}
