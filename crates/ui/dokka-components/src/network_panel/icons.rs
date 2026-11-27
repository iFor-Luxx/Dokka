use std::sync::Arc;

use dokka_icons::{
    network_ethernet_icon, network_offline_icon, network_other_icon, network_wifi_icon,
};
use dokka_scene::Drawing;
use dokka_theme::Theme;

use super::NetworkType;

/// Dibujos de red ya construidos con los colores de un tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo.
#[derive(Debug)]
pub struct NetworkIcons {
    panel: [Arc<Drawing>; 4],
    bar: [Arc<Drawing>; 4],
}

impl NetworkIcons {
    pub fn new(theme: &Theme) -> Self {
        let panel = theme.menu.text;
        let bar = theme.menubar.text;
        Self {
            panel: [
                Arc::new(network_offline_icon(panel)),
                Arc::new(network_ethernet_icon(panel)),
                Arc::new(network_wifi_icon(panel)),
                Arc::new(network_other_icon(panel)),
            ],
            bar: [
                Arc::new(network_offline_icon(bar)),
                Arc::new(network_ethernet_icon(bar)),
                Arc::new(network_wifi_icon(bar)),
                Arc::new(network_other_icon(bar)),
            ],
        }
    }

    /// Dibujo oscuro para la cabecera del panel claro.
    pub fn for_panel(&self, kind: NetworkType) -> Arc<Drawing> {
        Arc::clone(&self.panel[kind_index(kind)])
    }

    /// Dibujo blanco para la barra, según el tipo de red.
    pub fn for_bar(&self, kind: NetworkType) -> Arc<Drawing> {
        Arc::clone(&self.bar[kind_index(kind)])
    }
}

fn kind_index(kind: NetworkType) -> usize {
    match kind {
        NetworkType::Offline => 0,
        NetworkType::Ethernet => 1,
        NetworkType::Wifi => 2,
        NetworkType::Other => 3,
    }
}
