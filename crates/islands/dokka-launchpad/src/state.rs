use dokka_components::app_grid::{AppGridItem, AppGridProps, app_at};
use dokka_model::AppInfo;
use dokka_scene::Size;

use crate::LaunchpadAction;

/// Estado abierto/cerrado, apps visibles y resaltado bajo el cursor.
#[derive(Debug, Default)]
pub struct Launchpad {
    apps: Vec<AppInfo>,
    highlighted: Option<usize>,
    is_open: bool,
}

impl Launchpad {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abre con la lista ya unida y ordenada.
    pub fn open(&mut self, apps: Vec<AppInfo>) {
        self.apps = apps;
        self.highlighted = None;
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.highlighted = None;
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

    /// Resalta la app que hay en el punto.
    pub fn pointer_moved(&mut self, x: f32, y: f32, screen: Size) {
        self.highlighted = self.app_index(x, y, screen);
    }

    pub fn pointer_left(&mut self) {
        self.highlighted = None;
    }

    /// Clic en la pantalla: app elegida o cierre por vacío.
    pub fn click(&self, x: f32, y: f32, screen: Size) -> LaunchpadAction {
        let index = self.app_index(x, y, screen);
        match index.and_then(|at| self.apps.get(at)).cloned() {
            Some(app) => LaunchpadAction::Open(app),
            None => LaunchpadAction::Close,
        }
    }

    fn app_index(&self, x: f32, y: f32, screen: Size) -> Option<usize> {
        let apps = self
            .apps
            .iter()
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
