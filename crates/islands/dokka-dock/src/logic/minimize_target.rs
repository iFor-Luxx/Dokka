//! Hueco de destino del efecto genio dentro del dock.
//!
//! Con el dock visible es el hueco de la miniatura minimizada; con el dock
//! oculto con barrita es la propia barrita. La geometria es la misma que
//! dibuja la escena, asi el vuelo aterriza donde aparece la miniatura.

use dokka_model::{DockApp, WindowId};
use dokka_scene::Rect;

use super::{home_line, place_dock};
use crate::Dock;

impl Dock {
    /// Hueco en coordenadas de la ventana del dock para esta ventana
    /// minimizada, si ya tiene sitio a la derecha del separador.
    pub fn minimized_slot(&self, apps: &[DockApp], window: WindowId) -> Option<Rect> {
        let placement = place_dock(self, apps);
        placement
            .slots
            .iter()
            .find_map(|(entry, slot)| match entry {
                super::Entry::Minimized(_, id) if *id == window => Some(*slot),
                _ => None,
            })
    }

    /// `true` cuando solo se ve la barrita al borde.
    pub fn is_hidden(&self) -> bool {
        self.hide.is_hidden()
    }

    /// Destino del genio en coordenadas de la ventana del dock: la barrita
    /// cuando esta oculto, o el hueco de la miniatura cuando se ve.
    pub fn genie_target(&self, apps: &[DockApp], window: WindowId) -> Option<Rect> {
        if self.hide.is_hidden() {
            let size = self.surface_size(apps);
            return Some(home_line(self, size));
        }
        self.minimized_slot(apps, window)
    }
}
