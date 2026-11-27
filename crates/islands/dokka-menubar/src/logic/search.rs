//! Buscador del escritorio: texto, resultados, resaltado y cursor.
//!
//! La isla guarda lo escrito y lo resaltado, como el calendario su mes;
//! el shell filtra el catálogo y resuelve los iconos con `IconCache`.

use dokka_components::search_panel::{
    SearchHit, SearchPanelItem, SearchPanelProps, search_panel_hit, search_panel_panel,
    search_panel_size,
};
use dokka_scene::Overlay;

use super::tray::tray_place_centered;
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Abre separado de la barra, centrado a lo ancho de la pantalla.
    /// El shell rellena los resultados justo después con `set_search_results`.
    pub fn open_search(&mut self, bar_width: f32, screen_height: f32) {
        if !self.is_shown(super::TrayKind::Search) {
            return;
        }
        self.search_query.clear();
        self.search_highlighted = None;
        self.search_cursor = true;
        let size = search_panel_size(&self.search_props());
        self.show_tray(
            super::TrayKind::Search,
            tray_place_centered(size, bar_width, screen_height),
        );
    }

    /// Resultados ya filtrados en su orden (el shell los calcula).
    /// Recorta el resaltado si la lista encogió y recoloca el panel
    /// separado de la barra, que cambia de alto con el número de filas.
    pub fn set_search_results(
        &mut self,
        results: Vec<SearchPanelItem>,
        bar_width: f32,
        screen_height: f32,
    ) {
        self.search_results = results;
        let visible = self
            .search_results
            .len()
            .min(dokka_theme::search::MAX_RESULTS);
        if self
            .search_highlighted
            .is_some_and(|index| index >= visible)
        {
            self.search_highlighted = None;
        }
        if self.tray_kind() == Some(super::TrayKind::Search) {
            let size = search_panel_size(&self.search_props());
            self.tray = self.tray.map(|open| super::OpenTray {
                panel: tray_place_centered(size, bar_width, screen_height),
                ..open
            });
        }
    }

    /// Alto de ventana necesario con el buscador abierto en este monitor.
    pub fn search_window_height(&self, screen_height: f32) -> f32 {
        let size = search_panel_size(&self.search_props());
        dokka_theme::search::spotlight_window_height(screen_height, size.height)
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub(crate) fn is_search_open(&self) -> bool {
        self.tray_kind() == Some(super::TrayKind::Search)
    }

    fn search_props(&self) -> SearchPanelProps {
        SearchPanelProps {
            query: self.search_query.clone(),
            results: self.search_results.clone(),
            highlighted: self.search_highlighted,
            cursor_visible: self.search_cursor,
        }
    }

    pub(crate) fn search_overlay(&self) -> Overlay {
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: dokka_theme::menubar::HEIGHT,
            width: search_panel_size(&self.search_props()).width,
            height: search_panel_size(&self.search_props()).height,
        });
        search_panel_panel(
            &self.theme,
            &self.search_icons,
            bounds,
            &self.search_props(),
        )
    }

    /// Letra escrita: filtra desde el principio y parpadea el cursor.
    pub fn search_typed(&mut self, character: char) {
        if !self.is_search_open() {
            return;
        }
        self.search_query.push(character);
        self.search_highlighted = None;
        self.search_cursor = true;
    }

    /// Retroceso: borra la última letra.
    pub fn search_erased(&mut self) {
        if !self.is_search_open() {
            return;
        }
        self.search_query.pop();
        self.search_highlighted = None;
        self.search_cursor = true;
    }

    /// Flechas: mueven el resaltado entre lo visible.
    pub fn search_up(&mut self) {
        self.step_search(false);
    }

    pub fn search_down(&mut self) {
        self.step_search(true);
    }

    fn step_search(&mut self, forward: bool) {
        if !self.is_search_open() {
            return;
        }
        let visible = self
            .search_results
            .len()
            .min(dokka_theme::search::MAX_RESULTS);
        if visible == 0 {
            self.search_highlighted = None;
            return;
        }
        let next = match self.search_highlighted {
            None => Some(0),
            Some(index) if forward => (index + 1 < visible).then_some(index + 1),
            Some(index) if index > 0 => Some(index - 1),
            Some(_) => None,
        };
        self.search_highlighted = next;
    }

    /// Enter abre lo resaltado (o lo primero si no hay nada resaltado).
    pub fn search_confirmed(&mut self) -> Option<MenubarAction> {
        if !self.is_search_open() {
            return None;
        }
        let visible = self
            .search_results
            .len()
            .min(dokka_theme::search::MAX_RESULTS);
        if visible == 0 {
            return None;
        }
        let index = self.search_highlighted.unwrap_or(0).min(visible - 1);
        Some(MenubarAction::SearchOpen(index))
    }

    /// Parpadeo del cursor, una vez por `Tick`.
    pub fn tick_search_cursor(&mut self) {
        if self.is_search_open() {
            self.search_cursor = !self.search_cursor;
        }
    }

    pub(crate) fn search_click(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        search_panel_hit(bounds, x, y, &self.search_props())
            .map(|SearchHit::Result(index)| MenubarAction::SearchOpen(index))
    }

    pub(crate) fn search_hover(&mut self, x: f32, y: f32) {
        let hit = self
            .tray_bounds()
            .and_then(|bounds| search_panel_hit(bounds, x, y, &self.search_props()));
        self.search_highlighted = hit.map(|hit| match hit {
            SearchHit::Result(index) => index,
        });
    }
}
