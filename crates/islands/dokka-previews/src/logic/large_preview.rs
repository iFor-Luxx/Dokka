//! Espera de la vista grande: aparece tras un segundo sobre la mini.
//!
//! El resaltado de la mini y sus botones siguen al instante (`hovered`);
//! esto solo decide qué ventana se muestra grande.

use crate::Previews;

/// Cuánto tarda en aparecer la vista grande sobre una mini.
const LARGE_OPEN_SECONDS: f32 = 1.0;

impl Previews {
    /// Recuerda la mini bajo el cursor para la vista grande.
    /// El resaltado ya está puesto: esto solo decide lo grande.
    pub(crate) fn update_large(&mut self, hovered: Option<u64>) {
        match hovered {
            None => {
                // Salir de la mini la quita al instante.
                self.large = None;
                self.large_wait = None;
            }
            Some(window) if self.large == Some(window) => {
                self.large_wait = None;
            }
            Some(window) if self.large.is_some() => {
                // Ya visible: cambia al instante.
                self.large = Some(window);
                self.large_wait = None;
            }
            Some(window) => {
                let waited = match &self.large_wait {
                    Some((old, waited)) if *old == window => *waited,
                    _ => 0.0,
                };
                self.large_wait = Some((window, waited));
            }
        }
    }

    /// Avanza la espera del segundo; al cumplirlo muestra la ventana.
    pub(crate) fn advance_large(&mut self, seconds: f32) {
        let ready = match &mut self.large_wait {
            Some((_, waited)) => {
                *waited += seconds;
                *waited >= LARGE_OPEN_SECONDS
            }
            None => false,
        };
        if ready {
            self.large = self.large_wait.take().map(|(window, _)| window);
        }
    }

    /// Ventana con vista grande, tras esperar su segundo sobre la mini.
    /// Distinta de `hovered_window()`: el resaltado sigue al instante.
    pub fn large_preview_window(&self) -> Option<u64> {
        let large = self.large?;
        self.open_windows().contains(&large).then_some(large)
    }
}
