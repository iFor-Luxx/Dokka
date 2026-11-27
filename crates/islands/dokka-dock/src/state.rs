use std::collections::HashMap;
use std::sync::Arc;

use dokka_animation::Spring;
use dokka_components::menu_panel::{MenuChecks, MenuChevrons};
use dokka_model::{AppId, DockPreferences};
use dokka_theme::Theme;

use crate::logic::OpenMenu;
use crate::motion::LaunchBounce;
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
    pub(crate) bounces: HashMap<AppId, LaunchBounce>,
    pub(crate) menu: Option<OpenMenu>,
    pub(crate) preferences: DockPreferences,
}

impl Dock {
    pub fn new(explorer: AppId, theme: Arc<Theme>) -> Self {
        Self {
            explorer,
            icons: SystemIcons::new(&theme.icons),
            menu_chevrons: MenuChevrons::new(&theme),
            menu_checks: MenuChecks::new(&theme),
            theme,
            hover: Spring::new(0.0, HOVER_STIFFNESS),
            is_hovered: false,
            cursor_x: 0.0,
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
    }
}
