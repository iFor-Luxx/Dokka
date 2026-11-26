use dokka_scene::Text;
use windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::painter::Painter;

/// Texto ya pintado; solo se repinta si cambia.
pub(crate) struct TextLayer {
    visual: SpriteVisual,
    content: Option<String>,
    size: Vector2,
}

impl TextLayer {
    pub(crate) fn add(compositor: &Compositor, parent: &ContainerVisual) -> Result<Self> {
        let visual = compositor.CreateSpriteVisual()?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self {
            visual,
            content: None,
            size: Vector2::default(),
        })
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        text: &Text,
    ) -> Result<()> {
        if self.content.as_ref() != Some(&text.content) {
            let (surface, size) = painter.paint_text(text)?;
            self.visual
                .SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
            self.visual.SetSize(size)?;
            self.size = size;
            self.content = Some(text.content.clone());
        }
        self.visual.SetOffset(Vector3 {
            X: text.center_x - self.size.X / 2.0,
            Y: text.center_y - self.size.Y / 2.0,
            Z: 0.0,
        })
    }
}
