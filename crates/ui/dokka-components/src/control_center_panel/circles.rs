//! Botones circulares: Escritorio y Bloquear activos en la fila 2 (blanco
//! con icono azul); Vista de tareas y Proyectar en la fila 3 y Calculadora,
//! Recorte, Reloj y Tareas en la fila 5, en cristal normal con icono oscuro.

use dokka_scene::Rect;
use dokka_theme::{Theme, control_center as measures};

use super::icons::ControlCenterIcons;
use super::layout::circle_slot;
use super::paint::{Pieces, picture, tile, tile_tinted};

/// Suma los ocho círculos con su icono centrado a 22.
pub(crate) fn add_circles(
    pieces: &mut Pieces,
    theme: &Theme,
    bounds: Rect,
    icons: &ControlCenterIcons,
) {
    let active = [&icons.desktop_active, &icons.lock_active];
    for (index, drawing) in active.into_iter().enumerate() {
        let slot = circle_slot(bounds, index);
        pieces.tiles.push(tile_tinted(
            theme,
            slot,
            slot.width / 2.0,
            theme.tray.fill,
        ));
        pieces
            .icons
            .push(picture(theme, drawing, slot, measures::CIRCLE_ICON_SIDE));
    }
    let quiet = [
        &icons.task_view,
        &icons.project,
        &icons.calculator,
        &icons.screen_clip,
        &icons.clock,
        &icons.task_manager,
    ];
    for (offset, drawing) in quiet.into_iter().enumerate() {
        let slot = circle_slot(bounds, offset + 2);
        pieces.tiles.push(tile(theme, slot, slot.width / 2.0));
        pieces
            .icons
            .push(picture(theme, drawing, slot, measures::CIRCLE_ICON_SIDE));
    }
}
