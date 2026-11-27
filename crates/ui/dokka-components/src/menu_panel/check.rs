use std::sync::Arc;

use dokka_icons::checkmark_icon;
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Marcas de opción elegida ya construidas con los colores de un tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo. Van en la misma columna que los iconos.
#[derive(Debug)]
pub struct MenuChecks {
    pub(super) normal: Arc<Drawing>,
    pub(super) highlighted: Arc<Drawing>,
}

impl MenuChecks {
    pub fn new(theme: &Theme) -> Self {
        Self {
            normal: Arc::new(checkmark_icon(theme.menu.text)),
            highlighted: Arc::new(checkmark_icon(theme.menu.highlighted_text)),
        }
    }
}
