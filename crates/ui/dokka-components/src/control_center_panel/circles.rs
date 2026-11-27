//! Botones circulares, todos en el mismo cristal con icono oscuro: el blanco
//! solo marca un icono activo (el círculo interior de la cápsula de red),
//! nunca un botón entero. Lo oculto no se dibuja.

use dokka_model::ControlItem;
use dokka_theme::{Theme, control_center as measures};

use super::icons::ControlCenterIcons;
use super::layout::ControlLayout;
use super::paint::{Pieces, picture, tile};

/// Suma los círculos visibles con su icono centrado a 22.
pub(crate) fn add_circles(
    pieces: &mut Pieces,
    theme: &Theme,
    layout: &ControlLayout,
    _props: &super::ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let circles = [
        (ControlItem::Desktop, &icons.desktop),
        (ControlItem::Lock, &icons.lock),
        (ControlItem::TaskView, &icons.task_view),
        (ControlItem::Project, &icons.project),
        (ControlItem::Calculator, &icons.calculator),
        (ControlItem::ScreenClip, &icons.screen_clip),
        (ControlItem::Clock, &icons.clock),
        (ControlItem::TaskManager, &icons.task_manager),
    ];
    for (item, drawing) in circles {
        let Some(slot) = layout.rect(item) else {
            continue;
        };
        pieces.tiles.push(tile(theme, slot, slot.width / 2.0));
        pieces
            .icons
            .push(picture(theme, drawing, slot, measures::CIRCLE_ICON_SIDE));
    }
}
