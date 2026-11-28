//! Puntero de Preferencias: hover, arrastre de deslizadores y filas, suelta.
//!
//! Los deslizadores se agarran al pulsar y siguen al cursor hasta soltar;
//! las filas de la barra siguen al cursor en vertical tras pasar el umbral:
//! mientras tanto la isla solo redibuja. Al soltar emite `Changed` una vez.
//! El valor al pulsar y al arrastrar sale de la misma pista que el dibujo.

use dokka_model::Preferences;

use super::general::GeneralPage;
use super::logic::{
    DockSwitch, ListCounts, SliderTarget, about_button_at, datetime_button_at, datetime_switch_at,
    dock_slider_at, dock_switch_at, general_row_at, icon_scale_from, icon_scale_to,
    language_button_at, language_segment_at, magnification_from, magnification_to,
    menubar_switch_at, storage_button_at, taskbar_switch_at,
};
use super::painted::PaintedControl;
use super::slider_pointer::{clamped_slider_value, drag_track, slider_value_at_x_for};
use super::state::Settings;
use super::values::ContentValues;
use crate::SettingsAction;

/// Valores con el arrastre a medias aplicado, para dibujar mientras se
/// arrastra sin guardar todavía.
pub(crate) fn display_preferences(values: &ContentValues) -> Preferences {
    let mut shown = values.preferences;
    if let Some(target) = values.drag {
        match target {
            SliderTarget::DockScale => {
                shown.dock.icon_scale = icon_scale_from(values.drag_value);
            }
            SliderTarget::DockStrength => {
                shown.dock.magnification_strength = magnification_from(values.drag_value);
            }
        }
    }
    if let Some(drag) = values.reorder
        && drag.is_active()
    {
        shown.menubar.order = drag.display();
    }
    shown
}

/// Control bajo el punto en la sección elegida, si hay alguno.
pub(crate) fn control_at(
    selected: usize,
    general: GeneralPage,
    width: f32,
    scroll: f32,
    counts: ListCounts,
    x: f32,
    y: f32,
) -> Option<PaintedControl> {
    if let Some(switch) = dock_switch_at(selected, width, x, y) {
        let card = match switch {
            DockSwitch::Magnification => 0,
            DockSwitch::AutoHide => 1,
            DockSwitch::AnimateLaunch => 2,
            DockSwitch::ShowIndicators => 3,
            DockSwitch::Previews => 4,
        };
        return Some(PaintedControl::DockSwitch(card));
    }
    if let Some(row) = menubar_switch_at(selected, width, x, y) {
        return Some(PaintedControl::MenubarSwitch(row));
    }
    if taskbar_switch_at(selected, width, x, y).is_some() {
        return Some(PaintedControl::TaskbarSwitch);
    }
    if let Some(row) = general_row_at(selected, general, width, scroll, x, y) {
        return Some(PaintedControl::GeneralRow(row));
    }
    if let Some(button) = about_button_at(selected, general, width, scroll, x, y) {
        return Some(PaintedControl::AboutButton(button));
    }
    if datetime_switch_at(general, width, scroll, x, y).is_some() {
        return Some(PaintedControl::DateTimeSwitch);
    }
    if language_segment_at(general, width, counts.languages, scroll, x, y).is_some() {
        return Some(PaintedControl::LanguageSegment);
    }
    if let Some(button) = datetime_button_at(general, width, scroll, x, y)
        .or_else(|| language_button_at(general, width, counts.languages, scroll, x, y))
    {
        return Some(PaintedControl::GeneralButton(button.index()));
    }
    if let Some(button) = storage_button_at(selected, general, width, scroll, counts.disks, x, y) {
        return Some(PaintedControl::StorageButton(button));
    }
    match dock_slider_at(selected, width, x, y) {
        Some(SliderTarget::DockScale) => Some(PaintedControl::DockSlider(0)),
        Some(SliderTarget::DockStrength) => Some(PaintedControl::DockSlider(1)),
        None => None,
    }
}

impl Settings {
    /// Resalta la fila visible, los semáforos, la píldora y el control bajo el
    /// punto. Si hay un deslizador o una fila agarrados, siguen al cursor.
    pub fn pointer_moved(&mut self, x: f32, y: f32) {
        let width = self.content_width();
        let general = self.general();
        let scroll = self.scroll_offset();
        let counts = self.list_counts();
        self.set_hovered(self.filtered_at(x, y));
        self.set_traffic_hovered(Self::traffic_button_at(x, y).is_some());
        self.set_nav_hovered(self.pill_at(x, y));
        if let Some(target) = self.values().drag {
            let value = clamped_slider_value(target, self.values(), width, x);
            self.values_mut().drag_value = value;
            self.values_mut().control = match target {
                SliderTarget::DockScale => Some(PaintedControl::DockSlider(0)),
                SliderTarget::DockStrength => Some(PaintedControl::DockSlider(1)),
            };
        } else if self.values().reorder.is_some() {
            self.move_reorder(x, y);
            // Mientras la fila sigue al cursor no hay hover de interruptor.
            self.values_mut().control = if self.reorder_active() {
                None
            } else {
                control_at(self.selected(), general, width, scroll, counts, x, y)
            };
        } else {
            self.values_mut().control =
                control_at(self.selected(), general, width, scroll, counts, x, y);
        }
    }

    pub fn pointer_left(&mut self) {
        self.set_hovered(None);
        self.set_nav_hovered(None);
        self.set_traffic_hovered(false);
        // Con el botón agarrado el movimiento sigue llegando fuera: no se
        // pierde el arrastre al salir.
        if self.values().drag.is_none() && !self.reorder_active() {
            self.values_mut().control = None;
        }
    }

    /// La ventana ganó o perdió el foco: sin foco los semáforos van en gris
    /// y se apagan los resaltados.
    pub fn set_window_focused(&mut self, focused: bool) {
        if !focused {
            self.set_hovered(None);
            self.set_nav_hovered(None);
            self.set_traffic_hovered(false);
            self.values_mut().control = None;
            self.values_mut().drag = None;
            self.values_mut().reorder = None;
        }
    }

    /// Botón abajo: si cae en un deslizador, lo agarra; si cae en una fila de
    /// la barra fuera del interruptor, recuerda la fila para reordenar.
    pub fn press(&mut self, x: f32, y: f32) -> bool {
        let selected = self.selected();
        let width = self.content_width();
        if let Some(target) = dock_slider_at(selected, width, x, y) {
            // La cantidad atenuada no responde: pulsar no la agarra.
            if target == SliderTarget::DockStrength && !self.values().preferences.dock.magnification
            {
                self.remember_reorder_press(x, y);
                return false;
            }
            let start = match target {
                SliderTarget::DockScale => icon_scale_to(self.values().preferences.dock.icon_scale),
                SliderTarget::DockStrength => {
                    magnification_to(self.values().preferences.dock.magnification_strength)
                }
            };
            let mut value = start;
            if let Some(track) = drag_track(target, width) {
                let inside = x.clamp(track.x, track.x + track.width);
                value = slider_value_at_x_for(track, inside, start);
            }
            // Pulsar ya mueve el valor hasta el punto, como en el volumen.
            self.values_mut().drag = Some(target);
            self.values_mut().drag_value = value;
            self.values_mut().control = match target {
                SliderTarget::DockScale => Some(PaintedControl::DockSlider(0)),
                SliderTarget::DockStrength => Some(PaintedControl::DockSlider(1)),
            };
            return true;
        }
        self.remember_reorder_press(x, y);
        false
    }

    /// Movimiento con el botón abajo: sigue al deslizador o a la fila.
    pub fn drag_to(&mut self, x: f32, y: f32) {
        self.pointer_moved(x, y);
    }

    /// Suelta el botón: si había un deslizador o una fila agarrados, emite
    /// `Changed` una sola vez con el valor final; si no, resuelve el clic.
    pub fn release(&mut self, x: f32, y: f32) -> SettingsAction {
        let width = self.content_width();
        let general = self.general();
        let scroll = self.scroll_offset();
        let counts = self.list_counts();
        if let Some(target) = self.values().drag {
            // Suelta fuera de la barra: se queda en el extremo más cercano.
            let value = clamped_slider_value(target, self.values(), width, x);
            self.values_mut().drag = None;
            self.values_mut().control =
                control_at(self.selected(), general, width, scroll, counts, x, y);
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
        if self.values().reorder.is_some() {
            let was_active = self.reorder_active();
            let action = self.finish_reorder();
            self.values_mut().control =
                control_at(self.selected(), general, width, scroll, counts, x, y);
            if was_active {
                // Hubo arrastre: una sola vez y solo si cambió el orden.
                return action.unwrap_or(SettingsAction::Stay);
            }
            // Pulsar sin mover equivale a un clic simple.
            return self.click(x, y);
        }
        self.click(x, y)
    }

    /// Rueda sobre el contenido de General: baja o sube la ficha.
    pub fn scroll_wheel(&mut self, delta: i16) {
        if self.selected() != super::logic::SECTION_GENERAL {
            return;
        }
        // Un paso de rueda son 120 unidades: baja una fila de contenido.
        let step = f32::from(delta) / 120.0 * -dokka_theme::settings::CONTENT_ROW_HEIGHT;
        self.scroll_by(step);
    }
}
