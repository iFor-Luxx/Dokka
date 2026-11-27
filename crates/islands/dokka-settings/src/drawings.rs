use std::sync::Arc;

use dokka_components::nav_pill::NavPillIcons;
use dokka_components::window_controls::TrafficButtons;
use dokka_scene::{Color, Drawing};
use dokka_theme::{Theme, settings as measures};

use super::logic::{badge_drawing, section_drawing};

/// Dibujos creados una vez y reutilizados en cada redibujado: la plataforma
/// solo repinta cuando recibe otro dibujo.
#[derive(Debug, Default)]
pub(crate) struct CachedDrawings {
    badges: Vec<Option<(Color, Arc<Drawing>)>>,
    rows: Vec<Option<(Color, Arc<Drawing>)>>,
    search: Option<(Color, Arc<Drawing>)>,
    grip: Option<(Color, Arc<Drawing>)>,
    nav: Option<NavPillIcons>,
    traffic: Option<TrafficButtons>,
}

impl CachedDrawings {
    pub(crate) fn new() -> Self {
        Self {
            badges: vec![None; measures::SECTIONS],
            rows: vec![None; measures::SECTIONS],
            search: None,
            grip: None,
            nav: None,
            traffic: None,
        }
    }

    /// Cuadrado de color de la fila como imagen, creado una vez y reutilizado.
    pub(crate) fn badge(&mut self, theme: &Theme, index: usize) -> Arc<Drawing> {
        let color = theme.settings.badge[index];
        if self.badges[index]
            .as_ref()
            .is_none_or(|(known, _)| *known != color)
        {
            self.badges[index] = Some((color, Arc::new(badge_drawing(color))));
        }
        match &self.badges[index] {
            Some((_, drawing)) => Arc::clone(drawing),
            None => Arc::new(badge_drawing(color)),
        }
    }

    /// Dibujo blanco del icono de la fila, creado una vez y reutilizado.
    pub(crate) fn row(&mut self, theme: &Theme, index: usize) -> Arc<Drawing> {
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
    pub(crate) fn search(&mut self, theme: &Theme) -> Arc<Drawing> {
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

    /// Agarre de las filas reordenables, creado una vez y reutilizado.
    pub(crate) fn grip(&mut self, theme: &Theme) -> Arc<Drawing> {
        let color = theme.settings.card_text;
        if self
            .grip
            .as_ref()
            .is_none_or(|(known, _)| *known != color)
        {
            self.grip = Some((color, Arc::new(dokka_icons::settings_grip_icon(color))));
        }
        match &self.grip {
            Some((_, drawing)) => Arc::clone(drawing),
            None => Arc::new(dokka_icons::settings_grip_icon(color)),
        }
    }

    /// Chevrones de la píldora según el historial, creados una vez.
    pub(crate) fn nav(&mut self, theme: &Theme) -> NavPillIcons {
        if self.nav.is_none() {
            self.nav = Some(NavPillIcons::new(theme));
        }
        // El tema no cambia con Dokka abierto: si hay caché, vale.
        self.nav.clone().unwrap_or_else(|| NavPillIcons::new(theme))
    }

    /// Botones completos de los semáforos, creados una vez y reutilizados.
    pub(crate) fn traffic(&mut self, theme: &Theme) -> TrafficButtons {
        if self.traffic.is_none() {
            self.traffic = Some(TrafficButtons::new(theme));
        }
        // El tema no cambia con Dokka abierto: si hay caché, vale.
        self.traffic
            .clone()
            .unwrap_or_else(|| TrafficButtons::new(theme))
    }
}
