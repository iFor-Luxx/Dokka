use std::collections::HashMap;
use std::sync::Arc;

use dokka_components::app_grid::{AppGridItem, AppGridProps, app_at, page_capacity};
use dokka_components::page_dots::{PageDotsProps, dot_at};
use dokka_components::search_field::{SearchFieldProps, search_contains};
use dokka_icons::search_icon;
use dokka_model::{AppId, AppInfo};
use dokka_scene::{Bitmap, Color, Drawing, Size};
use dokka_theme::Theme;
use dokka_theme::launchpad as measures;

use crate::LaunchpadAction;
use crate::logic::filter_apps;
use crate::motion::PageMotion;
use crate::tiles::TileCache;

/// Estado abierto/cerrado, búsqueda, página y app resaltada.
#[derive(Debug, Default)]
pub struct Launchpad {
    apps: Vec<AppInfo>,
    pub(crate) highlighted: Option<usize>,
    is_open: bool,
    query: String,
    pub(crate) page: usize,
    /// Deslizamiento o cesión de la cuadrícula en curso.
    pub(crate) motion: PageMotion,
    /// Lupa de la casilla, creada una vez y reutilizada con su color.
    magnifier: Option<(Color, Arc<Drawing>)>,
    tiles: TileCache,
}

impl Launchpad {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abre con la lista ya unida y ordenada; la búsqueda empieza vacía.
    pub fn open(&mut self, apps: Vec<AppInfo>) {
        self.apps = apps;
        self.highlighted = None;
        self.query.clear();
        self.page = 0;
        self.motion = PageMotion::new();
        self.tiles.clear();
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.highlighted = None;
        self.tiles.clear();
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn apps(&self) -> &[AppInfo] {
        &self.apps
    }

    pub fn highlighted(&self) -> Option<usize> {
        self.highlighted
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    /// Apps que pasan el filtro, en el orden que ya traían.
    pub fn filtered_apps(&self) -> Vec<AppInfo> {
        filter_apps(&self.apps, &self.query)
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

    /// Añade lo escrito y vuelve a la primera página.
    pub fn type_character(&mut self, character: char) {
        if character.is_control() {
            return;
        }
        if self.query.chars().count() >= measures::SEARCH_MAX_CHARACTERS {
            return;
        }
        self.query.push(character);
        self.page = 0;
        self.motion = PageMotion::new();
        self.highlighted = None;
    }

    /// Borra la última letra y vuelve a la primera página.
    pub fn erase_character(&mut self) {
        self.query.pop();
        self.page = 0;
        self.motion = PageMotion::new();
        self.highlighted = None;
    }

    /// Enter abre la primera coincidencia; sin texto no hace nada.
    pub fn confirm(&self) -> LaunchpadAction {
        if self.query.is_empty() {
            return LaunchpadAction::Stay;
        }
        match self.first_match() {
            Some(app) => LaunchpadAction::Open(app),
            None => LaunchpadAction::Stay,
        }
    }

    /// Esc limpia lo escrito y, si ya está vacío, cierra.
    pub fn escape(&mut self) -> LaunchpadAction {
        if self.query.is_empty() {
            return LaunchpadAction::Close;
        }
        self.query.clear();
        self.page = 0;
        self.motion = PageMotion::new();
        self.highlighted = None;
        LaunchpadAction::Stay
    }

    /// Lupa de la casilla con este color, creada una vez y reutilizada.
    pub fn magnifier(&mut self, color: Color) -> Arc<Drawing> {
        if let Some((cached, drawing)) = &self.magnifier
            && *cached == color
        {
            return Arc::clone(drawing);
        }
        let drawing = Arc::new(search_icon(color));
        self.magnifier = Some((color, Arc::clone(&drawing)));
        drawing
    }

    /// Baldosas uniformes para el redibujado, calculadas una sola vez por app.
    pub fn tiled_images(
        &mut self,
        images: &HashMap<AppId, Arc<Bitmap>>,
        theme: &Theme,
    ) -> HashMap<AppId, Arc<Bitmap>> {
        self.tiles.tiled(&self.apps, images, theme)
    }

    /// Resalta la app que hay en el punto.
    pub fn pointer_moved(&mut self, x: f32, y: f32, screen: Size) {
        self.highlighted = self.app_index(x, y, screen);
    }

    pub fn pointer_left(&mut self) {
        self.highlighted = None;
    }

    /// Clic: la casilla no hace nada, los puntos cambian de página y el
    /// resto abre la app o cierra por vacío.
    pub fn click(&self, x: f32, y: f32, screen: Size) -> LaunchpadAction {
        let search = SearchFieldProps {
            screen,
            query: self.query.clone(),
        };
        if search_contains(&search, x, y) {
            return LaunchpadAction::Stay;
        }
        let dots = PageDotsProps {
            screen,
            total: self.page_count(screen),
            current: self.current_page(screen),
        };
        if let Some(page) = dot_at(&dots, x, y) {
            return LaunchpadAction::ShowPage(page);
        }
        let index = self.app_index(x, y, screen);
        match index.and_then(|at| self.visible_apps(screen).get(at).cloned()) {
            Some(app) => LaunchpadAction::Open(app),
            None => LaunchpadAction::Close,
        }
    }

    fn app_index(&self, x: f32, y: f32, screen: Size) -> Option<usize> {
        let apps = self
            .visible_apps(screen)
            .into_iter()
            .map(|app| AppGridItem {
                name: app.name.clone(),
                image: None,
            })
            .collect();
        let props = AppGridProps {
            apps,
            highlighted: self.highlighted,
            screen,
        };
        app_at(&props, x, y)
    }
}
