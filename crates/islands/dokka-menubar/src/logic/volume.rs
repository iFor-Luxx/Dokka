//! Panel de volumen: datos, apertura, dibujo y gestos del deslizador.

use dokka_components::volume_panel::{
    VolumePanelHit, VolumePanelProps, volume_panel, volume_panel_hit, volume_panel_size,
    volume_slider_value,
};
use dokka_scene::Overlay;
use dokka_theme::menubar as bar_measures;

use super::tray::{tray_anchor, tray_place};
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Nivel y silencio que muestra el panel (el shell los lee cada `Tick`).
    pub fn set_volume(&mut self, level: f32, is_muted: bool) {
        self.volume.level = level;
        self.volume.is_muted = is_muted;
    }

    /// Mide con su tamaño real y lo coloca bajo su icono.
    pub fn open_volume(&mut self, bar_width: f32) {
        let Some(anchor) = tray_anchor(super::TrayKind::Volume, bar_width) else {
            return;
        };
        let size = volume_panel_size();
        self.show_tray(super::TrayKind::Volume, tray_place(anchor, size, bar_width));
        self.volume_hovered = None;
    }

    pub(crate) fn volume_props(&self) -> VolumePanelProps {
        VolumePanelProps {
            level: self.volume.level,
            is_muted: self.volume.is_muted,
            hovered: self.volume_hovered,
        }
    }

    pub(crate) fn volume_overlay(&self) -> Overlay {
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: bar_measures::HEIGHT,
            width: volume_panel_size().width,
            height: volume_panel_size().height,
        });
        volume_panel(
            &self.theme,
            &self.volume_icons,
            bounds,
            &self.volume_props(),
        )
    }

    /// Botón abajo: el deslizador fija el nivel ya, lo demás espera al clic.
    pub(crate) fn volume_press(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match volume_panel_hit(bounds, x, y) {
            Some(VolumePanelHit::Slider(level)) => {
                self.dragging_slider = true;
                self.volume_hovered = Some(VolumePanelHit::Slider(level));
                Some(MenubarAction::VolumeSetLevel(level))
            }
            _ => None,
        }
    }

    /// Arrastrando desde la barra: el nivel sigue al cursor en vivo.
    pub(crate) fn volume_drag(&mut self, x: f32) -> Option<MenubarAction> {
        if !self.dragging_slider {
            return None;
        }
        let bounds = self.tray_bounds()?;
        let level = volume_slider_value(bounds, x);
        self.volume_hovered = Some(VolumePanelHit::Slider(level));
        Some(MenubarAction::VolumeSetLevel(level))
    }

    pub(crate) fn volume_click(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match volume_panel_hit(bounds, x, y) {
            Some(VolumePanelHit::Slider(level)) => Some(MenubarAction::VolumeSetLevel(level)),
            Some(VolumePanelHit::Mute) => Some(MenubarAction::VolumeToggleMute),
            Some(VolumePanelHit::Settings) => Some(MenubarAction::OpenSoundSettings),
            None => None,
        }
    }

    pub(crate) fn volume_hover(&mut self, x: f32, y: f32) {
        let hit = self
            .tray_bounds()
            .and_then(|bounds| volume_panel_hit(bounds, x, y));
        // El valor bajo el cursor cambia al mover: para no repintar por
        // cada píxel, la huella solo recuerda si hay barra o no.
        self.volume_hovered = hit.map(|hit| match hit {
            VolumePanelHit::Slider(_) => VolumePanelHit::Slider(0.0),
            rest => rest,
        });
    }
}
