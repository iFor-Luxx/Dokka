use dokka_scene::{Icon, IconImage};
use windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual, Visual};
use windows::core::{Interface, Result};
use windows_numerics::{Vector2, Vector3};

use crate::painter::Painter;
use crate::windows_color::windows_color;

/// Icono ya pintado; solo se repinta si cambia su imagen.
pub(crate) struct IconLayer {
    visual: Visual,
    /// Presente cuando el icono tiene imagen propia.
    sprite: Option<SpriteVisual>,
    image: Option<IconImage>,
    base_side: f32,
}

impl IconLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        painter: &Painter,
        parent: &ContainerVisual,
        icon: &Icon,
    ) -> Result<Self> {
        let size = Vector2 {
            X: icon.side,
            Y: icon.side,
        };
        let (visual, sprite): (Visual, _) = match &icon.image {
            Some(image) => {
                let sprite = compositor.CreateSpriteVisual()?;
                let surface = painter.paint(image, icon.side)?;
                sprite.SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
                (sprite.cast()?, Some(sprite))
            }
            None => (placeholder(compositor, icon, size)?, None),
        };
        visual.SetSize(size)?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self {
            visual,
            sprite,
            image: icon.image.clone(),
            base_side: icon.side,
        })
    }

    /// `true` si el icono puede actualizarse sin crear la capa de nuevo.
    pub(crate) fn accepts(&self, icon: &Icon) -> bool {
        self.image.is_some() == icon.image.is_some()
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        icon: &Icon,
    ) -> Result<()> {
        if let (Some(sprite), Some(old), Some(new)) = (&self.sprite, &self.image, &icon.image)
            && !old.is_same(new)
        {
            let surface = painter.paint(new, icon.side)?;
            sprite.SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
            self.image = Some(new.clone());
        }
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
