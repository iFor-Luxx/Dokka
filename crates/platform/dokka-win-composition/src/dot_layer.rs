use dokka_scene::{Color, Dot};
use windows::UI::Composition::{CompositionSpriteShape, Compositor, ContainerVisual, ShapeVisual};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::windows_color::windows_color;

pub(crate) struct DotLayer {
    visual: ShapeVisual,
    shape: CompositionSpriteShape,
    color: Color,
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
        Ok(Self {
            visual,
            shape,
            color: dot.color,
        })
    }

    pub(crate) fn update(&mut self, compositor: &Compositor, dot: &Dot) -> Result<()> {
        self.set_visible(true)?;
        if dot.color != self.color {
            // Solo se crea un pincel nuevo cuando el color cambia de verdad.
            let brush = compositor.CreateColorBrushWithColor(windows_color(dot.color))?;
            self.shape.SetFillBrush(&brush)?;
            self.color = dot.color;
        }
        self.visual.SetOffset(Vector3 {
            X: dot.x,
            Y: dot.y,
            Z: 0.0,
        })?;
        Ok(())
    }

    /// Oculta o muestra sin destruir la capa, conservando el orden de pintado.
    pub(crate) fn set_visible(&self, visible: bool) -> Result<()> {
        self.visual.SetIsVisible(visible)
    }
}
