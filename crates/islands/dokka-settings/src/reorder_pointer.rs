//! Arrastre de filas de la barra: del punto pulsado al hueco de suelta.
//!
//! Se agarra desde cualquier punto de la fila salvo el interruptor. Hasta
//! pasar el umbral no hay arrastre: un clic simple no mueve nada. La fila
//! sigue al cursor en vertical sin salirse de la tarjeta y las demás dejan
//! el hueco donde caería. Al soltar, una sola vez y solo si cambió el orden.

use dokka_theme::settings as measures;

use super::logic::{SECTION_MENUBAR, menubar_row_at, menubar_top, reorder_target};
use super::menubar_drag::MenubarDrag;
use super::state::Settings;

/// Orden mostrado con el arrastre a medias, para dibujar sin guardar todavía.
pub(crate) fn display_order(
    values: &super::values::ContentValues,
) -> [dokka_model::MenubarItem; 7] {
    match values.reorder {
        Some(drag) => drag.display(),
        None => values.preferences.menubar.order,
    }
}

impl Settings {
    /// Recuerda la fila pulsada si puede empezar un arrastre: sección de la
    /// barra, dentro de la tarjeta y fuera del interruptor.
    pub(crate) fn remember_reorder_press(&mut self, x: f32, y: f32) {
        if self.selected() != SECTION_MENUBAR {
            return;
        }
        let width = self.content_width();
        if super::logic::menubar_switch_at(self.selected(), width, x, y).is_some() {
            return;
        }
        let display = display_order(self.values());
        let Some(row) = menubar_row_at(&display, width, x, y) else {
            return;
        };
        let base = self.values().preferences.menubar.order;
        // La fila en el orden base que vive en este hueco mostrado.
        let item = display[row.min(6)];
        let Some(from) = base.iter().position(|known| *known == item) else {
            return;
        };
        self.values_mut().reorder = Some(MenubarDrag::pressed(x, y, from, base));
    }

    /// Avanza la fila agarrada hasta este punto, si la hay.
    pub(crate) fn move_reorder(&mut self, x: f32, y: f32) {
        let Some(mut drag) = self.values().reorder else {
            return;
        };
        let Some(top) = menubar_top() else {
            return;
        };
        let target = reorder_target(top, y);
        if drag.moved(x, y, measures::REORDER_THRESHOLD, target) {
            self.values_mut().reorder = Some(drag);
        } else {
            // Antes del umbral también sigue al cursor para el próximo paso.
            self.values_mut().reorder = Some(drag);
        }
    }

    /// Suelta la fila: si pasó el umbral y cambió el orden, lo guarda y emite
    /// `Changed` una sola vez. Devuelve si hubo arrastre que resolver.
    pub(crate) fn finish_reorder(&mut self) -> Option<crate::SettingsAction> {
        let drag = self.values().reorder?;
        self.values_mut().reorder = None;
        if !drag.is_active() {
            return None;
        }
        let order = super::logic::reordered(
            &self.values().preferences.menubar.order,
            drag.from(),
            drag.target(),
        );
        if order == self.values().preferences.menubar.order {
            return Some(crate::SettingsAction::Stay);
        }
        let mut preferences = self.values().preferences;
        preferences.menubar.order = order;
        self.values_mut().preferences = preferences;
        Some(crate::SettingsAction::Changed(preferences))
    }

    /// `true` si hay una fila agarrada que ya sigue al cursor.
    pub(crate) fn reorder_active(&self) -> bool {
        self.values().reorder.is_some_and(|drag| drag.is_active())
    }
}
