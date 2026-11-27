use dokka_scene::Rect;

use super::ControlCenterHit;
use super::layout::{
    music_button, music_card, mute_area, network_card, round_button, slider_area, slider_track,
    slider_value, volume_card,
};
use super::props::ControlCenterProps;

/// Zona pulsable bajo el punto: primero los botones redondos y los de
/// música, luego el silencio y el deslizador (con su valor 0 a 1) y al
/// final la tarjeta de red.
pub fn control_center_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    _props: &ControlCenterProps,
) -> Option<ControlCenterHit> {
    for (index, hit) in [
        ControlCenterHit::Preferences,
        ControlCenterHit::Desktop,
        ControlCenterHit::Lock,
        ControlCenterHit::TaskManager,
    ]
    .iter()
    .enumerate()
    {
        if contains(round_button(bounds, index), x, y) {
            return Some(*hit);
        }
    }
    let music = music_card(bounds);
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
    let volume = volume_card(bounds);
    if contains(mute_area(volume), x, y) {
        return Some(ControlCenterHit::Mute);
    }
    let track = slider_track(volume);
    if contains(slider_area(volume), x, y) {
        return Some(ControlCenterHit::Slider(slider_value(track, x)));
    }
    if contains(network_card(bounds), x, y) {
        return Some(ControlCenterHit::Network);
    }
    None
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
