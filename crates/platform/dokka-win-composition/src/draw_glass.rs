use dokka_scene::Panel;
use windows::UI::Composition::{Compositor, SpriteVisual};
use windows::core::Result;
use windows_numerics::Vector2;

/// Capa que muestra lo de detrás desenfocado, recortada con esquinas redondeadas.
pub(crate) fn draw_glass(compositor: &Compositor, panel: &Panel) -> Result<SpriteVisual> {
    let size = Vector2 {
        X: panel.size.width,
        Y: panel.size.height,
    };

    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetSize(size)?;
    outline.SetCornerRadius(Vector2 {
        X: panel.corner_radius,
        Y: panel.corner_radius,
    })?;

    let glass = compositor.CreateSpriteVisual()?;
    glass.SetSize(size)?;
    glass.SetBrush(&compositor.CreateHostBackdropBrush()?)?;
    glass.SetClip(&compositor.CreateGeometricClipWithGeometry(&outline)?)?;
    Ok(glass)
}
