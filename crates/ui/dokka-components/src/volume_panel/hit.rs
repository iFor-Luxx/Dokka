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
