use std::sync::Arc;

use dokka_icons::{
    volume_high_icon, volume_low_icon, volume_muted_icon, volume_off_icon, volume_settings_icon,
};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Altavoz del panel y de la barra ya construidos con los colores del tema,
/// en blanco Tahoe. Se reutilizan entre fotogramas: la plataforma solo
/// repinta cuando recibe otro dibujo. Los laterales del deslizador tienen su
/// versión atenuada para el silencio, como los botones de música.
#[derive(Debug, Clone)]
pub struct VolumePanelIcons {
    muted: Arc<Drawing>,
    off: Arc<Drawing>,
    low: Arc<Drawing>,
    high: Arc<Drawing>,
    bar_muted: Arc<Drawing>,
    bar_off: Arc<Drawing>,
    bar_low: Arc<Drawing>,
    bar_high: Arc<Drawing>,
    slider_low: Arc<Drawing>,
    slider_low_dim: Arc<Drawing>,
    slider_high: Arc<Drawing>,
    slider_high_dim: Arc<Drawing>,
    check: Arc<Drawing>,
    chevron: Arc<Drawing>,
}

impl VolumePanelIcons {
    pub fn new(theme: &Theme) -> Self {
        let panel = theme.tray.text;
        let dim = theme.tray.secondary;
        let bar = theme.menubar.text;
        Self {
            muted: Arc::new(volume_muted_icon(panel)),
            off: Arc::new(volume_off_icon(panel)),
            low: Arc::new(volume_low_icon(panel)),
            high: Arc::new(volume_high_icon(panel)),
            bar_muted: Arc::new(volume_muted_icon(bar)),
            bar_off: Arc::new(volume_off_icon(bar)),
            bar_low: Arc::new(volume_low_icon(bar)),
            bar_high: Arc::new(volume_high_icon(bar)),
            slider_low: Arc::new(volume_low_icon(panel)),
            slider_low_dim: Arc::new(volume_low_icon(dim)),
            slider_high: Arc::new(volume_high_icon(panel)),
            slider_high_dim: Arc::new(volume_high_icon(dim)),
            check: Arc::new(dokka_icons::language_check_icon(panel)),
            chevron: Arc::new(volume_settings_icon(panel)),
        }
    }

    /// Altavoz blanco junto a la barra, según nivel y silencio.
    /// Es el botón que silencia.
    pub fn speaker(&self, level: f32, is_muted: bool) -> &Arc<Drawing> {
        match level_kind(level, is_muted) {
            LevelKind::Muted => &self.muted,
            LevelKind::Off => &self.off,
            LevelKind::Low => &self.low,
            LevelKind::High => &self.high,
        }
    }

    /// Altavoz blanco de la barra superior, según nivel y silencio.
    pub fn bar(&self, level: f32, is_muted: bool) -> &Arc<Drawing> {
        match level_kind(level, is_muted) {
            LevelKind::Muted => &self.bar_muted,
            LevelKind::Off => &self.bar_off,
            LevelKind::Low => &self.bar_low,
            LevelKind::High => &self.bar_high,
        }
    }

    /// Altavoz bajo a la izquierda del deslizador, atenuado con silencio.
    pub(crate) fn slider_left(&self, is_muted: bool) -> &Arc<Drawing> {
        if is_muted {
            &self.slider_low_dim
        } else {
            &self.slider_low
        }
    }

    /// Altavoz con ondas a la derecha del deslizador, atenuado con silencio.
    pub(crate) fn slider_right(&self, is_muted: bool) -> &Arc<Drawing> {
        if is_muted {
            &self.slider_high_dim
        } else {
            &self.slider_high
        }
    }

    /// Marca de verificación de la salida activa, a la izquierda.
    pub(crate) fn output_check(&self) -> &Arc<Drawing> {
        &self.check
    }

    /// Chevrón blanco de la fila de ajustes.
    pub fn chevron(&self) -> &Arc<Drawing> {
        &self.chevron
    }
}

enum LevelKind {
    Muted,
    Off,
    Low,
    High,
}

fn level_kind(level: f32, is_muted: bool) -> LevelKind {
    if is_muted {
        LevelKind::Muted
    } else if level.is_nan() || level <= 0.0 {
        LevelKind::Off
    } else if level < 0.5 {
        LevelKind::Low
    } else {
        LevelKind::High
    }
}
