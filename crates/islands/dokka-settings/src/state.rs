use std::sync::Arc;

use dokka_icons::settings_back_icon;
use dokka_icons::settings_forward_icon;
use dokka_scene::{Color, Drawing};
use dokka_theme::{Theme, settings as measures};

use super::logic::section_drawing;
use crate::SettingsAction;

/// Ventana abierta o cerrada, sección elegida, fila bajo el cursor y dibujos
/// Lucide creados una vez y reutilizados en cada redibujado.
#[derive(Debug)]
pub struct Settings {
    is_open: bool,
    selected: usize,
    hovered: Option<usize>,
    rows: Vec<Option<(Color, Arc<Drawing>)>>,
    search: Option<(Color, Arc<Drawing>)>,
    nav: [Option<(Color, Arc<Drawing>)>; 2],
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            is_open: false,
            selected: 0,
            hovered: None,
            rows: vec![None; measures::SECTIONS],
            search: None,
            nav: [None, None],
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abre la ventana; si ya estaba abierta, no cambia la sección.
    pub fn open(&mut self) {
        self.is_open = true;
        self.hovered = None;
    }

    /// Cierra la ventana: ocultar no borra la sección elegida.
    pub fn close(&mut self) {
        self.is_open = false;
        self.hovered = None;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub(crate) fn hovered(&self) -> Option<usize> {
        self.hovered
    }

    /// Resalta la fila que hay en el punto.
    pub fn pointer_moved(&mut self, x: f32, y: f32) {
        self.hovered = dokka_components::settings_list::row_at(x, y);
    }

    pub fn pointer_left(&mut self) {
        self.hovered = None;
    }

    /// Aplica el clic: elige fila o pide cerrar o minimizar.
    pub fn click(&mut self, x: f32, y: f32) -> SettingsAction {
        if let Some(button) = dokka_components::window_controls::traffic_at(x, y) {
            return match button {
                dokka_components::window_controls::TrafficButton::Close => SettingsAction::Close,
                dokka_components::window_controls::TrafficButton::Minimize => {
                    SettingsAction::Minimize
                }
                dokka_components::window_controls::TrafficButton::Zoom => SettingsAction::Stay,
            };
        }
        if let Some(index) = dokka_components::settings_list::row_at(x, y) {
            self.selected = index;
            self.hovered = Some(index);
        }
        SettingsAction::Stay
    }

    /// Dibujo blanco del icono de la fila, creado una vez y reutilizado.
    pub(crate) fn row_drawing(&mut self, theme: &Theme, index: usize) -> Arc<Drawing> {
        let color = theme.settings.row_text_selected;
        if self.rows[index]
            .as_ref()
            .is_none_or(|(known, _)| *known != color)
        {
            self.rows[index] = Some((color, Arc::new(section_drawing(index, color))));
        }
        match &self.rows[index] {
            Some((_, drawing)) => Arc::clone(drawing),
            None => Arc::new(section_drawing(index, color)),
        }
    }

    /// Lupa de la casilla, creada una vez y reutilizada.
    pub(crate) fn search_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        let color = theme.settings.search_icon;
        if self
            .search
            .as_ref()
            .is_none_or(|(known, _)| *known != color)
        {
            self.search = Some((color, Arc::new(dokka_icons::search_icon(color))));
        }
        match &self.search {
            Some((_, drawing)) => Arc::clone(drawing),
            None => Arc::new(dokka_icons::search_icon(color)),
        }
    }

    /// Chevrones atenuados de la cabecera, creados una vez y reutilizados.
    pub(crate) fn nav_drawings(&mut self, theme: &Theme) -> [Arc<Drawing>; 2] {
        let color = theme.settings.nav;
        let makers = [settings_back_icon, settings_forward_icon];
        for (slot, make) in self.nav.iter_mut().zip(makers) {
            if slot.as_ref().is_none_or(|(known, _)| *known != color) {
                *slot = Some((color, Arc::new(make(color))));
            }
        }
        [
            self.nav_drawing(0, color, settings_back_icon),
            self.nav_drawing(1, color, settings_forward_icon),
        ]
    }

    fn nav_drawing(&self, index: usize, color: Color, make: fn(Color) -> Drawing) -> Arc<Drawing> {
        match &self.nav[index] {
            Some((_, drawing)) => Arc::clone(drawing),
            None => Arc::new(make(color)),
        }
    }
}
