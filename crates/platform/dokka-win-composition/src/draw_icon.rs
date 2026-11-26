use dokka_scene::Icon;
use windows::UI::Composition::{CompositionGraphicsDevice, Compositor, Visual};
use windows::core::{Interface, Result};
use windows_numerics::{Vector2, Vector3};

use crate::paint_bitmap::paint_bitmap;
use crate::windows_color::windows_color;

/// Capa de un icono: su imagen, o un cuadro redondeado si no tiene.
pub(crate) fn draw_icon(
    compositor: &Compositor,
    device: &CompositionGraphicsDevice,
    icon: &Icon,
) -> Result<Visual> {
    let size = Vector2 {
        X: icon.side,
        Y: icon.side,
    };
    let visual: Visual = match &icon.image {
        Some(bitmap) => {
            let sprite = compositor.CreateSpriteVisual()?;
            let surface = paint_bitmap(device, bitmap)?;
            sprite.SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
            sprite.cast()?
        }
        None => {
            let outline = compositor.CreateRoundedRectangleGeometry()?;
            outline.SetSize(size)?;
            outline.SetCornerRadius(Vector2 {
                X: icon.placeholder_radius,
                Y: icon.placeholder_radius,
            })?;
            let shape = compositor.CreateSpriteShapeWithGeometry(&outline)?;
            let color = windows_color(icon.placeholder);
            shape.SetFillBrush(&compositor.CreateColorBrushWithColor(color)?)?;
            let layer = compositor.CreateShapeVisual()?;
            layer.Shapes()?.Append(&shape)?;
            layer.cast()?
        }
    };
    visual.SetSize(size)?;
    visual.SetOffset(Vector3 {
        X: icon.x,
        Y: icon.y,
        Z: 0.0,
    })?;
    Ok(visual)
}
