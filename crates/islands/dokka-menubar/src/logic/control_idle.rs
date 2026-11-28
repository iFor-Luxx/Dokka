//! Cierre solo del centro por inactividad: 6 s con el cursor fuera
//! y sin clics, arrastres ni teclas. La isla cuenta y decide; el shell
//! cierra por el mismo camino que un cierre normal.
//!
//! Solo el centro: los demás paneles no pasan por aquí. La decisión vive
//! en una sola función (`control_should_auto_close`) y excluye el
//! modo editar.

use super::TrayKind;
use crate::Menubar;

impl Menubar {
    /// El cursor está sobre el panel del centro abierto (`true`) o fuera
    /// (`false`). Solo vale con el centro abierto; la ventana recortada no
    /// manda movimientos de fuera, así que fuera solo llega `pointer_left`
    /// (que ya limpia) o estas coordenadas. Si está encima, reinicia la cuenta.
    pub fn update_control_cursor(&mut self, x: f32, y: f32) {
        if self.tray_kind() != Some(TrayKind::ControlCenter) {
            return;
        }
        self.control_cursor_inside = self.tray_contains(x, y);
        if self.control_cursor_inside {
            self.control_idle_ticks = 0;
        }
    }

    /// Cualquier uso reinicia la cuenta: clic, arrastre o tecla.
    /// Sin el centro abierto no hace nada.
    pub fn note_control_used(&mut self) {
        if self.tray_kind() == Some(TrayKind::ControlCenter) {
            self.control_idle_ticks = 0;
        }
    }

    /// Un `Tick` (1 s) de inactividad. Devuelve si el shell debe cerrarlo ya
    /// por el camino normal (`dismiss_menubar`, que devuelve el foco igual).
    /// No pinta nada: la cuenta no entra en la huella para no repintar cada segundo.
    pub fn tick_control_idle(&mut self) -> bool {
        if self.tray_kind() != Some(TrayKind::ControlCenter) {
            return false;
        }
        if self.dragging_slider || self.control_cursor_inside {
            self.control_idle_ticks = 0;
            return false;
        }
        self.control_idle_ticks = self.control_idle_ticks.saturating_add(1);
        self.control_should_auto_close()
    }

    /// La única decisión: cerrado tras los segundos del tema sin uso con
    /// el cursor fuera y sin arrastre. Editando no se cierra solo.
    fn control_should_auto_close(&self) -> bool {
        self.tray_kind() == Some(TrayKind::ControlCenter)
            && !self.is_control_editing()
            && !self.dragging_slider
            && !self.control_cursor_inside
            && self.control_idle_ticks >= dokka_theme::control_center::IDLE_CLOSE_SECONDS
    }
}
