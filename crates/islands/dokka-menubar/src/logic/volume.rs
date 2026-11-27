//! Panel de volumen: datos, salidas, apertura, dibujo y gestos.

use dokka_components::volume_panel::{
    VolumePanelHit, VolumePanelProps, volume_panel, volume_panel_hit, volume_panel_size,
    volume_slider_value,
};
use dokka_model::AudioOutput;
use dokka_scene::Overlay;
use dokka_theme::menubar as bar_measures;
use dokka_theme::volume as measures;

use super::tray::tray_place;
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Nivel y silencio que muestra el panel (el shell los lee cada `Tick`).
    pub fn set_volume(&mut self, level: f32, is_muted: bool) {
        self.volume.level = level;
        self.volume.is_muted = is_muted;
    }

    /// Salidas activas para la sección "Salida"; recoloca el alto si el
    /// panel está abierto. Guarda como máximo seis, con la predeterminada
    /// siempre visible.
    pub fn set_audio_outputs(&mut self, outputs: Vec<AudioOutput>) {
        let default = outputs.iter().find(|output| output.is_default).cloned();
        let mut kept = outputs;
        if kept.len() > measures::MAX_OUTPUTS {
            kept.truncate(measures::MAX_OUTPUTS);
        }
        if !kept.iter().any(|output| output.is_default)
            && let Some(default) = default
            && let Some(last) = kept.last_mut()
        {
            *last = default;
        }
        self.audio_outputs = kept;
        if let Some(hovered) = self.volume_hovered {
            let still = match hovered {
                VolumePanelHit::Output(index) => index < visible_outputs(&self.audio_outputs),
                _ => true,
            };
            if !still {
                self.volume_hovered = None;
            }
        }
        if self.tray_kind() == Some(super::TrayKind::Volume) {
            let height = volume_panel_size(self.audio_outputs.len()).height;
            self.tray = self.tray.map(|open| super::OpenTray {
                panel: dokka_scene::Rect {
                    x: open.panel.x,
                    y: open.panel.y,
                    width: volume_panel_size(self.audio_outputs.len()).width,
                    height,
                },
                ..open
            });
        }
    }

    /// Identificador de la salida con este índice, si sigue en la lista.
    pub fn audio_output_id(&self, index: usize) -> Option<String> {
        self.audio_outputs.get(index).map(|output| output.id.clone())
    }

    /// Mide con su tamaño real (según las salidas) y lo coloca bajo su icono.
    pub fn open_volume(&mut self, bar_width: f32) {
        let Some(anchor) = self.tray_anchor(super::TrayKind::Volume, bar_width) else {
            return;
        };
        let size = volume_panel_size(self.audio_outputs.len());
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
        let count = self.audio_outputs.len();
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: bar_measures::HEIGHT,
            width: volume_panel_size(count).width,
            height: volume_panel_size(count).height,
        });
        volume_panel(
            &self.theme,
            &self.volume_icons,
            bounds,
            &self.volume_props(),
            &self.audio_outputs,
        )
    }

    /// Botón abajo: el deslizador fija el nivel ya, lo demás espera al clic.
    pub(crate) fn volume_press(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match volume_panel_hit(bounds, x, y, &self.audio_outputs) {
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
        match volume_panel_hit(bounds, x, y, &self.audio_outputs) {
            Some(VolumePanelHit::Slider(level)) => Some(MenubarAction::VolumeSetLevel(level)),
            Some(VolumePanelHit::Output(index)) => Some(MenubarAction::ChooseOutput(index)),
            Some(VolumePanelHit::Mute) => Some(MenubarAction::VolumeToggleMute),
            Some(VolumePanelHit::Settings) => Some(MenubarAction::OpenSoundSettings),
            None => None,
        }
    }

    pub(crate) fn volume_hover(&mut self, x: f32, y: f32) {
        let hit = self
            .tray_bounds()
            .and_then(|bounds| volume_panel_hit(bounds, x, y, &self.audio_outputs));
        // El valor bajo el cursor cambia al mover: para no repintar por
        // cada píxel, la huella solo recuerda si hay barra o no.
        self.volume_hovered = hit.map(|hit| match hit {
            VolumePanelHit::Slider(_) => VolumePanelHit::Slider(0.0),
            rest => rest,
        });
    }
}

/// Salidas visibles de la lista guardada.
fn visible_outputs(outputs: &[AudioOutput]) -> usize {
    measures::visible_outputs(outputs.len())
}
