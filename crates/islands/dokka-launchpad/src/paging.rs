//! Páginas de la cuadrícula: capacidad, recuento y apps visibles.
//!
//! La paginación vive aquí; el movimiento entre páginas vive en `motion`.

use dokka_components::app_grid::page_capacity;
use dokka_model::AppInfo;
use dokka_scene::Size;

use crate::Launchpad;
use crate::logic::{filter_apps, normalized};

impl Launchpad {
    /// Apps que pasan el filtro, en el orden que ya traían.
    pub fn filtered_apps(&self) -> Vec<AppInfo> {
        filter_apps(&self.apps, self.query())
    }

    /// Cuántas pasan el filtro, sin clonarlas.
    pub(crate) fn filtered_count(&self) -> usize {
        let wanted = normalized(self.query());
        if wanted.is_empty() {
            return self.apps.len();
        }
        self.apps
            .iter()
            .filter(|app| normalized(&app.name).contains(&wanted))
            .count()
    }

    /// Cuántas apps caben en una página para esta pantalla.
    pub fn apps_per_page(&self, screen: Size) -> usize {
        page_capacity(screen).max(1)
    }

    /// Cuántas páginas tiene lo filtrado; siempre al menos una.
    /// Cuenta sin clonar para no copiar todas las apps por fotograma.
    pub fn page_count(&self, screen: Size) -> usize {
        let per_page = self.apps_per_page(screen);
        (self.filtered_count().div_ceil(per_page)).max(1)
    }

    /// Página actual limitada a las que existen.
    pub fn current_page(&self, screen: Size) -> usize {
        self.page.min(self.page_count(screen) - 1)
    }

    /// Apps filtradas de la página actual, con un solo filtrado por llamada.
    pub fn visible_apps(&self, screen: Size) -> Vec<AppInfo> {
        let filtered = self.filtered_apps();
        let per_page = self.apps_per_page(screen);
        let count = (filtered.len().div_ceil(per_page)).max(1);
        let start = self.page.min(count - 1) * per_page;
        filtered.into_iter().skip(start).take(per_page).collect()
    }

    /// Primera coincidencia de la búsqueda, si hay alguna.
    /// No clona toda la lista: se queda con la primera que coincide.
    pub fn first_match(&self) -> Option<AppInfo> {
        let wanted = normalized(self.query());
        if wanted.is_empty() {
            return self.apps.first().cloned();
        }
        self.apps
            .iter()
            .find(|app| normalized(&app.name).contains(&wanted))
            .cloned()
    }
}
