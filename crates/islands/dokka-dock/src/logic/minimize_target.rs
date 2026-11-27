//! Hueco de destino del efecto genio dentro del dock.
//!
//! Con el dock visible es el hueco de la miniatura minimizada, ajustado a la
//! proporción de la ventana (como Muelle): centrado en su sitio a la derecha
//! del separador. Con el dock oculto con barrita es la propia barrita. La
//! geometría es la misma que dibuja la escena, así el vuelo aterriza donde
//! aparece la miniatura.

use dokka_model::{DockApp, WindowId};
use dokka_scene::Rect;

use super::{home_line, place_dock};
use crate::Dock;

impl Dock {
    /// Hueco en coordenadas de la ventana del dock para esta ventana
    /// minimizada (`aspect` es ancho entre alto), si ya tiene sitio a la
    /// derecha del separador.
    pub fn minimized_slot(&self, apps: &[DockApp], window: WindowId, aspect: f32) -> Option<Rect> {
        let placement = place_dock(self, apps);
        let slot = placement
            .slots
            .iter()
            .find_map(|(entry, slot)| match entry {
                super::Entry::Minimized(_, id) if *id == window => Some(*slot),
                _ => None,
            })?;
        Some(fit_to_aspect(slot, aspect))
    }

    /// `true` cuando solo se ve la barrita al borde.
    pub fn is_hidden(&self) -> bool {
        self.hide.is_hidden()
    }

    /// Destino del genio en coordenadas de la ventana del dock: la barrita
    /// cuando está oculto, o el hueco de la miniatura cuando se ve.
    pub fn genie_target(&self, apps: &[DockApp], window: WindowId, aspect: f32) -> Option<Rect> {
        if self.hide.is_hidden() {
            let size = self.surface_size(apps);
            return Some(home_line(self, size));
        }
        self.minimized_slot(apps, window, aspect)
    }
}

/// Centra un rectángulo de la proporción dada dentro del hueco.
/// Como `App::minimized_slot` de Muelle (MIT).
fn fit_to_aspect(slot: Rect, aspect: f32) -> Rect {
    if aspect.is_nan() || aspect <= 0.0 {
        return slot;
    }
    let (width, height) = if aspect >= 1.0 {
        (slot.width, slot.width / aspect)
    } else {
        (slot.height * aspect, slot.height)
    };
    Rect {
        x: slot.x + (slot.width - width) / 2.0,
        y: slot.y + (slot.height - height) / 2.0,
        width,
        height,
    }
}
