use dokka_scene::Panel;
use windows::UI::Composition::{Compositor, ShapeVisual};
use windows::core::Result;
use windows_numerics::Vector2;

use crate::windows_color::windows_color;

const BORDER_THICKNESS: f32 = 1.0;

/// Tinte y borde fino sobre el cristal.
pub(crate) fn draw_tint(compositor: &Compositor, panel: &Panel) -> Result<ShapeVisual> {
    // El trazo se centra en el contorno: se mete medio grosor para que no se corte.
    let inset = BORDER_THICKNESS / 2.0;
    let radius = panel.corner_radius - inset;

    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetOffset(Vector2 { X: inset, Y: inset })?;
    outline.SetSize(Vector2 {
        X: panel.size.width - BORDER_THICKNESS,
        Y: panel.size.height - BORDER_THICKNESS,
    })?;
    outline.SetCornerRadius(Vector2 {
        X: radius,
        Y: radius,
    })?;

    let shape = compositor.CreateSpriteShapeWithGeometry(&outline)?;
    shape.SetFillBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.tint))?)?;
    shape.SetStrokeBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.border))?)?;
    shape.SetStrokeThickness(BORDER_THICKNESS)?;

    let layer = compositor.CreateShapeVisual()?;
    layer.SetSize(Vector2 {
        X: panel.size.width,
        Y: panel.size.height,
    })?;
    layer.Shapes()?.Append(&shape)?;
    Ok(layer)
}
