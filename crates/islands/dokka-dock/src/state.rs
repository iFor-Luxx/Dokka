use std::collections::HashMap;
use std::sync::Arc;

use dokka_animation::Spring;
use dokka_components::menu_panel::{MenuChecks, MenuChevrons};
use dokka_model::{AppId, DockPreferences};
use dokka_theme::Theme;

use crate::logic::OpenMenu;
use crate::motion::{Hide, LaunchBounce};
use crate::view::system_icons::SystemIcons;

const HOVER_STIFFNESS: f32 = 28.0;

/// Lo que el dock recuerda entre fotogramas.
#[derive(Debug)]
pub struct Dock {
    /// App que ocupa el primer lugar con el icono del explorador.
    pub(crate) explorer: AppId,
    pub(crate) theme: Arc<Theme>,
    pub(crate) icons: SystemIcons,
    /// Chevrones del menú en los dos colores; se reutilizan para no repintar.
    pub(crate) menu_chevrons: MenuChevrons,
    /// Marcas de opción elegida en los dos colores; se reutilizan igual.
    pub(crate) menu_checks: MenuChecks,
    pub(crate) hover: Spring,
    pub(crate) is_hovered: bool,
    pub(crate) cursor_x: f32,
    pub(crate) cursor_y: f32,
    pub(crate) cursor_inside: bool,
    pub(crate) hide: Hide,
    /// Encogido para caber en vertical con muchas apps (1.0 sin encoger).
    pub(crate) fit: f32,
    pub(crate) bounces: HashMap<AppId, LaunchBounce>,
    pub(crate) menu: Option<OpenMenu>,
    pub(crate) preferences: DockPreferences,
}

impl Dock {
    pub fn new(explorer: AppId, theme: Arc<Theme>) -> Self {
        Self {
            explorer,
            icons: SystemIcons::new(&theme),
            menu_chevrons: MenuChevrons::new(&theme),
            menu_checks: MenuChecks::new(&theme),
            theme,
            hover: Spring::new(0.0, HOVER_STIFFNESS),
            is_hovered: false,
            cursor_x: 0.0,
            cursor_y: 0.0,
            cursor_inside: false,
            hide: Hide::new(),
            fit: 1.0,
            bounces: HashMap::new(),
            menu: None,
            preferences: DockPreferences::default(),
        }
    }

    /// Guarda lo ajustado en Preferencias para el dock.
    /// Sin rebote (`animate_launch`), olvida los saltos en curso para que
    /// el icono vuelva a su sitio y no pida más fotogramas.
    pub fn set_preferences(&mut self, preferences: DockPreferences) {
        if !preferences.animate_launch {
            self.bounces.clear();
        }
        self.preferences = preferences;
        self.hide.set_enabled(preferences.auto_hide);
        // Sin auto-ocultado, vuelve a verse.
        if !preferences.auto_hide {
            self.hide.show_now();
        }
    }

    /// Guarda si una app ocupa la pantalla principal.
    /// Sin ese dato (falla sola) se llama con `false` y el dock se ve.
    pub fn set_occluded(&mut self, occluded: bool) {
        self.hide.set_occluded(occluded);
    }

    pub fn is_occluded(&self) -> bool {
        self.hide.is_occluded()
    }

    /// Coordenada a lo largo del dock bajo el cursor (x abajo, y en laterales).
    pub(crate) fn cursor_along(&self) -> f32 {
        if crate::logic::is_vertical(self) {
            self.cursor_y
        } else {
            self.cursor_x
        }
    }

    pub(crate) fn fit(&self) -> f32 {
        self.fit.clamp(0.2, 1.0)
    }

    /// Encoge el dock vertical para caber; lo calcula quien conoce la pantalla.
    pub fn set_fit(&mut self, fit: f32) {
        self.fit = fit.clamp(0.2, 1.0);
    }
}
