use std::sync::Arc;

use dokka_icons::{search_bar_icon, search_field_icon};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Dibujos del buscador ya construidos con los colores del tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo. Sin tema en caliente, basta con crearlos una vez.
#[derive(Debug, Clone)]
pub struct SearchPanelIcons {
    field: Arc<Drawing>,
    bar: Arc<Drawing>,
}

impl SearchPanelIcons {
    pub fn new(theme: &Theme) -> Self {
        Self {
            field: Arc::new(search_field_icon(theme.tray.text)),
            bar: Arc::new(search_bar_icon(theme.menubar.text)),
        }
    }

    /// Lupa blanca de la cápsula grande sobre el cristal ahumado.
    pub fn field(&self) -> &Arc<Drawing> {
        &self.field
    }

    /// Lupa blanca de la barra superior para abrir el buscador.
    pub fn bar(&self) -> &Arc<Drawing> {
        &self.bar
    }
}
