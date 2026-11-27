use dokka_model::AudioOutput;
use dokka_scene::Rect;

use super::VolumePanelHit;
use super::layout::{
    mute_row, output_row, settings_row, slider_area, slider_touch, visible_count,
};
use crate::slider::{slider_hit, slider_track_bounds, slider_value_at};

/// Zona tocada: salida, silenciar, ajustes o valor sobre la barra fina.
/// La zona de toque es la fila entera, pero el valor siempre sale del
/// componente `slider` con su misma geometría del dibujo.
pub fn volume_panel_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    outputs: &[AudioOutput],
) -> Option<VolumePanelHit> {
    for index in 0..visible_count(outputs) {
        if contains(output_row(bounds, index), x, y) {
            return Some(VolumePanelHit::Output(index));
        }
    }
    if contains(mute_row(bounds, outputs), x, y) {
        return Some(VolumePanelHit::Mute);
    }
    if contains(settings_row(bounds, outputs), x, y) {
        return Some(VolumePanelHit::Settings);
    }
    let area = slider_area(slider_touch(bounds));
    if let Some(level) = slider_hit(area, x, y) {
        return Some(VolumePanelHit::Slider(level));
    }
    None
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    (rect.x..rect.x + rect.width).contains(&x) && (rect.y..rect.y + rect.height).contains(&y)
}

/// Nivel que corresponde a `x` mientras se arrastra, sobre la misma pista
/// que dibuja el deslizador: se queda en 0 o en 1 en los extremos.
pub fn volume_slider_value(bounds: Rect, x: f32) -> f32 {
    let area = slider_area(slider_touch(bounds));
    slider_value_at(slider_track_bounds(area), x)
}
