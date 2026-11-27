use std::sync::Arc;

use dokka_icons::{
    control_center_bar_icon, control_center_desktop_icon, control_center_ethernet_icon,
    control_center_globe_icon, control_center_lock_icon, control_center_muted_icon,
    control_center_next_icon, control_center_pause_icon, control_center_play_icon,
    control_center_preferences_icon, control_center_previous_icon,
    control_center_task_manager_icon, control_center_volume_icon, control_center_wifi_icon,
    control_center_wifi_off_icon,
};
use dokka_model::{NetworkKind, NetworkState};
use dokka_scene::Drawing;
use dokka_theme::Theme;

/// Iconos ya construidos con los colores de un tema. Se reutilizan entre
/// fotogramas: la plataforma solo repinta cuando recibe otro dibujo.
#[derive(Debug, Clone)]
pub struct ControlCenterIcons {
    /// Icono blanco de la barra superior (deslizadores Lucide). El resto son
    /// blancos de cristal salvo las variantes activas de cápsula (azul Tahoe
    /// sobre círculo blanco).
    pub bar: Arc<Drawing>,
    pub(crate) wifi: Arc<Drawing>,
    pub(crate) wifi_active: Arc<Drawing>,
    pub(crate) wifi_off: Arc<Drawing>,
    pub(crate) ethernet: Arc<Drawing>,
    pub(crate) ethernet_active: Arc<Drawing>,
    pub(crate) globe: Arc<Drawing>,
    pub(crate) globe_active: Arc<Drawing>,
    pub(crate) volume: Arc<Drawing>,
    pub(crate) muted: Arc<Drawing>,
    pub(crate) previous: Arc<Drawing>,
    pub(crate) play: Arc<Drawing>,
    pub(crate) pause: Arc<Drawing>,
    pub(crate) next: Arc<Drawing>,
    pub(crate) preferences: Arc<Drawing>,
    pub(crate) desktop_active: Arc<Drawing>,
    pub(crate) lock_active: Arc<Drawing>,
    pub(crate) task_manager_active: Arc<Drawing>,
}

impl ControlCenterIcons {
    pub fn new(theme: &Theme) -> Self {
        let ink = theme.tray.circle_icon;
        let active = theme.tray.circle_active_icon;
        Self {
            bar: Arc::new(control_center_bar_icon(theme.menubar.text)),
            wifi: Arc::new(control_center_wifi_icon(ink)),
            wifi_active: Arc::new(control_center_wifi_icon(active)),
            wifi_off: Arc::new(control_center_wifi_off_icon(ink)),
            ethernet: Arc::new(control_center_ethernet_icon(ink)),
            ethernet_active: Arc::new(control_center_ethernet_icon(active)),
            globe: Arc::new(control_center_globe_icon(ink)),
            globe_active: Arc::new(control_center_globe_icon(active)),
            volume: Arc::new(control_center_volume_icon(ink)),
            muted: Arc::new(control_center_muted_icon(ink)),
            previous: Arc::new(control_center_previous_icon(ink)),
            play: Arc::new(control_center_play_icon(ink)),
            pause: Arc::new(control_center_pause_icon(ink)),
            next: Arc::new(control_center_next_icon(ink)),
            preferences: Arc::new(control_center_preferences_icon(ink)),
            desktop_active: Arc::new(control_center_desktop_icon(active)),
            lock_active: Arc::new(control_center_lock_icon(active)),
            task_manager_active: Arc::new(control_center_task_manager_icon(active)),
        }
    }

    pub(crate) fn network(&self, network: &NetworkState) -> &Arc<Drawing> {
        match network.kind {
            NetworkKind::Offline => &self.wifi_off,
            NetworkKind::Ethernet => &self.ethernet,
            NetworkKind::Wifi => {
                if network.has_internet {
                    &self.wifi
                } else {
                    &self.wifi_off
                }
            }
            NetworkKind::Other => &self.globe,
        }
    }

    /// Glifo de la cápsula de red: azul Tahoe si hay conexión, blanco si no.
    pub(crate) fn network_glyph(&self, network: &NetworkState, active: bool) -> &Arc<Drawing> {
        if active {
            match network.kind {
                NetworkKind::Ethernet => &self.ethernet_active,
                NetworkKind::Other => &self.globe_active,
                NetworkKind::Wifi => &self.wifi_active,
                NetworkKind::Offline => &self.wifi_off,
            }
        } else {
            self.network(network)
        }
    }

    pub(crate) fn volume(&self, is_muted: bool) -> &Arc<Drawing> {
        if is_muted { &self.muted } else { &self.volume }
    }

    pub(crate) fn toggle(&self, is_playing: bool) -> &Arc<Drawing> {
        if is_playing { &self.pause } else { &self.play }
    }
}
