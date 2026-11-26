use std::collections::HashMap;
use std::sync::Arc;

use dokka_icons::search_icon;
use dokka_model::{AppId, AppInfo};
use dokka_scene::{Bitmap, Color, Drawing, Size};
use dokka_theme::Theme;

use crate::LaunchpadAction;
use crate::logic::app_index;
use crate::motion::{PageMotion, ZoomMotion};
use crate::search_box::SearchBox;
use crate::tiles::TileCache;

/// Estado abierto/cerrado, búsqueda, página y app resaltada.
#[derive(Debug, Default)]
pub struct Launchpad {
    pub(crate) apps: Vec<AppInfo>,
    pub(crate) highlighted: Option<usize>,
    is_open: bool,
    pub(crate) search: SearchBox,
    pub(crate) page: usize,
    /// Deslizamiento o cesión de la cuadrícula en curso.
    pub(crate) motion: PageMotion,
    /// Apertura o cierre con zoom en curso.
    pub(crate) zoom: ZoomMotion,
    /// Lupa de la casilla, creada una vez y reutilizada con su color.
    magnifier: Option<(Color, Arc<Drawing>)>,
    tiles: TileCache,
}

impl Launchpad {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abre con la lista ya unida y ordenada; la búsqueda empieza vacía y entra con zoom.
    pub fn open(&mut self, apps: Vec<AppInfo>) {
        self.apps = apps;
        self.highlighted = None;
        self.search.reset();
        self.page = 0;
        self.motion = PageMotion::new();
        self.zoom.open();
        self.tiles.clear();
        self.is_open = true;
    }

    /// Cierra del todo: se llama cuando termina la animación de cierre.
    pub fn close(&mut self) {
        self.is_open = false;
        self.highlighted = None;
        self.search.reset();
        self.zoom = ZoomMotion::new();
        self.tiles.clear();
    }

    /// Empieza el cierre animado; sigue abierto hasta que termina.
    pub fn begin_close(&mut self) {
        self.zoom.begin_close();
    }

    /// Vuelve a abrirse desde donde está, cancelando el cierre.
    pub fn reopen(&mut self) {
        self.zoom.reopen();
    }

    /// `true` mientras el cierre está en curso.
    pub fn is_closing(&self) -> bool {
        self.zoom.is_closing()
    }

    /// `true` cuando el cierre terminó y se puede ocultar la ventana.
    pub fn close_finished(&self) -> bool {
        self.zoom.close_finished()
    }

    /// `true` mientras se abre o se cierra con zoom.
    pub fn is_zooming(&self) -> bool {
        self.zoom.is_zooming()
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
        self.search.query()
    }

    pub(crate) fn is_focused(&self) -> bool {
        self.search.is_focused()
    }

    pub(crate) fn is_cursor_visible(&self) -> bool {
        self.search.is_cursor_visible()
    }

    pub(crate) fn focus_search(&mut self) {
        self.search.focus();
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

    /// Clic: la casilla enfoca sin cerrar, los puntos cambian de página y el
    /// resto abre la app o cierra por vacío.
    pub fn click(&mut self, x: f32, y: f32, screen: Size) -> LaunchpadAction {
        crate::logic::click_launchpad(self, x, y, screen)
    }

    /// Resalta la app que hay en el punto.
    pub fn pointer_moved(&mut self, x: f32, y: f32, screen: Size) {
        self.highlighted = app_index(self, x, y, screen);
    }

    pub fn pointer_left(&mut self) {
        self.highlighted = None;
    }
}
