use dokka_model::ControlItem;
use dokka_scene::Rect;

use super::ControlCenterHit;
use super::layout::{arrange, music_button, slider_area, sound_mute_box};
use super::props::ControlCenterProps;
use crate::slider::{slider_hit, slider_track_bounds, slider_value_at};

/// Zona pulsable bajo el punto, solo entre los controles visibles: primero
/// los círculos y los botones de música, luego la Pantalla si se ve, el
/// silencio y el deslizador de Sonido y al final las cápsulas. Dibujo,
/// golpe y arrastre usan la misma geometría del componente `slider`.
pub fn control_center_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    props: &ControlCenterProps,
) -> Option<ControlCenterHit> {
    let layout = arrange(bounds, props.hidden, props.brightness.is_some());
    for (item, hit) in [
        (ControlItem::Desktop, ControlCenterHit::Desktop),
        (ControlItem::Lock, ControlCenterHit::Lock),
        (ControlItem::TaskView, ControlCenterHit::TaskView),
        (ControlItem::Project, ControlCenterHit::Project),
        (ControlItem::Calculator, ControlCenterHit::Calculator),
        (ControlItem::ScreenClip, ControlCenterHit::ScreenClip),
        (ControlItem::Clock, ControlCenterHit::Clock),
        (ControlItem::TaskManager, ControlCenterHit::TaskManager),
    ] {
        if let Some(slot) = layout.rect(item)
            && contains(slot, x, y)
        {
            return Some(hit);
        }
    }
    if let Some(music) = layout.rect(ControlItem::Music) {
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
    }
    if let Some(card) = layout.rect(ControlItem::Brightness) {
        let area = slider_area(card);
        if let Some(level) = slider_hit(area, x, y) {
            return Some(ControlCenterHit::Brightness(level));
        }
    }
    if let Some(sound) = layout.rect(ControlItem::Sound) {
        if contains(sound_mute_box(sound), x, y) {
            return Some(ControlCenterHit::Mute);
        }
        let area = slider_area(sound);
        if let Some(level) = slider_hit(area, x, y) {
            return Some(ControlCenterHit::Slider(level));
        }
    }
    if let Some(network) = layout.rect(ControlItem::Network)
        && contains(network, x, y)
    {
        return Some(ControlCenterHit::Network);
    }
    if let Some(prefs) = layout.rect(ControlItem::Settings)
        && contains(prefs, x, y)
    {
        return Some(ControlCenterHit::Preferences);
    }
    None
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}

/// Nivel de Sonido que corresponde a `x` mientras se arrastra, aunque el
/// cursor se pase de la barra: se queda en 0 o en 1. Sin Sonido visible
/// devuelve el nivel que ya había.
pub fn control_center_slider_value(bounds: Rect, x: f32, props: &ControlCenterProps) -> f32 {
    let layout = arrange(bounds, props.hidden, props.brightness.is_some());
    let Some(card) = layout.rect(ControlItem::Sound) else {
        return props.volume.level;
    };
    let area = slider_area(card);
    slider_value_at(slider_track_bounds(area), x)
}

/// Nivel de Pantalla que corresponde a `x` mientras se arrastra, aunque el
/// cursor se pase de la barra: se queda en 0 o en 1. Misma geometría que
/// Sonido, en su tarjeta de encima. Sin Pantalla visible no hay nivel.
pub fn control_center_brightness_value(bounds: Rect, x: f32, props: &ControlCenterProps) -> f32 {
    let layout = arrange(bounds, props.hidden, props.brightness.is_some());
    let Some(card) = layout.rect(ControlItem::Brightness) else {
        return props.brightness.unwrap_or(0.0);
    };
    let area = slider_area(card);
    slider_value_at(slider_track_bounds(area), x)
}
