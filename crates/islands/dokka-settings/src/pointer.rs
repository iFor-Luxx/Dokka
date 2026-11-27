//! Puntero de Preferencias: hover, arrastre de deslizadores y suelta.
//!
//! Los deslizadores se agarran al pulsar y siguen al cursor hasta soltar:
//! mientras tanto la isla solo redibuja. Al soltar emite `Changed` una vez.
//! El valor al pulsar y al arrastrar sale de la misma pista que el dibujo.

use dokka_model::Preferences;

use super::logic::{
    DockSwitch, SliderTarget, card_heights, content_slider_track, dock_slider_at,
    dock_switch_at, icon_scale_from, icon_scale_to, magnification_from, magnification_to,
    menubar_switch_at, taskbar_switch_at,
};
use super::painted::PaintedControl;
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
    shown
}

/// Control bajo el punto en la sección elegida, si hay alguno.
pub(crate) fn control_at(selected: usize, x: f32, y: f32) -> Option<PaintedControl> {
    if let Some(switch) = dock_switch_at(selected, x, y) {
        let card = match switch {
            DockSwitch::Magnification => 0,
            DockSwitch::Previews => 1,
        };
        return Some(PaintedControl::DockSwitch(card));
    }
    if let Some(row) = menubar_switch_at(selected, x, y) {
        return Some(PaintedControl::MenubarSwitch(row));
    }
    if taskbar_switch_at(selected, x, y).is_some() {
        return Some(PaintedControl::TaskbarSwitch);
    }
    match dock_slider_at(selected, x, y) {
        Some(SliderTarget::DockScale) => Some(PaintedControl::DockSlider(0)),
        Some(SliderTarget::DockStrength) => Some(PaintedControl::DockSlider(1)),
        None => None,
    }
}

/// Pista del deslizador agarrado: la misma que pinta el contenido.
fn drag_track(target: SliderTarget) -> Option<dokka_scene::Rect> {
    let tops = super::logic::card_tops(super::logic::SECTION_DOCK);
    let top = tops.first()?;
    let heights = card_heights(super::logic::SECTION_DOCK, 0);
    match target {
        SliderTarget::DockScale => Some(content_slider_track(*top, &heights, 0)),
        SliderTarget::DockStrength => Some(content_slider_track(*top, &heights, 2)),
    }
}

/// Valor de 0 a 1 del deslizador agarrado bajo este `x`, aunque el cursor se
/// pase de la barra: se queda en 0 o en 1.
pub(crate) fn clamped_slider_value(target: SliderTarget, values: &ContentValues, x: f32) -> f32 {
    let Some(track) = drag_track(target) else {
        return values.drag_value;
    };
    let inside = x.clamp(track.x, track.x + track.width);
    slider_value_at_x_for(track, inside, values.drag_value)
}

pub(crate) fn slider_value_at_x_for(track: dokka_scene::Rect, x: f32, fallback: f32) -> f32 {
    use dokka_components::slider::slider_value_at;
    if track.width <= 0.0 {
        return fallback;
    }
    slider_value_at(track, x)
}

impl Settings {
    /// Resalta la fila visible, los semáforos, la píldora y el control bajo el
    /// punto. Si hay un deslizador agarrado, sigue al cursor.
    pub fn pointer_moved(&mut self, x: f32, y: f32) {
        self.set_hovered(self.filtered_at(x, y));
        self.set_traffic_hovered(Self::traffic_button_at(x, y).is_some());
        self.set_nav_hovered(self.pill_at(x, y));
        if let Some(target) = self.values().drag {
            let value = clamped_slider_value(target, self.values(), x);
            self.values_mut().drag_value = value;
            self.values_mut().control = match target {
                SliderTarget::DockScale => Some(PaintedControl::DockSlider(0)),
                SliderTarget::DockStrength => Some(PaintedControl::DockSlider(1)),
            };
        } else {
            self.values_mut().control = control_at(self.selected(), x, y);
        }
    }

    pub fn pointer_left(&mut self) {
        self.set_hovered(None);
        self.set_nav_hovered(None);
        self.set_traffic_hovered(false);
        // Con el botón agarrado el movimiento sigue llegando fuera: no se
        // pierde el arrastre al salir.
        if self.values().drag.is_none() {
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
        }
    }

    /// Botón abajo: si cae en un deslizador, lo agarra y sigue al cursor.
    pub fn press(&mut self, x: f32, y: f32) -> bool {
        let selected = self.selected();
        let Some(target) = dock_slider_at(selected, x, y) else {
            return false;
        };
        // La cantidad atenuada no responde: pulsar no la agarra.
        if target == SliderTarget::DockStrength && !self.values().preferences.dock.magnification {
            return false;
        }
        let start = match target {
            SliderTarget::DockScale => icon_scale_to(self.values().preferences.dock.icon_scale),
            SliderTarget::DockStrength => {
                magnification_to(self.values().preferences.dock.magnification_strength)
            }
        };
        let mut value = start;
        if let Some(track) = drag_track(target) {
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
        true
    }

    /// Movimiento con el botón abajo: sigue al deslizador agarrado.
    pub fn drag_to(&mut self, x: f32, y: f32) {
        self.pointer_moved(x, y);
    }

    /// Suelta el botón: si había un deslizador agarrado, emite `Changed` una
    /// sola vez con el valor final; si no, resuelve el clic normal.
    pub fn release(&mut self, x: f32, y: f32) -> SettingsAction {
        if let Some(target) = self.values().drag {
            // Suelta fuera de la barra: se queda en el extremo más cercano.
            let value = clamped_slider_value(target, self.values(), x);
            self.values_mut().drag = None;
            self.values_mut().control = control_at(self.selected(), x, y);
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
        self.click(x, y)
    }
}
