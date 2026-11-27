use std::sync::Arc;

use dokka_icons::{
    network_ethernet_icon, network_offline_icon, network_other_icon, network_wifi_icon,
    tray_chevron_icon, tray_settings_icon,
};
use dokka_scene::Drawing;
use dokka_theme::Theme;

use super::NetworkType;

/// Dibujos de red ya construidos con los colores de un tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo.
#[derive(Debug)]
pub struct NetworkIcons {
    /// Icono azul sobre el círculo blanco activo, por tipo.
    active: [Arc<Drawing>; 4],
    /// Icono blanco sobre el círculo translúcido apagado, por tipo.
    inactive: [Arc<Drawing>; 4],
    bar: [Arc<Drawing>; 4],
    settings: Arc<Drawing>,
    chevron: Arc<Drawing>,
}

impl NetworkIcons {
    pub fn new(theme: &Theme) -> Self {
        let active_color = theme.tray.circle_active_icon;
        let inactive_color = theme.tray.circle_icon;
        let bar = theme.menubar.text;
        Self {
            active: [
                Arc::new(network_offline_icon(active_color)),
                Arc::new(network_ethernet_icon(active_color)),
                Arc::new(network_wifi_icon(active_color)),
                Arc::new(network_other_icon(active_color)),
            ],
            inactive: [
                Arc::new(network_offline_icon(inactive_color)),
                Arc::new(network_ethernet_icon(inactive_color)),
                Arc::new(network_wifi_icon(inactive_color)),
                Arc::new(network_other_icon(inactive_color)),
            ],
            bar: [
                Arc::new(network_offline_icon(bar)),
                Arc::new(network_ethernet_icon(bar)),
                Arc::new(network_wifi_icon(bar)),
                Arc::new(network_other_icon(bar)),
            ],
            settings: Arc::new(tray_settings_icon(inactive_color)),
            chevron: Arc::new(tray_chevron_icon(theme.tray.text)),
        }
    }

    /// Dibujo azul para el círculo blanco activo (se mantiene por
    /// compatibilidad: es el icono de la cabecera).
    pub fn for_panel(&self, kind: NetworkType) -> Arc<Drawing> {
        Arc::clone(&self.active[kind_index(kind)])
    }

    /// Icono del círculo según si la conexión está activa.
    pub fn for_circle(&self, kind: NetworkType, is_active: bool) -> Arc<Drawing> {
        if is_active {
            self.for_panel(kind)
        } else {
            Arc::clone(&self.inactive[kind_index(kind)])
        }
    }

    /// Engranaje blanco de la zona de ajustes.
    pub fn settings(&self) -> &Arc<Drawing> {
        &self.settings
    }

    /// Chevrón blanco de la zona de ajustes.
    pub fn chevron(&self) -> &Arc<Drawing> {
        &self.chevron
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
