use std::sync::Arc;

use dokka_components::nav_pill::{NavHalf, NavPillIcons, NavPillProps, nav_pill_at};
use dokka_components::search_field::search_contains_at;
use dokka_components::settings_list::{filtered_row_at, search_bounds};
use dokka_components::window_controls::{TrafficButton, TrafficButtons, traffic_at};
use dokka_scene::{Drawing, Rect};
use dokka_theme::{Theme, settings as measures};

use super::drawings::CachedDrawings;
use super::logic::filter_sections;
use super::painted::PaintedSettings;
use crate::SettingsAction;

/// Ventana abierta o cerrada, historial de secciones, búsqueda y focos, con
/// los dibujos creados una vez y reutilizados en cada redibujado.
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
    }

    /// Cierra la ventana: ocultar no borra la sección elegida ni el historial.
    pub fn close(&mut self) {
        self.is_open = false;
        self.hovered = None;
        self.nav_hovered = None;
        self.traffic_hovered = false;
        self.search_focused = false;
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

    /// Lo ya dibujado, para que el shell solo repinte si cambió algo.
    pub fn painted(&self) -> PaintedSettings {
        PaintedSettings::of(self)
    }

    /// Resalta la fila visible que hay en el punto, los semáforos y la píldora.
    pub fn pointer_moved(&mut self, x: f32, y: f32) {
        self.hovered = filtered_row_at(&self.filtered_order(), x, y);
        self.traffic_hovered = traffic_at(x, y).is_some();
        let pill = self.pill_props();
        self.nav_hovered = nav_pill_at(&pill, x, y);
    }

    pub fn pointer_left(&mut self) {
        self.hovered = None;
        self.nav_hovered = None;
        self.traffic_hovered = false;
    }

    /// La ventana ganó o perdió el foco: sin foco los semáforos van en gris
    /// y se apagan los resaltados.
    pub fn set_window_focused(&mut self, focused: bool) {
        self.window_focused = focused;
        if !focused {
            self.hovered = None;
            self.nav_hovered = None;
            self.traffic_hovered = false;
        }
    }

    /// Aplica el clic: semáforos, foco de la casilla, píldora o fila visible.
    pub fn click(&mut self, x: f32, y: f32) -> SettingsAction {
        if let Some(button) = traffic_at(x, y) {
            return match button {
                TrafficButton::Close => SettingsAction::Close,
                TrafficButton::Minimize => SettingsAction::Minimize,
                TrafficButton::Zoom => SettingsAction::Stay,
            };
        }
        if search_contains_at(search_bounds(), x, y) {
            self.search_focused = true;
            return SettingsAction::Stay;
        }
        self.search_focused = false;
        let pill = self.pill_props();
        if let Some(half) = nav_pill_at(&pill, x, y) {
            match half {
                NavHalf::Previous => self.go_back(),
                NavHalf::Next => self.go_forward(),
            }
            return SettingsAction::Stay;
        }
        if let Some(index) = filtered_row_at(&self.filtered_order(), x, y) {
            self.select_section(index);
        }
        SettingsAction::Stay
    }

    /// Letra escrita con la casilla enfocada: filtra desde arriba.
    pub fn type_character(&mut self, character: char) {
        if !self.search_focused {
            return;
        }
        if self.query.chars().count() >= measures::SEARCH_MAX_CHARACTERS {
            return;
        }
        self.query.push(character);
    }

    /// Retroceso con la casilla enfocada: borra la última letra.
    pub fn erase_character(&mut self) {
        if self.search_focused {
            self.query.pop();
        }
    }

    /// Enter con búsqueda: elige la primera coincidencia.
    pub fn confirm_search(&mut self) {
        if !self.search_focused || self.query.is_empty() {
            return;
        }
        if let Some(first) = self.filtered_order().into_iter().next() {
            self.select_section(first);
        }
    }

    /// Esc: limpia lo escrito y quita el foco.
    pub fn escape_search(&mut self) {
        if self.search_focused || !self.query.is_empty() {
            self.query.clear();
            self.search_focused = false;
        }
    }

    /// Elige una sección nueva y borra lo que había "adelante".
    fn select_section(&mut self, index: usize) {
        if index != self.selected {
            self.history.truncate(self.position + 1);
            self.history.push(index);
            self.position = self.history.len() - 1;
            self.selected = index;
        }
        self.hovered = Some(index);
    }

    /// Vuelve a la sección anterior del historial.
    fn go_back(&mut self) {
        if self.can_previous() {
            self.position -= 1;
            self.selected = self.history[self.position];
        }
    }

    /// Avanza a la sección siguiente del historial.
    fn go_forward(&mut self) {
        if self.can_next() {
            self.position += 1;
            self.selected = self.history[self.position];
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
}
