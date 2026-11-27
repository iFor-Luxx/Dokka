//! Conversión entre la barra (0 a 1) y los rangos de `dokka_model`.
//!
//! El valor de fábrica es 1.0 en ambos: su posición en la barra es el
//! neutral desde donde empieza el relleno.

use dokka_model::{DOCK_ICON_SCALE_RANGE, DOCK_MAGNIFICATION_RANGE};

/// De 0-1 al rango del tamaño de iconos, limitado.
pub(crate) fn icon_scale_from(value: f32) -> f32 {
    let (minimum, maximum) = DOCK_ICON_SCALE_RANGE;
    (minimum + value.clamp(0.0, 1.0) * (maximum - minimum)).clamp(minimum, maximum)
}

/// Del tamaño de iconos a 0-1 para la barra.
pub(crate) fn icon_scale_to(scale: f32) -> f32 {
    let (minimum, maximum) = DOCK_ICON_SCALE_RANGE;
    if maximum <= minimum {
        return 0.0;
    }
    ((scale - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
}

/// Posición en la barra del valor de fábrica del tamaño (1.0).
pub(crate) fn icon_scale_neutral() -> f32 {
    icon_scale_to(1.0)
}

/// De 0-1 al rango de la lupa, limitado.
pub(crate) fn magnification_from(value: f32) -> f32 {
    let (minimum, maximum) = DOCK_MAGNIFICATION_RANGE;
    (minimum + value.clamp(0.0, 1.0) * (maximum - minimum)).clamp(minimum, maximum)
}

/// De la lupa a 0-1 para la barra.
pub(crate) fn magnification_to(strength: f32) -> f32 {
    let (minimum, maximum) = DOCK_MAGNIFICATION_RANGE;
    if maximum <= minimum {
        return 0.0;
    }
    ((strength - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
}

/// Posición en la barra del valor de fábrica de la lupa (1.0).
pub(crate) fn magnification_neutral() -> f32 {
    magnification_to(1.0)
}
