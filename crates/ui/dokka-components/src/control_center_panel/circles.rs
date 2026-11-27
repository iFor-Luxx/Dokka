//! Botones circulares blancos: Escritorio y Bloqueo en la fila 2 junto a
//! la red, Administrador de tareas en la fila 3 junto a Preferencias. La
//! cuarta columna queda vacía: no hay más acciones y no se inventa ninguna.

use dokka_scene::Rect;
use dokka_theme::{Theme, tray_glass as glass};

use super::icons::ControlCenterIcons;
use super::layout::circle_slot;
use super::paint::{Pieces, picture, tile_tinted};

/// Suma los tres círculos blancos con su icono azul y su borde luminoso.
pub(crate) fn add_circles(
    pieces: &mut Pieces,
    theme: &Theme,
    bounds: Rect,
    icons: &ControlCenterIcons,
) {
    let drawings = [
        &icons.desktop_active,
        &icons.lock_active,
        &icons.task_manager_active,
    ];
    for (index, drawing) in drawings.into_iter().enumerate() {
        let slot = circle_slot(bounds, index);
        pieces.tiles.push(tile_tinted(
            theme,
            slot,
            slot.width / 2.0,
            theme.tray.circle_active,
        ));
        pieces
            .icons
            .push(picture(theme, drawing, slot, glass::CIRCLE_ICON));
    }
}
