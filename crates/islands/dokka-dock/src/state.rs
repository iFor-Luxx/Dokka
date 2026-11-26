use std::collections::HashMap;

use dokka_animation::Spring;
use dokka_model::AppId;

use crate::motion::LaunchBounce;

const HOVER_STIFFNESS: f32 = 28.0;

/// Lo que el dock recuerda entre fotogramas.
#[derive(Debug)]
pub struct Dock {
    pub(crate) hover: Spring,
    pub(crate) is_hovered: bool,
    pub(crate) cursor_x: f32,
    pub(crate) bounces: HashMap<AppId, LaunchBounce>,
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            hover: Spring::new(0.0, HOVER_STIFFNESS),
            is_hovered: false,
            cursor_x: 0.0,
            bounces: HashMap::new(),
        }
    }
}
