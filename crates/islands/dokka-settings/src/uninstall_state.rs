//! Estado de desinstalación en "Actualización de Dokka".
//!
//! Vive aparte para que `state.rs` no crezca: el shell fija si Dokka está
//! instalado (`set_uninstall_available`) y la isla solo muestra el botón en
//! rojo o atenuado y emite `Uninstall` al pulsarlo.

use super::painted::PaintedControl;
use super::state::Settings;

impl Settings {
    /// `true` si Dokka está instalado y se puede desinstalar.
    pub fn can_uninstall(&self) -> bool {
        self.uninstall_available
    }

    /// Fija si el botón responde (el shell lo pasa al abrir y al entrar).
    pub fn set_uninstall_available(&mut self, available: bool) {
        self.uninstall_available = available;
    }

    /// Resalta la fila de desinstalar si el cursor cae en ella y responde.
    pub(crate) fn refresh_uninstall_hover(&mut self, width: f32, scroll: f32, x: f32, y: f32) {
        use super::general::GeneralPage as Page;
        if self.general() != Page::Update || !self.uninstall_available {
            return;
        }
        if super::logic::uninstall_button_at(width, scroll, x, y).is_some() {
            self.values_mut().control = Some(PaintedControl::Uninstall);
        }
    }
}
