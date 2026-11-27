use dokka_scene::Rect;

use super::ControlCenterHit;
use super::layout::{
    circle_slot, music_button, music_tile, network_capsule, prefs_capsule, slider_area,
    slider_track, slider_value, sound_card, sound_mute_box,
};
use super::props::ControlCenterProps;

/// Zona pulsable bajo el punto: primero los ocho círculos y los botones
/// de música, luego el silencio y el deslizador (con su valor 0 a 1) y al
/// final las cápsulas de red y Preferencias.
pub fn control_center_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    _props: &ControlCenterProps,
) -> Option<ControlCenterHit> {
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
        if contains(circle_slot(bounds, index), x, y) {
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
    let sound = sound_card(bounds);
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
