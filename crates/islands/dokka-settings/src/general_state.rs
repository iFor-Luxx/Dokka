//! Estado de General: resumen del equipo, rueda y dibujos propios.
//!
//! Vive aparte para que `state.rs` no crezca: la navegación ya vive en
//! `navigation.rs` y los valores en `values.rs`.

use std::sync::Arc;

use dokka_components::nav_pill::NavPillProps;
use dokka_scene::{Drawing, Rect};
use dokka_theme::{Theme, settings as measures};

use super::state::Settings;

impl Settings {
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
            hovered: self.nav_hovered(),
        }
    }

    /// Cuadrado de color de una fila de General, creado una vez.
    pub(crate) fn general_badge_drawing(&mut self, theme: &Theme, row: usize) -> Arc<Drawing> {
        self.drawings.general_badge(theme, row)
    }

    /// Glifo blanco de una fila de General, creado una vez.
    pub(crate) fn general_row_drawing(&mut self, theme: &Theme, row: usize) -> Arc<Drawing> {
        self.drawings.general_row(theme, row)
    }

    /// Chevrón derecho de las filas de General, creado una vez.
    pub(crate) fn chevron_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        self.drawings.chevron(theme)
    }

    /// Engranaje grande de la cabecera de General, creado una vez.
    pub(crate) fn gear_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        self.drawings.gear(theme)
    }

    /// Resumen del equipo que muestra "Acerca de este PC".
    pub fn system_summary(&self) -> Option<dokka_model::SystemSummary> {
        self.system.clone()
    }

    /// Guarda el resumen leído por el shell (hilo propio).
    pub fn set_system_info(&mut self, summary: Option<dokka_model::SystemSummary>) {
        self.system = summary;
    }

    /// Desplazamiento vertical del contenido de General, en lógicas.
    pub fn scroll_offset(&self) -> f32 {
        self.scroll
    }

    /// Mueve el contenido con la rueda, sin salirse de su altura.
    /// Solo la ficha "Acerca" baja; la lista cabe sin moverse.
    pub(crate) fn scroll_by(&mut self, delta: f32) {
        let max = if self.navigation_general() == super::general::GeneralPage::About {
            super::logic::general_scroll_max(self.window_height())
        } else {
            0.0
        };
        self.scroll = (self.scroll + delta).clamp(0.0, max);
    }

    /// Vuelve arriba al cambiar de apartado o de sección.
    pub(crate) fn clear_scroll(&mut self) {
        self.scroll = 0.0;
    }
}
