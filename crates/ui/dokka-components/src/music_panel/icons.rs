use std::sync::Arc;

use dokka_icons::{media_next_icon, media_pause_icon, media_play_icon, media_previous_icon};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Botones ya construidos con los colores del tema. Se reutilizan entre
/// fotogramas: la plataforma solo repinta cuando recibe otro dibujo.
/// Blancos sobre el cristal ahumado; los atenuados (misma forma en blanco
/// tenue) se muestran sin canción.
#[derive(Debug)]
pub struct MusicPanelIcons {
    pub(super) previous: Arc<Drawing>,
    pub(super) play: Arc<Drawing>,
    pub(super) pause: Arc<Drawing>,
    pub(super) next: Arc<Drawing>,
    pub(super) previous_dim: Arc<Drawing>,
    pub(super) play_dim: Arc<Drawing>,
    pub(super) pause_dim: Arc<Drawing>,
    pub(super) next_dim: Arc<Drawing>,
}

impl MusicPanelIcons {
    pub fn new(theme: &Theme) -> Self {
        let bright = theme.tray.text;
        let dim = theme.tray.secondary;
        Self {
            previous: Arc::new(media_previous_icon(bright)),
            play: Arc::new(media_play_icon(bright)),
            pause: Arc::new(media_pause_icon(bright)),
            next: Arc::new(media_next_icon(bright)),
            previous_dim: Arc::new(media_previous_icon(dim)),
            play_dim: Arc::new(media_play_icon(dim)),
            pause_dim: Arc::new(media_pause_icon(dim)),
            next_dim: Arc::new(media_next_icon(dim)),
        }
    }
}
