use std::sync::Arc;

use dokka_icons::{language_check_icon, tray_chevron_icon};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Marca de cada fila e icono de ajustes, ya construidos con el tema.
/// La marca de verificación indica el idioma en uso. Se reutiliza entre
/// fotogramas.
#[derive(Debug)]
pub struct LanguageIcons {
    check: Arc<Drawing>,
    chevron: Arc<Drawing>,
}

impl LanguageIcons {
    pub fn new(theme: &Theme) -> Self {
        Self {
            check: Arc::new(language_check_icon(theme.tray.text)),
            chevron: Arc::new(tray_chevron_icon(theme.tray.text)),
        }
    }

    /// Marca de verificación blanca del idioma activo, a la izquierda.
    pub(crate) fn check(&self) -> Arc<Drawing> {
        Arc::clone(&self.check)
    }

    /// Chevrón blanco de la zona de ajustes.
    pub(crate) fn chevron(&self) -> &Arc<Drawing> {
        &self.chevron
    }
}
