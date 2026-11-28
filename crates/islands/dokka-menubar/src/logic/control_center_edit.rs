//! Modo editar del Centro de control: entra con la burbuja, sale al cerrar.
//!
//! El estado vive en la isla (`control_editing`); la geometría de pintar y
//! pulsar es la misma del componente. Ocultar es inmediato (se reacomoda ya)
//! y el shell lo guarda. Preparado para que el paso 2b abra aquí su galería.

use dokka_model::{ControlItem, HiddenControls};

use crate::Menubar;

impl Menubar {
    /// `true` si el centro está en modo editar (con botones "−").
    pub fn is_control_editing(&self) -> bool {
        self.control_editing && self.tray_kind() == Some(super::TrayKind::ControlCenter)
    }

    /// Entra en modo editar: aparecen los "−" y desaparece la burbuja.
    pub(crate) fn enter_control_edit(&mut self) {
        self.control_editing = true;
        self.dragging_slider = false;
        self.dragging_brightness = false;
        self.resize_control_center();
    }

    /// Oculta este control al momento, reacomoda y devuelve lo guardable.
    pub(crate) fn hide_control(&mut self, item: ControlItem) -> HiddenControls {
        let mut hidden = self.shown.hidden_controls;
        hidden.insert(item);
        self.shown.hidden_controls = hidden;
        self.resize_control_center();
        hidden
    }
}
