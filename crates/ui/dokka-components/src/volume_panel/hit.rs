use dokka_model::AudioOutput;
use dokka_scene::Rect;

use super::VolumePanelHit;
use super::layout::{
    mute_row, output_row, settings_row, slider_bar, slider_touch, slider_value_at, visible_count,
};

/// Zona tocada: salida, silenciar, ajustes o valor sobre la barra fina.
/// La zona de toque es la fila entera, pero el valor siempre sale de la
/// barra, la misma geometría que dibuja el deslizador.
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
    let touch = slider_touch(bounds);
    if contains(touch, x, y) {
        return Some(VolumePanelHit::Slider(slider_value_at(
            slider_bar(touch),
            x,
        )));
    }
    None
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    (rect.x..rect.x + rect.width).contains(&x) && (rect.y..rect.y + rect.height).contains(&y)
}

/// Nivel que corresponde a `x` mientras se arrastra, sobre la misma barra
/// que dibuja el deslizador: se queda en 0 o en 1 en los extremos.
pub fn volume_slider_value(bounds: Rect, x: f32) -> f32 {
    slider_value_at(slider_bar(slider_touch(bounds)), x)
}
