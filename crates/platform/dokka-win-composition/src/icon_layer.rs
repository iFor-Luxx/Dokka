use std::sync::Arc;

use dokka_scene::{Bitmap, Icon};
use windows::UI::Composition::{CompositionGraphicsDevice, Compositor, ContainerVisual, Visual};
use windows::core::{Interface, Result};
use windows_numerics::{Vector2, Vector3};

use crate::paint_bitmap::paint_bitmap;
use crate::windows_color::windows_color;

/// Icono ya pintado; después solo se mueve y se escala.
pub(crate) struct IconLayer {
    visual: Visual,
    base_side: f32,
    pub(crate) image: Option<Arc<Bitmap>>,
}

impl IconLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        device: &CompositionGraphicsDevice,
        parent: &ContainerVisual,
        icon: &Icon,
    ) -> Result<Self> {
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
            None => placeholder(compositor, icon, size)?,
        };
        visual.SetSize(size)?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self {
            visual,
            base_side: icon.side,
            image: icon.image.clone(),
        })
    }

    pub(crate) fn update(&self, icon: &Icon) -> Result<()> {
        let scale = icon.side / self.base_side;
        self.visual.SetScale(Vector3 {
            X: scale,
            Y: scale,
            Z: 1.0,
        })?;
        self.visual.SetOffset(Vector3 {
            X: icon.x,
            Y: icon.y,
            Z: 0.0,
        })
    }
}

fn placeholder(compositor: &Compositor, icon: &Icon, size: Vector2) -> Result<Visual> {
    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetSize(size)?;
    outline.SetCornerRadius(Vector2 {
        X: icon.placeholder_radius,
        Y: icon.placeholder_radius,
    })?;
    let shape = compositor.CreateSpriteShapeWithGeometry(&outline)?;
    shape.SetFillBrush(&compositor.CreateColorBrushWithColor(windows_color(icon.placeholder))?)?;
    let layer = compositor.CreateShapeVisual()?;
    layer.Shapes()?.Append(&shape)?;
    layer.cast()
}
