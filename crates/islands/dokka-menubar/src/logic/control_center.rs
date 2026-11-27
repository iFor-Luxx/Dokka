//! Centro de control: combina red, volumen, brillo y música con sus botones.

use dokka_components::control_center_panel::{
    ControlCenterHit, ControlCenterProps, control_center_brightness_value, control_center_hit,
    control_center_panel, control_center_size, control_center_slider_value,
};
use dokka_scene::Overlay;

use super::tray::tray_place;
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Mide con su tamaño real (crece con brillo) y lo coloca bajo su icono.
    /// Reutiliza los datos de volumen, brillo, red y música ya guardados.
    pub fn open_control_center(&mut self, bar_width: f32) {
        let Some(anchor) = self.tray_anchor(super::TrayKind::ControlCenter, bar_width) else {
            return;
        };
        self.show_tray(
            super::TrayKind::ControlCenter,
            tray_place(anchor, control_center_size(self.brightness), bar_width),
        );
    }

    /// Brillo DDC de 0 a 1 (`None` sin DDC); recoloca el alto si el centro
    /// está abierto, como las salidas de audio en el volumen.
    pub fn set_brightness(&mut self, brightness: Option<f32>) {
        let changed = self.brightness != brightness;
        self.brightness = brightness;
        if changed && self.tray_kind() == Some(super::TrayKind::ControlCenter) {
            let size = control_center_size(self.brightness);
            self.tray = self.tray.map(|open| super::OpenTray {
                panel: dokka_scene::Rect {
                    x: open.panel.x,
                    y: open.panel.y,
                    width: size.width,
                    height: size.height,
                },
                ..open
            });
        }
    }

    pub(crate) fn control_props(&self) -> ControlCenterProps {
        ControlCenterProps {
            network: self.network.clone(),
            volume: self.volume,
            now_playing: self.now_playing.clone(),
            artwork: self.artwork(),
            brightness: self.brightness,
        }
    }

    pub(crate) fn control_overlay(&self) -> Overlay {
        let size = control_center_size(self.brightness);
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: dokka_theme::menubar::HEIGHT,
            width: size.width,
            height: size.height,
        });
        control_center_panel(
            &self.theme,
            &self.control_icons,
            bounds,
            &self.control_props(),
        )
    }

    /// Botón abajo en un deslizador: fija el nivel ya, como en volumen.
    /// Recuerda cuál se arrastra para que el movimiento siga al mismo.
    pub(crate) fn control_press(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match control_center_hit(bounds, x, y, &self.control_props()) {
            Some(ControlCenterHit::Slider(level)) => {
                self.dragging_slider = true;
                self.dragging_brightness = false;
                Some(MenubarAction::VolumeSetLevel(level))
            }
            Some(ControlCenterHit::Brightness(level)) => {
                self.dragging_slider = true;
                self.dragging_brightness = true;
                Some(MenubarAction::BrightnessSetLevel(level))
            }
            _ => None,
        }
    }

    /// Arrastrando desde el centro: el nivel sigue al cursor en su tarjeta.
    pub(crate) fn control_drag(&mut self, x: f32) -> Option<MenubarAction> {
        if !self.dragging_slider {
            return None;
        }
        let bounds = self.tray_bounds()?;
        if self.dragging_brightness {
            Some(MenubarAction::BrightnessSetLevel(
                control_center_brightness_value(bounds, x),
            ))
        } else {
            Some(MenubarAction::VolumeSetLevel(
                control_center_slider_value(bounds, x, self.brightness),
            ))
        }
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
            Some(ControlCenterHit::Brightness(level)) => {
                Some(MenubarAction::BrightnessSetLevel(level))
            }
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
