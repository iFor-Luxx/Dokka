//! Posición del dock como índice del segmentado.
//!
//! La isla traduce: el componente solo ve 0, 1 y 2 de izquierda a derecha.

use dokka_model::DockPosition;

/// Índice del segmentado para esta posición (0 izquierda, 1 abajo, 2 derecha).
pub(crate) fn position_to_index(position: DockPosition) -> usize {
    match position {
        DockPosition::Left => 0,
        DockPosition::Bottom => 1,
        DockPosition::Right => 2,
    }
}

/// Posición para este índice; fuera de rango, la de fábrica (abajo).
pub(crate) fn position_from_index(index: usize) -> DockPosition {
    match index {
        0 => DockPosition::Left,
        1 => DockPosition::Bottom,
        2 => DockPosition::Right,
        _ => DockPosition::Bottom,
    }
}

/// Textos del segmentado, en su orden.
pub(crate) fn position_options() -> Vec<String> {
    vec![
        "Izquierda".to_owned(),
        "Abajo".to_owned(),
        "Derecha".to_owned(),
    ]
}
