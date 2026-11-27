use dokka_scene::Rect;

use super::ControlCenterHit;
use super::layout::{
    brightness_card, circle_slot, music_button, music_tile, network_capsule, prefs_capsule,
    slider_area, sound_card, sound_mute_box,
};
use super::props::ControlCenterProps;
use crate::slider::{slider_hit, slider_track_bounds, slider_value_at};

/// Zona pulsable bajo el punto: primero los ocho círculos y los botones
/// de música, luego la Pantalla si hay brillo, el silencio y el deslizador
/// de Sonido (cada uno con su valor 0 a 1) y al final las cápsulas de red y
/// Preferencias. Dibujo, golpe y arrastre usan la misma geometría del
/// componente `slider`.
pub fn control_center_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    props: &ControlCenterProps,
) -> Option<ControlCenterHit> {
    let with_brightness = props.brightness.is_some();
    for (index, hit) in [
        ControlCenterHit::Desktop,
        ControlCenterHit::Lock,
        ControlCenterHit::TaskView,
        ControlCenterHit::Project,
        ControlCenterHit::Calculator,
        ControlCenterHit::ScreenClip,
        ControlCenterHit::Clock,
        ControlCenterHit::TaskManager,
    ]
    .iter()
    .enumerate()
    {
        if contains(circle_slot(bounds, index, with_brightness), x, y) {
            return Some(*hit);
        }
    }
    let music = music_tile(bounds);
    for (index, hit) in [
        ControlCenterHit::Previous,
        ControlCenterHit::Toggle,
        ControlCenterHit::Next,
    ]
    .iter()
    .enumerate()
    {
        if contains(music_button(music, index), x, y) {
            return Some(*hit);
        }
    }
    if with_brightness {
        let area = slider_area(brightness_card(bounds));
        if let Some(level) = slider_hit(area, x, y) {
            return Some(ControlCenterHit::Brightness(level));
        }
    }
    let sound = sound_card(bounds, with_brightness);
    if contains(sound_mute_box(sound), x, y) {
        return Some(ControlCenterHit::Mute);
    }
    let area = slider_area(sound);
    if let Some(level) = slider_hit(area, x, y) {
        return Some(ControlCenterHit::Slider(level));
    }
    if contains(network_capsule(bounds), x, y) {
        return Some(ControlCenterHit::Network);
    }
    if contains(prefs_capsule(bounds), x, y) {
        return Some(ControlCenterHit::Preferences);
    }
    None
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}

/// Nivel de Sonido que corresponde a `x` mientras se arrastra, aunque el
/// cursor se pase de la barra: se queda en 0 o en 1.
pub fn control_center_slider_value(bounds: Rect, x: f32, brightness: Option<f32>) -> f32 {
    let with_brightness = brightness.is_some();
    let area = slider_area(sound_card(bounds, with_brightness));
    slider_value_at(slider_track_bounds(area), x)
}

/// Nivel de Pantalla que corresponde a `x` mientras se arrastra, aunque el
/// cursor se pase de la barra: se queda en 0 o en 1. Misma geometría que
/// Sonido, en su tarjeta de encima.
pub fn control_center_brightness_value(bounds: Rect, x: f32) -> f32 {
    let area = slider_area(brightness_card(bounds));
    slider_value_at(slider_track_bounds(area), x)
}
