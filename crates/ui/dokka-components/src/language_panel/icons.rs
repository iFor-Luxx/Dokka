use std::sync::Arc;

use dokka_icons::{language_row_icon, tray_chevron_icon, tray_settings_icon};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Teclado de cada fila e iconos de ajustes, ya construidos con el tema.
/// Sin marca de verificación: el círculo blanco con icono azul ya indica
/// el idioma en uso. Se reutiliza entre fotogramas.
#[derive(Debug)]
pub struct LanguageIcons {
    active: Arc<Drawing>,
    inactive: Arc<Drawing>,
    settings: Arc<Drawing>,
    chevron: Arc<Drawing>,
}

impl LanguageIcons {
    pub fn new(theme: &Theme) -> Self {
        Self {
            active: Arc::new(language_row_icon(theme.tray.circle_active_icon)),
            inactive: Arc::new(language_row_icon(theme.tray.circle_icon)),
            settings: Arc::new(tray_settings_icon(theme.tray.circle_icon)),
            chevron: Arc::new(tray_chevron_icon(theme.tray.text)),
        }
    }

    /// Teclado del círculo: azul sobre blanco si es el idioma en uso,
    /// blanco sobre translúcido si no.
    pub(crate) fn for_row(&self, is_active: bool) -> Arc<Drawing> {
        if is_active {
            Arc::clone(&self.active)
        } else {
            Arc::clone(&self.inactive)
        }
    }

    /// Engranaje blanco de la zona de ajustes.
    pub(crate) fn settings(&self) -> &Arc<Drawing> {
        &self.settings
    }

    /// Chevrón blanco de la zona de ajustes.
    pub(crate) fn chevron(&self) -> &Arc<Drawing> {
        &self.chevron
    }
}
