//! Avance del fundido del contenido al cambiar de pantalla.

use super::Setup;

impl Setup {
    /// Acerca el fundido a 1. Devuelve `true` mientras sigue animando.
    pub fn advance(&mut self, seconds: f32) -> bool {
        self.advance_fade(seconds.max(0.0));
        self.fade() < 1.0
    }
}
