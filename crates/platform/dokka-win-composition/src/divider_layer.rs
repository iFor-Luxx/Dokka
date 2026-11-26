use dokka_scene::Divider;
use windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::windows_color::windows_color;

pub(crate) struct DividerLayer {
    visual: SpriteVisual,
}

impl DividerLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        divider: &Divider,
    ) -> Result<Self> {
        let visual = compositor.CreateSpriteVisual()?;
        visual.SetBrush(&compositor.CreateColorBrushWithColor(windows_color(divider.color))?)?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self { visual })
    }

    pub(crate) fn update(&self, divider: &Divider) -> Result<()> {
        let bounds = divider.bounds;
        self.visual.SetOffset(Vector3 {
            X: bounds.x,
            Y: bounds.y,
            Z: 0.0,
        })?;
        self.visual.SetSize(Vector2 {
            X: bounds.width,
            Y: bounds.height,
        })
    }
}
