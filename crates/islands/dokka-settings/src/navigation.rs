//! Historial de secciones y búsqueda: qué se ve en la barra lateral.
//!
//! Elegir una sección nueva borra lo de adelante; la búsqueda filtra las
//! filas sin cambiar la sección elegida. Dentro de General se navega entre
//! la lista y sus apartados: cada paso guarda su sitio para que la píldora
//! vuelva atrás y adelante y el título cambie.

use super::general::GeneralPage;

/// Un paso del historial: sección lateral y apartado de General.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NavTarget {
    section: usize,
    general: GeneralPage,
}

/// Navegación de Preferencias: sección, apartado, historial y búsqueda.
/// Vive aparte para que `state.rs` no crezca: el zoom y los hovers siguen allí.
#[derive(Debug)]
pub(crate) struct Navigation {
    history: Vec<NavTarget>,
    position: usize,
    query: String,
    search_focused: bool,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            history: vec![NavTarget {
                section: 0,
                general: GeneralPage::Root,
            }],
            position: 0,
            query: String::new(),
            search_focused: false,
        }
    }
}

impl Navigation {
    /// Vuelve a General sin historial, búsqueda ni foco. Los ajustes no se tocan.
    pub(crate) fn reset(&mut self) {
        self.history.clear();
        self.history.push(NavTarget {
            section: 0,
            general: GeneralPage::Root,
        });
        self.position = 0;
        self.query.clear();
        self.search_focused = false;
    }

    pub(crate) fn selected(&self) -> usize {
        self.history[self.position].section
    }

    pub(crate) fn general(&self) -> GeneralPage {
        self.history[self.position].general
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

    /// Guarda la sección lateral con su apartado raíz.
    pub(crate) fn push_history(&mut self, index: usize) {
        self.push_target(NavTarget {
            section: index,
            general: GeneralPage::Root,
        });
    }

    /// Entra en un apartado de General.
    pub(crate) fn push_general(&mut self, page: GeneralPage) {
        self.push_target(NavTarget {
            section: 0,
            general: page,
        });
    }

    fn push_target(&mut self, target: NavTarget) {
        if self.history[self.position] == target {
            return;
        }
        self.history.truncate(self.position + 1);
        self.history.push(target);
        self.position = self.history.len() - 1;
    }

    pub(crate) fn step_back(&mut self) {
        self.position -= 1;
    }

    pub(crate) fn step_forward(&mut self) {
        self.position += 1;
    }
}
