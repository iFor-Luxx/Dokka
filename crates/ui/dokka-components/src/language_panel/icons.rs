use std::sync::Arc;

use dokka_icons::language_check_icon;
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Marca Lucide del idioma activo, ya construida con los colores de un tema.
/// Se reutiliza entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo.
#[derive(Debug)]
pub struct LanguageIcons {
    normal: Arc<Drawing>,
    highlighted: Arc<Drawing>,
}

impl LanguageIcons {
    pub fn new(theme: &Theme) -> Self {
        Self {
            normal: Arc::new(language_check_icon(theme.menu.text)),
            highlighted: Arc::new(language_check_icon(theme.menu.highlighted_text)),
        }
    }

    pub(crate) fn for_row(&self, is_highlighted: bool) -> Arc<Drawing> {
        if is_highlighted {
            Arc::clone(&self.highlighted)
        } else {
            Arc::clone(&self.normal)
        }
    }
}
