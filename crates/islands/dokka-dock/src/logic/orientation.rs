//! Eje del dock: calcula en "a lo largo / hacia dentro" y convierte
//! a x/y en un solo sitio, en vez de duplicar código por posición.

use dokka_model::DockPosition;
use dokka_scene::Rect;

use crate::Dock;

/// `true` con el dock en un lateral (vertical).
pub(crate) fn is_vertical(dock: &Dock) -> bool {
    matches!(
        dock.preferences.position,
        DockPosition::Left | DockPosition::Right
    )
}

/// Posición vigente; si no se reconoce, abajo y visible (falla sola).
pub(crate) fn position_or_bottom(dock: &Dock) -> DockPosition {
    dock.preferences.position
}

/// Convierte coordenadas a lo largo / hacia dentro a un rectángulo.
/// Abajo: a lo largo es x, hacia dentro es y.
/// Laterales: a lo largo es y, hacia dentro es x.
pub(crate) fn pack(dock: &Dock, along: f32, cross: f32, size_along: f32, size_cross: f32) -> Rect {
    if is_vertical(dock) {
        Rect {
            x: cross,
            y: along,
            width: size_cross,
            height: size_along,
        }
    } else {
        Rect {
            x: along,
            y: cross,
            width: size_along,
            height: size_cross,
        }
    }
}
