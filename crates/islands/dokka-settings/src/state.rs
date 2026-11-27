use std::sync::Arc;

use dokka_components::nav_pill::{NavHalf, NavPillIcons, NavPillProps, nav_pill_at};
use dokka_components::settings_list::{filtered_row_at, search_bounds};
use dokka_components::window_controls::{TrafficButton, TrafficButtons, traffic_at};
use dokka_model::Preferences;
use dokka_scene::{Drawing, Rect};
use dokka_theme::{Theme, settings as measures};

use super::drawings::CachedDrawings;
use super::logic::filter_sections;
use super::painted::{PaintedSettings, painted_drag};
use super::values::ContentValues;

/// Ventana abierta o cerrada, historial de secciones, búsqueda y focos, con
/// los dibujos creados una vez y reutilizados en cada redibujado. Los valores
/// reales del contenido van en `values`: el shell los fija al abrir con
/// `set_values` y la isla avisa con `Changed` o `TaskbarAuto` al soltar.
#[derive(Debug)]
pub struct Settings {
    is_open: bool,
    selected: usize,
    hovered: Option<usize>,
    history: Vec<usize>,
    position: usize,
    query: String,
    search_focused: bool,
    traffic_hovered: bool,
    nav_hovered: Option<NavHalf>,
    window_focused: bool,
    values: ContentValues,
    drawings: CachedDrawings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            is_open: false,
            selected: 0,
            hovered: None,
            history: vec![0],
            position: 0,
            query: String::new(),
            search_focused: false,
            traffic_hovered: false,
            nav_hovered: None,
            window_focused: true,
            values: ContentValues::default(),
            drawings: CachedDrawings::new(),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abre la ventana; si ya estaba abierta, no cambia la sección. Vuelve al
    /// frente, así que recupera el foco.
    pub fn open(&mut self) {
        self.is_open = true;
        self.hovered = None;
        self.nav_hovered = None;
        self.traffic_hovered = false;
        self.window_focused = true;
        self.values.control = None;
        self.values.drag = None;
    }

    /// Cierra la ventana: ocultar no borra la sección elegida ni el historial.
    pub fn close(&mut self) {
        self.is_open = false;
        self.hovered = None;
        self.nav_hovered = None;
        self.traffic_hovered = false;
        self.search_focused = false;
        self.values.control = None;
        self.values.drag = None;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn hovered(&self) -> Option<usize> {
        self.hovered
    }

    /// Mitad de la píldora bajo el cursor, si hay alguna disponible.
    pub fn nav_hovered(&self) -> Option<NavHalf> {
        self.nav_hovered
    }

    /// Lo escrito en la casilla de búsqueda.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// `true` cuando la casilla tiene el foco: el cursor se ve.
    pub fn search_focused(&self) -> bool {
        self.search_focused
    }

    /// `true` cuando el cursor está sobre algún semáforo.
    pub fn traffic_hovered(&self) -> bool {
        self.traffic_hovered
    }

    /// `true` cuando la ventana es la activa: los semáforos van en color.
    pub fn window_focused(&self) -> bool {
        self.window_focused
    }

    /// Valores reales que muestra el contenido.
    pub fn preferences(&self) -> Preferences {
        self.values.preferences
    }

    /// Si la barra de Windows se oculta sola.
    pub fn taskbar_auto(&self) -> bool {
        self.values.taskbar_auto
    }

    /// `true` si hay una sección anterior a la que volver.
    pub fn can_previous(&self) -> bool {
        self.position > 0
    }

    /// `true` si hay una sección siguiente a la que avanzar.
    pub fn can_next(&self) -> bool {
        self.position + 1 < self.history.len()
    }

    /// Secciones visibles con la búsqueda actual, en su orden.
    pub fn filtered_order(&self) -> Vec<usize> {
        filter_sections(&self.query)
    }

    /// Recuerda lo vigente antes de mostrar: el shell lo llama al abrir.
    pub fn set_values(&mut self, preferences: Preferences, taskbar_auto: bool) {
        self.values.set_values(preferences, taskbar_auto);
    }

    /// Píldora con los lados disponibles según el historial y el hover.
    pub(crate) fn pill_props(&self) -> NavPillProps {
        NavPillProps {
            bounds: Rect {
                x: measures::NAV_LEFT,
                y: measures::NAV_TOP,
                width: measures::NAV_PILL_WIDTH,
                height: measures::NAV_PILL_HEIGHT,
            },
            can_previous: self.can_previous(),
            can_next: self.can_next(),
            hovered: self.nav_hovered,
        }
    }

    /// Lo ya dibujado, para que el shell solo repinte si cambió algo: incluye
    /// los valores y el hover del contenido.
    pub fn painted(&self) -> PaintedSettings {
        PaintedSettings {
            selected: self.selected,
            hovered: self.hovered,
            nav_hovered: self.nav_hovered,
            traffic_hovered: self.traffic_hovered,
            window_focused: self.window_focused,
            query: self.query.clone(),
            search_focused: self.search_focused,
            can_previous: self.can_previous(),
            can_next: self.can_next(),
            preferences: self.values.preferences,
            taskbar_auto: self.values.taskbar_auto,
            control: self.values.control,
            drag: painted_drag(&self.values),
        }
    }

    /// Cuadrado de color de la fila como imagen, creado una vez y reutilizado.
    pub(crate) fn badge_drawing(&mut self, theme: &Theme, index: usize) -> Arc<Drawing> {
        self.drawings.badge(theme, index)
    }

    /// Dibujo blanco del icono de la fila, creado una vez y reutilizado.
    pub(crate) fn row_drawing(&mut self, theme: &Theme, index: usize) -> Arc<Drawing> {
        self.drawings.row(theme, index)
    }

    /// Lupa de la casilla, creada una vez y reutilizada.
    pub(crate) fn search_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        self.drawings.search(theme)
    }

    /// Chevrones de la píldora según el historial, creados una vez.
    pub(crate) fn nav_drawings(&mut self, theme: &Theme) -> NavPillIcons {
        self.drawings.nav(theme)
    }

    /// Botones completos de los semáforos, creados una vez y reutilizados.
    pub(crate) fn traffic_drawings(&mut self, theme: &Theme) -> TrafficButtons {
        self.drawings.traffic(theme)
    }

    pub(crate) fn values(&self) -> &ContentValues {
        &self.values
    }

    pub(crate) fn values_mut(&mut self) -> &mut ContentValues {
        &mut self.values
    }

    pub(crate) fn set_hovered(&mut self, hovered: Option<usize>) {
        self.hovered = hovered;
    }

    pub(crate) fn set_traffic_hovered(&mut self, hovered: bool) {
        self.traffic_hovered = hovered;
    }

    pub(crate) fn set_nav_hovered(&mut self, hovered: Option<NavHalf>) {
        self.nav_hovered = hovered;
    }

    pub(crate) fn set_search_focused(&mut self, focused: bool) {
        self.search_focused = focused;
    }

    pub(crate) fn query_mut(&mut self) -> &mut String {
        &mut self.query
    }

    pub(crate) fn select_section(&mut self, index: usize) {
        if index != self.selected {
            self.history.truncate(self.position + 1);
            self.history.push(index);
            self.position = self.history.len() - 1;
            self.selected = index;
        }
        self.hovered = Some(index);
    }

    pub(crate) fn go_back(&mut self) {
        if self.can_previous() {
            self.position -= 1;
            self.selected = self.history[self.position];
        }
    }

    pub(crate) fn go_forward(&mut self) {
        if self.can_next() {
            self.position += 1;
            self.selected = self.history[self.position];
        }
    }

    pub(crate) fn clear_search_state(&mut self) {
        self.search_focused = false;
    }

    /// Reenvío para no exponer el tráfico en el puntero.
    pub(crate) fn traffic_button_at(x: f32, y: f32) -> Option<TrafficButton> {
        traffic_at(x, y)
    }

    pub(crate) fn search_hit_at(x: f32, y: f32) -> bool {
        use dokka_components::search_field::search_contains_at;
        search_contains_at(search_bounds(), x, y)
    }

    pub(crate) fn pill_at(&self, x: f32, y: f32) -> Option<NavHalf> {
        nav_pill_at(&self.pill_props(), x, y)
    }

    pub(crate) fn filtered_at(&self, x: f32, y: f32) -> Option<usize> {
        filtered_row_at(&self.filtered_order(), x, y)
    }
}
