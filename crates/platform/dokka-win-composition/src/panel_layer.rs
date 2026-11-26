use dokka_scene::Panel;
use windows::UI::Composition::{
    CompositionRoundedRectangleGeometry, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::windows_color::windows_color;

const BORDER_THICKNESS: f32 = 1.0;

/// Cristal desenfocado con tinte y borde, que puede cambiar de tamaño.
pub(crate) struct PanelLayer {
    glass: SpriteVisual,
    glass_outline: CompositionRoundedRectangleGeometry,
    tint: ShapeVisual,
    tint_outline: CompositionRoundedRectangleGeometry,
}

impl PanelLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        panel: &Panel,
    ) -> Result<Self> {
        let glass_outline = rounded(compositor, panel.corner_radius)?;
        let glass = compositor.CreateSpriteVisual()?;
        glass.SetBrush(&compositor.CreateHostBackdropBrush()?)?;
        glass.SetClip(&compositor.CreateGeometricClipWithGeometry(&glass_outline)?)?;

        // El trazo se centra en el contorno: se mete medio grosor para que no se corte.
        let inset = BORDER_THICKNESS / 2.0;
        let tint_outline = rounded(compositor, panel.corner_radius - inset)?;
        tint_outline.SetOffset(Vector2 { X: inset, Y: inset })?;
        let shape = compositor.CreateSpriteShapeWithGeometry(&tint_outline)?;
        shape.SetFillBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.tint))?)?;
        shape
            .SetStrokeBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.border))?)?;
        shape.SetStrokeThickness(BORDER_THICKNESS)?;
        let tint = compositor.CreateShapeVisual()?;
        tint.Shapes()?.Append(&shape)?;

        let children = parent.Children()?;
        children.InsertAtTop(&glass)?;
        children.InsertAtTop(&tint)?;
        Ok(Self {
            glass,
            glass_outline,
            tint,
            tint_outline,
        })
    }

    pub(crate) fn update(&self, panel: &Panel) -> Result<()> {
        let bounds = panel.bounds;
        let offset = Vector3 {
            X: bounds.x,
            Y: bounds.y,
            Z: 0.0,
        };
        let size = Vector2 {
            X: bounds.width,
            Y: bounds.height,
        };
        self.glass.SetOffset(offset)?;
        self.glass.SetSize(size)?;
        self.glass_outline.SetSize(size)?;
        self.tint.SetOffset(offset)?;
        self.tint.SetSize(size)?;
        self.tint_outline.SetSize(Vector2 {
            X: bounds.width - BORDER_THICKNESS,
            Y: bounds.height - BORDER_THICKNESS,
        })
    }
}

fn rounded(compositor: &Compositor, radius: f32) -> Result<CompositionRoundedRectangleGeometry> {
    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetCornerRadius(Vector2 {
        X: radius,
        Y: radius,
    })?;
    Ok(outline)
}
