//! Pista del deslizador agarrado: la misma que pinta el contenido.
//!
//! El valor bajo el cursor sale de la misma pista del dibujo y se queda en
//! 0 o en 1 aunque el cursor se pase de la barra.

use super::logic::{SliderTarget, card_heights, content_slider_track};
use super::values::ContentValues;

/// Pista del deslizador agarrado: la misma que pinta el contenido.
pub(crate) fn drag_track(target: SliderTarget) -> Option<dokka_scene::Rect> {
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
pub(crate) fn clamped_slider_value(
    target: SliderTarget,
    values: &ContentValues,
    x: f32,
) -> f32 {
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
