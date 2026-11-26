use dokka_scene::Dot;
use windows::UI::Composition::{Compositor, ContainerVisual, ShapeVisual};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::windows_color::windows_color;

pub(crate) struct DotLayer {
    visual: ShapeVisual,
}

impl DotLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        dot: &Dot,
    ) -> Result<Self> {
        let radius = dot.diameter / 2.0;
        let circle = compositor.CreateEllipseGeometry()?;
        let both = Vector2 {
            X: radius,
            Y: radius,
        };
        circle.SetCenter(both)?;
        circle.SetRadius(both)?;
        let shape = compositor.CreateSpriteShapeWithGeometry(&circle)?;
        shape.SetFillBrush(&compositor.CreateColorBrushWithColor(windows_color(dot.color))?)?;

        let visual = compositor.CreateShapeVisual()?;
        visual.SetSize(Vector2 {
            X: dot.diameter,
            Y: dot.diameter,
        })?;
        visual.Shapes()?.Append(&shape)?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self { visual })
    }

    pub(crate) fn update(&self, dot: &Dot) -> Result<()> {
        self.visual.SetOffset(Vector3 {
            X: dot.x,
            Y: dot.y,
            Z: 0.0,
        })
    }
}
