use dokka_scene::Panel;
use windows::UI::Composition::{
    CompositionRoundedRectangleGeometry, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::windows_color::windows_color;

/// Cristal desenfocado con tinte y borde, que puede cambiar de tamaño.
/// El resaltado de un `Overlay` es plano (solo tinte, sin cristal):
/// se crea con `add_flat` para que el `HostBackdropBrush` no muestree
/// el fondo y tape el tinte.
pub(crate) struct PanelLayer {
    glass: Option<SpriteVisual>,
    glass_outline: Option<CompositionRoundedRectangleGeometry>,
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

        let (tint, tint_outline) = tint_visual(compositor, panel)?;
        let children = parent.Children()?;
        children.InsertAtTop(&glass)?;
        children.InsertAtTop(&tint)?;
        Ok(Self {
            glass: Some(glass),
            glass_outline: Some(glass_outline),
            tint,
            tint_outline,
        })
    }

    /// Tinte plano con borde, sin cristal ni `HostBackdropBrush`.
    /// Solo para el resaltado del `Overlay` (píldora y menú).
    pub(crate) fn add_flat(
        compositor: &Compositor,
        parent: &ContainerVisual,
        panel: &Panel,
    ) -> Result<Self> {
        let (tint, tint_outline) = tint_visual(compositor, panel)?;
        parent.Children()?.InsertAtTop(&tint)?;
        Ok(Self {
            glass: None,
            glass_outline: None,
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
        if let (Some(glass), Some(glass_outline)) = (&self.glass, &self.glass_outline) {
            glass.SetOffset(offset)?;
            glass.SetSize(size)?;
            glass_outline.SetSize(size)?;
        }
        self.tint.SetOffset(offset)?;
        self.tint.SetSize(size)?;
        self.tint_outline.SetSize(Vector2 {
            X: bounds.width - panel.border_width,
            Y: bounds.height - panel.border_width,
        })
    }
}

/// Visual del tinte con su borde, compartido por el cristal y el plano.
fn tint_visual(
    compositor: &Compositor,
    panel: &Panel,
) -> Result<(ShapeVisual, CompositionRoundedRectangleGeometry)> {
    // El trazo se centra en el contorno: se mete medio grosor para que no se corte.
    let inset = panel.border_width / 2.0;
    let tint_outline = rounded(compositor, panel.corner_radius - inset)?;
    tint_outline.SetOffset(Vector2 { X: inset, Y: inset })?;
    let shape = compositor.CreateSpriteShapeWithGeometry(&tint_outline)?;
    shape.SetFillBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.tint))?)?;
    shape.SetStrokeBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.border))?)?;
    shape.SetStrokeThickness(panel.border_width)?;
    let tint = compositor.CreateShapeVisual()?;
    tint.Shapes()?.Append(&shape)?;
    Ok((tint, tint_outline))
}

fn rounded(compositor: &Compositor, radius: f32) -> Result<CompositionRoundedRectangleGeometry> {
    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetCornerRadius(Vector2 {
        X: radius,
        Y: radius,
    })?;
    Ok(outline)
}
