use std::sync::Arc;

use dokka_icons::{nav_next_icon, nav_previous_icon};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Chevrones de la píldora ya construidos con los colores del tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo. Sin tema en caliente, basta con crearlos una vez.
#[derive(Debug, Clone)]
pub struct NavPillIcons {
    previous: Arc<Drawing>,
    previous_dimmed: Arc<Drawing>,
    next: Arc<Drawing>,
    next_dimmed: Arc<Drawing>,
}

impl NavPillIcons {
    pub fn new(theme: &Theme) -> Self {
        Self {
            previous: Arc::new(nav_previous_icon(theme.nav_pill.icon)),
            previous_dimmed: Arc::new(nav_previous_icon(theme.nav_pill.icon_disabled)),
            next: Arc::new(nav_next_icon(theme.nav_pill.icon)),
            next_dimmed: Arc::new(nav_next_icon(theme.nav_pill.icon_disabled)),
        }
    }

    pub(crate) fn previous(&self, available: bool) -> &Arc<Drawing> {
        if available {
            &self.previous
        } else {
            &self.previous_dimmed
        }
    }

    pub(crate) fn next(&self, available: bool) -> &Arc<Drawing> {
        if available {
            &self.next
        } else {
            &self.next_dimmed
        }
    }
}
