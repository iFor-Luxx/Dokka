use dokka_scene::Rect;

use super::ControlCenterHit;
use super::layout::{
    brightness_card, circle_slot, music_button, music_tile, network_capsule, prefs_capsule,
    slider_area, slider_track, slider_value, sound_card, sound_mute_box,
};
use super::props::ControlCenterProps;

/// Zona pulsable bajo el punto: primero los ocho círculos y los botones
/// de música, luego la Pantalla si hay brillo, el silencio y el deslizador
/// de Sonido (cada uno con su valor 0 a 1) y al final las cápsulas de red y
/// Preferencias. Alto, golpe y arrastre usan la misma geometría.
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
        let card = brightness_card(bounds);
        let track = slider_track(card);
        if contains(slider_area(card), x, y) {
            return Some(ControlCenterHit::Brightness(slider_value(track, x)));
        }
    }
    let sound = sound_card(bounds, with_brightness);
    if contains(sound_mute_box(sound), x, y) {
        return Some(ControlCenterHit::Mute);
    }
    let track = slider_track(sound);
    if contains(slider_area(sound), x, y) {
        return Some(ControlCenterHit::Slider(slider_value(track, x)));
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
    slider_value(
        slider_track(sound_card(bounds, with_brightness)),
        x,
    )
}

/// Nivel de Pantalla que corresponde a `x` mientras se arrastra, aunque el
/// cursor se pase de la barra: se queda en 0 o en 1. Misma geometría que
/// Sonido, en su tarjeta de encima.
pub fn control_center_brightness_value(bounds: Rect, x: f32) -> f32 {
    slider_value(slider_track(brightness_card(bounds)), x)
}
