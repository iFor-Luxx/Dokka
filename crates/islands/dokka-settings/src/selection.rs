//! Historial de secciones y búsqueda: qué se ve en la barra lateral.
//!
//! Elegir una sección nueva borra lo de adelante; la búsqueda filtra las
//! filas sin cambiar la sección elegida.

use super::state::Settings;

impl Settings {
    /// `true` si hay una sección anterior a la que volver.
    pub fn can_previous(&self) -> bool {
        self.navigation_can_previous()
    }

    /// `true` si hay una sección siguiente a la que avanzar.
    pub fn can_next(&self) -> bool {
        self.navigation_can_next()
    }

    /// Secciones visibles con la búsqueda actual, en su orden.
    pub fn filtered_order(&self) -> Vec<usize> {
        super::logic::filter_sections(self.query())
    }

    pub(crate) fn navigation_can_previous(&self) -> bool {
        self.position() > 0
    }

    pub(crate) fn navigation_can_next(&self) -> bool {
        self.position() + 1 < self.history_len()
    }

    pub(crate) fn select_section(&mut self, index: usize) {
        let selected = self.selected();
        if index != selected {
            self.push_history(index);
        }
        self.set_hovered(Some(index));
    }

    pub(crate) fn go_back(&mut self) {
        if self.navigation_can_previous() {
            self.step_back();
        }
    }

    pub(crate) fn go_forward(&mut self) {
        if self.navigation_can_next() {
            self.step_forward();
        }
    }

    pub(crate) fn clear_search_state(&mut self) {
        self.set_search_focused(false);
    }
}
