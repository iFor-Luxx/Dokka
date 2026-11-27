//! Qué iconos de la fila se ven, según Preferencias.

use dokka_model::MenubarPreferences;

use super::TrayKind;
use crate::Menubar;

impl Menubar {
    /// Qué iconos de la fila derecha se muestran. Los apagados no se dibujan
    /// ni reciben clics; si estaba abierto su panel, se cierra.
    pub fn set_shown(&mut self, shown: MenubarPreferences) {
        self.shown = shown;
        if self.tray.is_some_and(|open| !self.is_shown(open.kind)) {
            self.close_tray();
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
