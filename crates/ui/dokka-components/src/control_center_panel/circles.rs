//! Botones circulares, todos en el mismo cristal con icono oscuro: el blanco
//! solo marca un icono activo (el círculo interior de la cápsula de red),
//! nunca un botón entero.

use dokka_scene::Rect;
use dokka_theme::{Theme, control_center as measures};

use super::icons::ControlCenterIcons;
use super::layout::circle_slot;
use super::paint::{Pieces, picture, tile};

/// Suma los ocho círculos con su icono centrado a 22.
pub(crate) fn add_circles(
    pieces: &mut Pieces,
    theme: &Theme,
    bounds: Rect,
    props: &super::ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let with_brightness = props.brightness.is_some();
    let circles = [
        &icons.desktop,
        &icons.lock,
        &icons.task_view,
        &icons.project,
        &icons.calculator,
        &icons.screen_clip,
        &icons.clock,
        &icons.task_manager,
    ];
    for (index, drawing) in circles.into_iter().enumerate() {
        let slot = circle_slot(bounds, index, with_brightness);
        pieces.tiles.push(tile(theme, slot, slot.width / 2.0));
        pieces
            .icons
            .push(picture(theme, drawing, slot, measures::CIRCLE_ICON_SIDE));
    }
}
