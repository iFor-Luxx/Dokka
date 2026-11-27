use std::sync::Arc;

use dokka_icons::{media_next_icon, media_pause_icon, media_play_icon, media_previous_icon};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Botones ya construidos con los colores del tema. Se reutilizan entre
/// fotogramas: la plataforma solo repinta cuando recibe otro dibujo.
/// Reutiliza los glifos de música existentes, en el texto oscuro del panel.
#[derive(Debug)]
pub struct MusicPanelIcons {
    pub(super) previous: Arc<Drawing>,
    pub(super) play: Arc<Drawing>,
    pub(super) pause: Arc<Drawing>,
    pub(super) next: Arc<Drawing>,
}

impl MusicPanelIcons {
    pub fn new(theme: &Theme) -> Self {
        let color = theme.panel.text;
        Self {
            previous: Arc::new(media_previous_icon(color)),
            play: Arc::new(media_play_icon(color)),
            pause: Arc::new(media_pause_icon(color)),
            next: Arc::new(media_next_icon(color)),
        }
    }
}
