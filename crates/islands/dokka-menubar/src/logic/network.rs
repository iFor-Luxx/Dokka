//! Panel de red: datos reales, apertura, dibujo y clic.

use dokka_components::network_panel::{
    NetworkHit, NetworkType, network_hit, network_panel, network_size,
};
use dokka_model::{NetworkKind, NetworkState};
use dokka_scene::Overlay;

use super::tray::{tray_anchor, tray_place};
use crate::{Menubar, MenubarAction};

/// Tipo del componente según el tipo del modelo.
pub(crate) fn network_type(kind: NetworkKind) -> NetworkType {
    match kind {
        NetworkKind::Offline => NetworkType::Offline,
        NetworkKind::Ethernet => NetworkType::Ethernet,
        NetworkKind::Wifi => NetworkType::Wifi,
        NetworkKind::Other => NetworkType::Other,
    }
}

impl Menubar {
    /// Conexión en uso (el shell la lee cada pocos `Tick` y al abrir).
    pub fn set_network(&mut self, network: NetworkState) {
        self.network = network;
    }

    /// Mide con su tamaño real y lo coloca bajo su icono.
    pub fn open_network(&mut self, bar_width: f32) {
        let Some(anchor) = tray_anchor(super::TrayKind::Network, bar_width) else {
            return;
        };
        self.show_tray(
            super::TrayKind::Network,
            tray_place(anchor, network_size(), bar_width),
        );
        self.network_hovered = None;
    }

    pub(crate) fn network_overlay(&self) -> Overlay {
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: dokka_theme::menubar::HEIGHT,
            width: network_size().width,
            height: network_size().height,
        });
        network_panel(
            &self.theme,
            &self.network_icons,
            bounds,
            &self.network.name,
            network_type(self.network.kind),
            self.network.has_internet,
            self.network_hovered,
        )
    }

    pub(crate) fn network_click(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        network_hit(bounds, x, y).map(|NetworkHit::Settings| MenubarAction::OpenNetworkSettings)
    }

    pub(crate) fn network_hover(&mut self, x: f32, y: f32) {
        self.network_hovered = self
            .tray_bounds()
            .and_then(|bounds| network_hit(bounds, x, y));
    }
}
