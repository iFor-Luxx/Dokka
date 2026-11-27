use dokka_scene::Rect;

use crate::slider::slider_hit;

use super::VolumePanelHit;
use super::layout::{mute_row, settings_row, slider_touch};

/// Zona tocada: silenciar, ajustes o valor sobre la cápsula.
pub fn volume_panel_hit(bounds: Rect, x: f32, y: f32) -> Option<VolumePanelHit> {
    if contains(mute_row(bounds), x, y) {
        return Some(VolumePanelHit::Mute);
    }
    if contains(settings_row(bounds), x, y) {
        return Some(VolumePanelHit::Settings);
    }
    slider_hit(slider_touch(bounds), x, y).map(VolumePanelHit::Slider)
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    (rect.x..rect.x + rect.width).contains(&x) && (rect.y..rect.y + rect.height).contains(&y)
}

/// Nivel que corresponde a `x` mientras se arrastra, aunque el cursor se
/// pase de la barra: se queda en 0 o en 1.
pub fn volume_slider_value(bounds: Rect, x: f32) -> f32 {
    let touch = slider_touch(bounds);
    let inside = x.clamp(touch.x, touch.x + touch.width);
    slider_hit(touch, inside, touch.y + touch.height / 2.0).unwrap_or(if x < touch.x {
        0.0
    } else {
        1.0
    })
}
