//! Estado de General: resumen del equipo, rueda y dibujos creados una vez.
//!
//! Vive aparte para que `state.rs` no crezca: la navegación ya vive en
//! `navigation.rs` y los valores en `values.rs`.

use std::sync::Arc;

use dokka_components::nav_pill::{NavPillIcons, NavPillProps};
use dokka_components::window_controls::TrafficButtons;
use dokka_scene::{Drawing, Rect};
use dokka_theme::{Theme, settings as measures};

use super::state::Settings;

impl Settings {
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

    /// Agarre de las filas reordenables, creado una vez y reutilizado.
    pub(crate) fn grip_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        self.drawings.grip(theme)
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

    /// Cuadrado de color de una categoría de almacenamiento, creado una vez.
    pub(crate) fn storage_badge_drawing(&mut self, theme: &Theme, index: usize) -> Arc<Drawing> {
        self.drawings.storage_badge(theme, index)
    }

    /// Glifo blanco de una categoría de almacenamiento, creado una vez.
    pub(crate) fn storage_row_drawing(&mut self, theme: &Theme, index: usize) -> Arc<Drawing> {
        self.drawings.storage_row(theme, index)
    }

    /// Chevrón derecho de las filas de General, creado una vez.
    pub(crate) fn chevron_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        self.drawings.chevron(theme)
    }

    /// Menos del editor de fecha, creado una vez.
    pub(crate) fn stepper_minus_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        self.drawings.stepper_minus(theme)
    }

    /// Más del editor de fecha, creado una vez.
    pub(crate) fn stepper_plus_drawing(&mut self, theme: &Theme) -> Arc<Drawing> {
        self.drawings.stepper_plus(theme)
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

    /// Fecha y hora que muestra "Fecha y hora", si el shell ya la pasó.
    pub fn page_now(&self) -> Option<super::general::PageNow> {
        self.values().now
    }

    /// Guarda la fecha y hora en vivo para la página de fecha y hora.
    pub fn set_page_now(&mut self, now: super::general::PageNow) {
        self.values_mut().now = Some(now);
    }

    /// Idiomas de la tarjeta de "Idioma y región": cuantos leyó Windows.
    pub fn language_count(&self) -> usize {
        self.system
            .as_ref()
            .and_then(|summary| summary.locale.languages.as_ref())
            .map(|languages| languages.len())
            .unwrap_or(0)
    }

    /// Otros discos de "Almacenamiento": cuantos trae el desglose de hoy.
    pub(crate) fn disk_count(&self) -> usize {
        self.storage
            .as_ref()
            .map(|known| known.other_disks.len())
            .unwrap_or(0)
    }

    /// Cuentas variables de General: idiomas y otros discos de hoy.
    pub(crate) fn list_counts(&self) -> super::logic::ListCounts {
        super::logic::ListCounts::new(self.language_count(), self.disk_count())
    }

    /// Desplazamiento vertical del contenido de General, en lógicas.
    pub fn scroll_offset(&self) -> f32 {
        self.scroll
    }

    /// Mueve el contenido con la rueda, sin salirse de su altura.
    /// La lista cabe sin moverse; cada apartado baja con su propia altura.
    pub(crate) fn scroll_by(&mut self, delta: f32) {
        let page = self.navigation_general();
        let edit = self.datetime_edit.clone();
        let max = super::logic::general_scroll_max(self.window_height(), page, self.list_counts(), &edit);
        self.scroll = (self.scroll + delta).clamp(0.0, max);
    }

    /// Vuelve arriba al cambiar de apartado o de sección.
    pub(crate) fn clear_scroll(&mut self) {
        self.scroll = 0.0;
    }
}
