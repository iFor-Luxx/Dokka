//! Historial de secciones y búsqueda: qué se ve en la barra lateral.
//!
//! Elegir una sección nueva borra lo de adelante; la búsqueda filtra las
//! filas sin cambiar la sección elegida.

/// Navegación de Preferencias: sección, historial, búsqueda y su foco.
/// Vive aparte para que `state.rs` no crezca: el zoom y los hovers siguen allí.
#[derive(Debug)]
pub(crate) struct Navigation {
    selected: usize,
    history: Vec<usize>,
    position: usize,
    query: String,
    search_focused: bool,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            selected: 0,
            history: vec![0],
            position: 0,
            query: String::new(),
            search_focused: false,
        }
    }
}

impl Navigation {
    /// Vuelve a General sin historial, búsqueda ni foco. Los ajustes no se tocan.
    pub(crate) fn reset(&mut self) {
        self.selected = 0;
        self.history.clear();
        self.history.push(0);
        self.position = 0;
        self.query.clear();
        self.search_focused = false;
    }

    pub(crate) fn selected(&self) -> usize {
        self.selected
    }

    pub(crate) fn query(&self) -> &str {
        &self.query
    }

    pub(crate) fn query_mut(&mut self) -> &mut String {
        &mut self.query
    }

    pub(crate) fn search_focused(&self) -> bool {
        self.search_focused
    }

    pub(crate) fn set_search_focused(&mut self, focused: bool) {
        self.search_focused = focused;
    }

    pub(crate) fn position(&self) -> usize {
        self.position
    }

    pub(crate) fn history_len(&self) -> usize {
        self.history.len()
    }

    pub(crate) fn push_history(&mut self, index: usize) {
        self.history.truncate(self.position + 1);
        self.history.push(index);
        self.position = self.history.len() - 1;
        self.selected = index;
    }

    pub(crate) fn step_back(&mut self) {
        self.position -= 1;
        self.selected = self.history[self.position];
    }

    pub(crate) fn step_forward(&mut self) {
        self.position += 1;
        self.selected = self.history[self.position];
    }
}
