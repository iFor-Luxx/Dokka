//! Páginas de la cuadrícula: capacidad, recuento y apps visibles.
//!
//! La paginación vive aquí; el movimiento entre páginas vive en `motion`.

use dokka_components::app_grid::page_capacity;
use dokka_model::AppInfo;
use dokka_scene::Size;

use crate::Launchpad;
use crate::logic::filter_apps;

impl Launchpad {
    /// Apps que pasan el filtro, en el orden que ya traían.
    pub fn filtered_apps(&self) -> Vec<AppInfo> {
        filter_apps(&self.apps, self.query())
    }

    /// Cuántas apps caben en una página para esta pantalla.
    pub fn apps_per_page(&self, screen: Size) -> usize {
        page_capacity(screen).max(1)
    }

    /// Cuántas páginas tiene lo filtrado; siempre al menos una.
    pub fn page_count(&self, screen: Size) -> usize {
        let per_page = self.apps_per_page(screen);
        (self.filtered_apps().len().div_ceil(per_page)).max(1)
    }

    /// Página actual limitada a las que existen.
    pub fn current_page(&self, screen: Size) -> usize {
        self.page.min(self.page_count(screen) - 1)
    }

    /// Apps filtradas de la página actual.
    pub fn visible_apps(&self, screen: Size) -> Vec<AppInfo> {
        let per_page = self.apps_per_page(screen);
        let filtered = self.filtered_apps();
        let start = self.current_page(screen) * per_page;
        filtered.into_iter().skip(start).take(per_page).collect()
    }

    /// Primera coincidencia de la búsqueda, si hay alguna.
    pub fn first_match(&self) -> Option<AppInfo> {
        self.filtered_apps().into_iter().next()
    }
}
