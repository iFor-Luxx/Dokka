//! Qué iconos de la fila se ven, según Preferencias.

use dokka_model::MenubarPreferences;

use super::TrayKind;
use crate::Menubar;

impl Menubar {
    /// Qué iconos de la fila derecha se muestran. Los apagados no se dibujan
    /// ni reciben clics; si estaba abierto su panel, se cierra. Si cambia el
    /// orden con un panel abierto, también se cierra para no quedarse colgando
    /// del icono viejo. Si cambian los ocultos del centro con el centro
    /// abierto, su tamaño se recalcula sin cerrarlo.
    pub fn set_shown(&mut self, shown: MenubarPreferences) {
        let order_changed = self.shown.order != shown.order;
        let hidden_changed = self.shown.hidden_controls != shown.hidden_controls;
        self.shown = shown;
        if order_changed && self.tray.is_some() {
            self.close_tray();
            return;
        }
        if self.tray.is_some_and(|open| !self.is_shown(open.kind)) {
            self.close_tray();
            return;
        }
        if hidden_changed && self.tray_kind() == Some(TrayKind::ControlCenter) {
            self.resize_control_center();
        }
    }

    /// `true` si el icono dado está encendido en Preferencias.
    /// Añadir un `TrayKind` nuevo es una línea más en este `match`.
    pub(crate) fn is_shown(&self, kind: TrayKind) -> bool {
        match kind {
            TrayKind::Volume => self.shown.volume,
            TrayKind::Network => self.shown.network,
            TrayKind::Music => self.shown.music,
            TrayKind::Background => self.shown.background_apps,
            TrayKind::Language => self.shown.language,
            TrayKind::Search => self.shown.search,
            TrayKind::ControlCenter => self.shown.control_center,
        }
    }
}
