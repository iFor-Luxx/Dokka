//! Degradado del tinte y recorte a la ventana de la franja de Preferencias.

use dokka_scene::Panel;
use windows::UI::Composition::{
    CompositionLinearGradientBrush, CompositionMappingMode, CompositionRoundedRectangleGeometry,
    Compositor,
};
use windows::core::Result;
use windows_numerics::Vector2;

use crate::tile_brushes::scaled_alpha;
use crate::windows_color::windows_color;

/// Degradado vertical del tinte: casi opaco arriba y transparente abajo,
/// sin línea dura en el borde inferior. En relativo, sigue al tamaño solo.
pub(crate) fn veil_gradient(
    compositor: &Compositor,
    tint: dokka_scene::Color,
) -> Result<CompositionLinearGradientBrush> {
    let base = windows_color(tint);
    let brush = compositor.CreateLinearGradientBrush()?;
    brush.SetMappingMode(CompositionMappingMode::Relative)?;
    brush.SetStartPoint(Vector2 { X: 0.5, Y: 0.0 })?;
    brush.SetEndPoint(Vector2 { X: 0.5, Y: 1.0 })?;
    let stops = brush.ColorStops()?;
    let soft = scaled_alpha(base.A, 11, 20);
    for (offset, alpha) in [(0.0, base.A), (0.55, soft), (1.0, 0)] {
        stops.Append(&compositor.CreateColorGradientStopWithOffsetAndColor(
            offset,
            windows::UI::Color {
                A: alpha,
                R: base.R,
                G: base.G,
                B: base.B,
            },
        )?)?;
    }
    Ok(brush)
}

/// Recorte a la ventana redondeada, en coordenadas de su contenedor.
pub(crate) fn window_clip_of(
    geometry: &CompositionRoundedRectangleGeometry,
    window: &Panel,
) -> Result<()> {
    geometry.SetSize(Vector2 {
        X: window.bounds.width.max(0.0),
        Y: window.bounds.height.max(0.0),
    })?;
    geometry.SetCornerRadius(Vector2 {
        X: window.corner_radius,
        Y: window.corner_radius,
    })?;
    geometry.SetOffset(Vector2 {
        X: window.bounds.x,
        Y: window.bounds.y,
    })
}
