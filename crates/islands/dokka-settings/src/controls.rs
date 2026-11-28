//! Clic y teclado de Preferencias: semáforos, casilla, píldora, filas,
//! interruptores y deslizadores sin arrastrar.

use dokka_components::nav_pill::NavHalf;

use super::datetime_click::datetime_edit_action;
use super::general::{AboutButton, GeneralPage};
use super::logic::{
    DockSwitch, SliderTarget, about_button_at, datetime_button_at, dock_segment_at,
    dock_slider_at, dock_switch_at, general_row_at, icon_scale_from, language_button_at,
    language_segment_at, magnification_from, menubar_switch_at, position_from_index,
    storage_button_at, taskbar_switch_at, toggle_menubar,
};
use super::slider_pointer::clamped_slider_value;
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
                dokka_components::window_controls::TrafficButton::Zoom => SettingsAction::Zoom,
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
        let general = self.general();
        let width = self.content_width();
        let scroll = self.scroll_offset();
        let counts = self.list_counts();
        // Interruptores del Dock: hábitos y ocultado.
        if let Some(switch) = dock_switch_at(selected, width, x, y) {
            let mut preferences = self.values().preferences;
            match switch {
                DockSwitch::Magnification => {
                    preferences.dock.magnification = !preferences.dock.magnification;
                }
                DockSwitch::AutoHide => {
                    preferences.dock.auto_hide = !preferences.dock.auto_hide;
                }
                DockSwitch::AnimateLaunch => {
                    preferences.dock.animate_launch = !preferences.dock.animate_launch;
                }
                DockSwitch::ShowIndicators => {
                    preferences.dock.show_indicators = !preferences.dock.show_indicators;
                }
                DockSwitch::Previews => {
                    preferences.dock.previews = !preferences.dock.previews;
                }
            }
            self.values_mut().preferences = preferences;
            return SettingsAction::Changed(preferences);
        }
        // Interruptores de la barra superior: la fila de arriba es la de más a
        // la izquierda, así que el interruptor sigue al icono en su orden.
        if let Some(row) = menubar_switch_at(selected, width, x, y) {
            let order = self.values().preferences.menubar.order;
            let Some(item) = order.get(row.min(6)).copied() else {
                return SettingsAction::Stay;
            };
            let mut preferences = self.values().preferences;
            toggle_menubar(&mut preferences.menubar, item);
            self.values_mut().preferences = preferences;
            return SettingsAction::Changed(preferences);
        }
        // Interruptor de la barra de tareas.
        if taskbar_switch_at(selected, width, x, y).is_some() {
            let auto = !self.values().taskbar_auto;
            self.values_mut().taskbar_auto = auto;
            return SettingsAction::TaskbarAuto(auto);
        }
        // Segmentado de posición: el componente da el índice, la isla traduce.
        if let Some(index) = dock_segment_at(selected, width, x, y) {
            let mut preferences = self.values().preferences;
            preferences.dock.position = position_from_index(index);
            self.values_mut().preferences = preferences;
            return SettingsAction::Changed(preferences);
        }
        // Clic en un deslizador sin arrastrar: salta hasta el punto y emite.
        if let Some(target) = dock_slider_at(selected, width, x, y) {
            if target == SliderTarget::DockStrength && !self.values().preferences.dock.magnification
            {
                return SettingsAction::Stay;
            }
            let value = clamped_slider_value(target, self.values(), width, x);
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
        // Filas de la lista de General: entran en su apartado.
        if let Some(row) = general_row_at(selected, general, width, scroll, x, y) {
            let Some(page) = GeneralPage::from_row(row) else {
                return SettingsAction::Stay;
            };
            self.enter_general(page);
            return SettingsAction::EnterGeneral(page);
        }
        // Botones de "Acerca de este PC": el shell abre la página.
        if let Some(button) = about_button_at(selected, general, width, scroll, x, y) {
            let action = match button {
                0 => AboutButton::DisplaySettings,
                1 => AboutButton::StorageSettings,
                _ => AboutButton::SystemReport,
            };
            return SettingsAction::OpenAbout(action);
        }
        // Controles nuevos de fecha y 24 horas, propios y de Windows.
        if let Some(action) = datetime_edit_action(self, x, y) {
            return action;
        }
        // Segmentado de "Idioma y región": primer día de la semana.
        if let Some(index) = language_segment_at(general, width, counts.languages, scroll, x, y) {
            let mut preferences = self.values().preferences;
            preferences.general.week_starts_monday = index == 1;
            self.values_mut().preferences = preferences;
            return SettingsAction::Changed(preferences);
        }
        // Botones de "Fecha y hora" e "Idioma y región": el shell abre Windows.
        if let Some(button) = datetime_button_at(general, width, scroll, x, y)
            .or_else(|| language_button_at(general, width, counts.languages, scroll, x, y))
        {
            return SettingsAction::OpenGeneral(button);
        }
        // Botones de "Almacenamiento": vaciar la papelera o abrir sus ajustes.
        if let Some(button) =
            storage_button_at(selected, general, width, scroll, counts.disks, x, y)
        {
            if button == 0 {
                return SettingsAction::EmptyTrash;
            }
            return SettingsAction::OpenAbout(AboutButton::StorageSettings);
        }
        if let Some(index) = self.filtered_at(x, y) {
            self.select_section(index);
        }
        SettingsAction::Stay
    }

    /// Letra escrita con la casilla enfocada: filtra desde arriba.
    pub fn type_character(&mut self, character: char) {
        if self.general() == GeneralPage::DateTime && self.datetime_edit.server_editing {
            self.datetime_edit_mut().push_server(character);
            return;
        }
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
        if self.general() == GeneralPage::DateTime && self.datetime_edit.server_editing {
            self.datetime_edit_mut().pop_server();
            return;
        }
        if self.search_focused() {
            self.query_mut().pop();
        }
    }

    /// Enter con búsqueda: elige la primera coincidencia.
    pub fn confirm_search(&mut self) {
        if self.general() == GeneralPage::DateTime && self.datetime_edit.server_editing {
            return;
        }
        if !self.search_focused() || self.query().is_empty() {
            return;
        }
        if let Some(first) = self.filtered_order().into_iter().next() {
            self.select_section(first);
        }
    }

    /// Esc: limpia lo escrito y quita el foco.
    pub fn escape_search(&mut self) {
        if self.general() == GeneralPage::DateTime && self.datetime_edit.server_editing {
            self.datetime_edit_mut().cancel_server();
            return;
        }
        if self.general() == GeneralPage::DateTime && self.datetime_edit.manual_editing {
            self.datetime_edit_mut().cancel_manual();
            return;
        }
        if self.search_focused() || !self.query().is_empty() {
            self.query_mut().clear();
            self.set_search_focused(false);
        }
    }
}
