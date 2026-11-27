use std::sync::Arc;

use dokka_icons::{
    volume_high_icon, volume_low_icon, volume_mute_icon, volume_muted_icon, volume_off_icon,
    volume_settings_icon,
};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Altavoz del panel y de la barra ya construidos con los colores del tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo.
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
    mute: Arc<Drawing>,
    chevron: Arc<Drawing>,
}

impl VolumePanelIcons {
    pub fn new(theme: &Theme) -> Self {
        Self {
            muted: Arc::new(volume_muted_icon(theme.panel.text)),
            off: Arc::new(volume_off_icon(theme.panel.text)),
            low: Arc::new(volume_low_icon(theme.panel.text)),
            high: Arc::new(volume_high_icon(theme.panel.text)),
            bar_muted: Arc::new(volume_muted_icon(theme.menubar.text)),
            bar_off: Arc::new(volume_off_icon(theme.menubar.text)),
            bar_low: Arc::new(volume_low_icon(theme.menubar.text)),
            bar_high: Arc::new(volume_high_icon(theme.menubar.text)),
            mute: Arc::new(volume_mute_icon(theme.panel.text)),
            chevron: Arc::new(volume_settings_icon(theme.panel.text)),
        }
    }

    /// Altavoz oscuro junto a la barra, según nivel y silencio.
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

    /// Icono oscuro del botón de silencio.
    pub fn mute(&self) -> &Arc<Drawing> {
        &self.mute
    }

    /// Chevrón oscuro de la fila de ajustes.
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
    } else if !(level > 0.0) {
        LevelKind::Off
    } else if level < 0.5 {
        LevelKind::Low
    } else {
        LevelKind::High
    }
}
