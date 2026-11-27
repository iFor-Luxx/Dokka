//! Clic y teclado de Preferencias: semáforos, casilla, píldora, filas,
//! interruptores y deslizadores sin arrastrar.

use dokka_components::nav_pill::NavHalf;

use super::logic::{
    SliderTarget, dock_slider_at, dock_switch_at, icon_scale_from, magnification_from,
    menubar_switch_at, taskbar_switch_at,
};
use super::pointer::clamped_slider_value;
use super::state::Settings;
use crate::SettingsAction;

impl Settings {
    /// Aplica el clic: semáforos, foco de la casilla, píldora, fila visible o
    /// controles del contenido. Sin arrastre a medias.
    pub fn click(&mut self, x: f32, y: f32) -> SettingsAction {
        if let Some(button) = Self::traffic_button_at(x, y) {
            return match button {
                dokka_components::window_controls::TrafficButton::Close => SettingsAction::Close,
                dokka_components::window_controls::TrafficButton::Minimize => {
                    SettingsAction::Minimize
                }
                dokka_components::window_controls::TrafficButton::Zoom => SettingsAction::Stay,
            };
        }
        if Self::search_hit_at(x, y) {
            self.set_search_focused(true);
            return SettingsAction::Stay;
        }
        self.clear_search_state();
        if let Some(half) = self.pill_at(x, y) {
            match half {
                NavHalf::Previous => self.go_back(),
                NavHalf::Next => self.go_forward(),
            }
            return SettingsAction::Stay;
        }
        let selected = self.selected();
        // Interruptores del Dock: ampliación y vistas previas.
        if let Some(card) = dock_switch_at(selected, x, y) {
            let mut preferences = self.values().preferences;
            if card == 1 {
                preferences.dock.magnification = !preferences.dock.magnification;
            } else if card == 3 {
                preferences.dock.previews = !preferences.dock.previews;
            } else {
                return SettingsAction::Stay;
            }
            self.values_mut().preferences = preferences;
            return SettingsAction::Changed(preferences);
        }
        // Interruptores de la barra superior.
        if let Some(row) = menubar_switch_at(selected, x, y) {
            let mut preferences = self.values().preferences;
            match row {
                0 => preferences.menubar.volume = !preferences.menubar.volume,
                1 => preferences.menubar.network = !preferences.menubar.network,
                2 => preferences.menubar.music = !preferences.menubar.music,
                3 => preferences.menubar.language = !preferences.menubar.language,
                4 => preferences.menubar.search = !preferences.menubar.search,
                5 => preferences.menubar.control_center = !preferences.menubar.control_center,
                6 => preferences.menubar.background_apps = !preferences.menubar.background_apps,
                _ => return SettingsAction::Stay,
            }
            self.values_mut().preferences = preferences;
            return SettingsAction::Changed(preferences);
        }
        // Interruptor de la barra de tareas.
        if taskbar_switch_at(selected, x, y).is_some() {
            let auto = !self.values().taskbar_auto;
            self.values_mut().taskbar_auto = auto;
            return SettingsAction::TaskbarAuto(auto);
        }
        // Clic en un deslizador sin arrastrar: salta hasta el punto y emite.
        if let Some(target) = dock_slider_at(selected, x, y) {
            if target == SliderTarget::DockStrength
                && !self.values().preferences.dock.magnification
            {
                return SettingsAction::Stay;
            }
            let value = clamped_slider_value(target, self.values(), x);
            let mut preferences = self.values().preferences;
            match target {
                SliderTarget::DockScale => {
                    preferences.dock.icon_scale = icon_scale_from(value);
                }
                SliderTarget::DockStrength => {
                    preferences.dock.magnification_strength = magnification_from(value);
                }
            }
            self.values_mut().preferences = preferences;
            return SettingsAction::Changed(preferences);
        }
        if let Some(index) = self.filtered_at(x, y) {
            self.select_section(index);
        }
        SettingsAction::Stay
    }

    /// Letra escrita con la casilla enfocada: filtra desde arriba.
    pub fn type_character(&mut self, character: char) {
        if !self.search_focused() {
            return;
        }
        if self.query().chars().count() >= dokka_theme::settings::SEARCH_MAX_CHARACTERS {
            return;
        }
        self.query_mut().push(character);
    }

    /// Retroceso con la casilla enfocada: borra la última letra.
    pub fn erase_character(&mut self) {
        if self.search_focused() {
            self.query_mut().pop();
        }
    }

    /// Enter con búsqueda: elige la primera coincidencia.
    pub fn confirm_search(&mut self) {
        if !self.search_focused() || self.query().is_empty() {
            return;
        }
        if let Some(first) = self.filtered_order().into_iter().next() {
            self.select_section(first);
        }
    }

    /// Esc: limpia lo escrito y quita el foco.
    pub fn escape_search(&mut self) {
        if self.search_focused() || !self.query().is_empty() {
            self.query_mut().clear();
            self.set_search_focused(false);
        }
    }
}
