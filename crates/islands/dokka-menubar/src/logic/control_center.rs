//! Centro de control: combina red, volumen y música con sus botones y carátula.

use dokka_components::control_center_panel::{
    ControlCenterHit, ControlCenterProps, control_center_hit, control_center_panel,
    control_center_size, control_center_slider_value,
};
use dokka_scene::Overlay;

use super::tray::{tray_anchor, tray_place};
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Mide con su tamaño fijo y lo coloca bajo su icono.
    /// Reutiliza los datos de volumen, red y música ya guardados.
    pub fn open_control_center(&mut self, bar_width: f32) {
        let Some(anchor) = tray_anchor(super::TrayKind::ControlCenter, bar_width) else {
            return;
        };
        self.show_tray(
            super::TrayKind::ControlCenter,
            tray_place(anchor, control_center_size(), bar_width),
        );
    }

    pub(crate) fn control_props(&self) -> ControlCenterProps {
        ControlCenterProps {
            network: self.network.clone(),
            volume: self.volume,
            now_playing: self.now_playing.clone(),
            artwork: self.artwork(),
        }
    }

    pub(crate) fn control_overlay(&self) -> Overlay {
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: dokka_theme::menubar::HEIGHT,
            width: control_center_size().width,
            height: control_center_size().height,
        });
        control_center_panel(
            &self.theme,
            &self.control_icons,
            bounds,
            &self.control_props(),
        )
    }

    /// Botón abajo en el deslizador: fija el nivel ya, como en volumen.
    pub(crate) fn control_press(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match control_center_hit(bounds, x, y, &self.control_props()) {
            Some(ControlCenterHit::Slider(level)) => {
                self.dragging_slider = true;
                Some(MenubarAction::VolumeSetLevel(level))
            }
            _ => None,
        }
    }

    /// Arrastrando desde la barra del centro: el nivel sigue al cursor.
    pub(crate) fn control_drag(&mut self, x: f32) -> Option<MenubarAction> {
        if !self.dragging_slider {
            return None;
        }
        let bounds = self.tray_bounds()?;
        Some(MenubarAction::VolumeSetLevel(control_center_slider_value(
            bounds, x,
        )))
    }

    /// El resto delega en las variantes de cada panel; la tarjeta de red
    /// abre el panel de red sin pasar por el shell. Los círculos nuevos
    /// devuelven su acción para que el shell los abra con el lanzador.
    pub(crate) fn control_click(
        &mut self,
        x: f32,
        y: f32,
        bar_width: f32,
    ) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        let hit = control_center_hit(bounds, x, y, &self.control_props());
        match hit {
            Some(ControlCenterHit::Slider(level)) => Some(MenubarAction::VolumeSetLevel(level)),
            Some(ControlCenterHit::Mute) => Some(MenubarAction::VolumeToggleMute),
            Some(ControlCenterHit::Previous) => Some(MenubarAction::MusicPrevious),
            Some(ControlCenterHit::Toggle) => Some(MenubarAction::MusicToggle),
            Some(ControlCenterHit::Next) => Some(MenubarAction::MusicNext),
            Some(ControlCenterHit::Preferences) => Some(MenubarAction::OpenSettings),
            Some(ControlCenterHit::Desktop) => Some(MenubarAction::ShowDesktop),
            Some(ControlCenterHit::Lock) => Some(MenubarAction::LockScreen),
            Some(ControlCenterHit::TaskView) => Some(MenubarAction::TaskView),
            Some(ControlCenterHit::Project) => Some(MenubarAction::Project),
            Some(ControlCenterHit::Calculator) => Some(MenubarAction::Calculator),
            Some(ControlCenterHit::ScreenClip) => Some(MenubarAction::ScreenClip),
            Some(ControlCenterHit::Clock) => Some(MenubarAction::Clock),
            Some(ControlCenterHit::TaskManager) => Some(MenubarAction::TaskManager),
            Some(ControlCenterHit::Network) => {
                self.open_network(bar_width);
                None
            }
            None => None,
        }
    }
}
