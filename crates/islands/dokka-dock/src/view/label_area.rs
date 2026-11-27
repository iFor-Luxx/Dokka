use dokka_model::{DockApp, DockPosition};
use dokka_scene::Rect;
use dokka_theme::tooltip as measures;

use crate::Dock;
use crate::logic::{hovered_slot, is_vertical, place_dock, position_or_bottom};

impl Dock {
    /// Zona de la etiqueta lateral mientras se muestra: `Some` solo con el
    /// dock en vertical y cuando hay etiqueta (la misma condición que dibuja
    /// `view/dock_bar.rs`, vía `hovered_slot`). Empieza en el borde interior
    /// del hueco señalado, va hacia dentro el ancho reservado y centra el alto
    /// reservado en el centro vertical del hueco. Abajo no cambia nada.
    pub fn label_area(&self, apps: &[DockApp]) -> Option<Rect> {
        if !is_vertical(self) {
            return None;
        }
        let placement = place_dock(self, apps);
        let (_, slot) = hovered_slot(self, &placement)?;
        let width = measures::RESERVED_WIDTH;
        let height = measures::RESERVED_HEIGHT;
        let middle = slot.y + slot.height / 2.0;
        match position_or_bottom(self) {
            DockPosition::Right => Some(Rect {
                x: slot.x - width,
                y: middle - height / 2.0,
                width,
                height,
            }),
            _ => Some(Rect {
                x: slot.x + slot.width,
                y: middle - height / 2.0,
                width,
                height,
            }),
        }
    }
}
