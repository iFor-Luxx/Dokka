//! Pinceles en degradado de las piezas separadas (Liquid Glass).
//!
//! El filo del borde y el brillo alto derivan del color recibido del tema
//! variando solo su alfa: aquí no hay ningún color escrito. Ambos usan
//! `MappingMode::Relative`, así que siguen bien al cambiar de tamaño sin
//! reconstruir capas en `update`.

use windows::UI::Composition::{Compositor, CompositionLinearGradientBrush};
use windows::core::Result;
use windows_numerics::Vector2;

use crate::windows_color::windows_color;

/// Filo de luz en diagonal: brilla arriba a la izquierda, se apaga en los
/// laterales y vuelve a brillar algo abajo a la derecha.
pub(crate) fn edge_brush(
    compositor: &Compositor,
    border: dokka_scene::Color,
) -> Result<CompositionLinearGradientBrush> {
    let base = windows_color(border);
    let brush = compositor.CreateLinearGradientBrush()?;
    brush.SetMappingMode(windows::UI::Composition::CompositionMappingMode::Relative)?;
    brush.SetStartPoint(Vector2 { X: 0.0, Y: 0.0 })?;
    brush.SetEndPoint(Vector2 { X: 1.0, Y: 1.0 })?;
    let stops = brush.ColorStops()?;
    let full = base.A;
    let faint = scaled_alpha(full, 1, 8);
    let soft = scaled_alpha(full, 1, 2);
    for (offset, alpha) in [
        (0.0, full),
        (0.38, faint),
        (0.62, faint),
        (1.0, soft),
    ] {
        stops.Append(&compositor.CreateColorGradientStopWithOffsetAndColor(
            offset,
            with_alpha(base, alpha),
        )?)?;
    }
    Ok(brush)
}

/// Brillo interior muy suave de la parte alta: blanco casi transparente
/// que se apaga hacia la mitad y queda transparente abajo.
pub(crate) fn sheen_brush(
    compositor: &Compositor,
    border: dokka_scene::Color,
) -> Result<CompositionLinearGradientBrush> {
    let base = windows_color(border);
    let brush = compositor.CreateLinearGradientBrush()?;
    brush.SetMappingMode(windows::UI::Composition::CompositionMappingMode::Relative)?;
    brush.SetStartPoint(Vector2 { X: 0.5, Y: 0.0 })?;
    brush.SetEndPoint(Vector2 { X: 0.5, Y: 1.0 })?;
    let stops = brush.ColorStops()?;
    let top = scaled_alpha(base.A, 1, 4);
    for (offset, alpha) in [(0.0, top), (0.45, 0), (1.0, 0)] {
        stops.Append(&compositor.CreateColorGradientStopWithOffsetAndColor(
            offset,
            with_alpha(base, alpha),
        )?)?;
    }
    Ok(brush)
}

fn with_alpha(color: windows::UI::Color, alpha: u8) -> windows::UI::Color {
    windows::UI::Color {
        A: alpha,
        R: color.R,
        G: color.G,
        B: color.B,
    }
}

fn scaled_alpha(alpha: u8, num: u16, den: u16) -> u8 {
    if den == 0 {
        return alpha;
    }
    ((u16::from(alpha) * num) / den).min(u16::from(u8::MAX)) as u8
}
