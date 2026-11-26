use std::collections::HashMap;
use std::sync::Arc;

use dokka_animation::Spring;
use dokka_model::AppId;
use dokka_theme::Theme;

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
    pub(crate) hover: Spring,
    pub(crate) is_hovered: bool,
    pub(crate) cursor_x: f32,
    pub(crate) bounces: HashMap<AppId, LaunchBounce>,
}

impl Dock {
    pub fn new(explorer: AppId, theme: Arc<Theme>) -> Self {
        Self {
            explorer,
            icons: SystemIcons::new(&theme.icons),
            theme,
            hover: Spring::new(0.0, HOVER_STIFFNESS),
            is_hovered: false,
            cursor_x: 0.0,
            bounces: HashMap::new(),
        }
    }
}
