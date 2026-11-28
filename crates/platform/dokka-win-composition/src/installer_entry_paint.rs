//! Visuales estáticos de la entrada: fondo y montañas con su recorte.
//!
//! El alfa de los PNG se respeta porque se decodifican a PBGRA como la foto.
//! Las animaciones (viaje, desenfoque, fundidos) viven en
//! `installer_entry_layers` y las hace el compositor.

use std::sync::Arc;

use dokka_scene::IconImage;
use windows::UI::Composition::{CompositionEffectBrush, Compositor, SpriteVisual};
use windows::core::{HSTRING, Result};
use windows_numerics::{Vector2, Vector3};

use crate::blur_effect::{BLUR_ANIMATABLE, SOURCE_NAME, create_blur_factory};
use crate::painter::Painter;

/// Rectángulo de una capa de la entrada, como datos internos.
pub(crate) struct EntryRect {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

/// Montaña con desenfoque animable desde `blur_max`, ya colocada al inicio.
pub(crate) fn blurred_visual(
    compositor: &Compositor,
    painter: &Painter,
    bitmap: dokka_scene::Bitmap,
    blur_max: f32,
    rect: &EntryRect,
) -> Result<(SpriteVisual, CompositionEffectBrush)> {
    let surface = painter.paint(&IconImage::Pixels(Arc::new(bitmap)), rect.width)?;
    let brush_source = compositor.CreateSurfaceBrushWithSurface(&surface)?;
    let factory = create_blur_factory(compositor)?;
    let brush = factory.CreateBrush()?;
    brush.SetSourceParameter(&HSTRING::from(SOURCE_NAME), &brush_source)?;
    brush
        .Properties()?
        .InsertScalar(&HSTRING::from(BLUR_ANIMATABLE), blur_max)?;
    let visual = compositor.CreateSpriteVisual()?;
    visual.SetBrush(&brush)?;
    visual.SetSize(Vector2 {
        X: rect.width,
        Y: rect.height,
    })?;
    visual.SetOffset(Vector3 {
        X: rect.x,
        Y: rect.y,
        Z: 0.0,
    })?;
    clip_to_photo(compositor, &visual, rect.width, rect.height, rect.radius)?;
    Ok((visual, brush))
}

/// Fondo estático con el mismo tamaño, posición y recorte que `playa`.
pub(crate) fn still_visual(
    compositor: &Compositor,
    visual: &SpriteVisual,
    brush: &windows::UI::Composition::CompositionSurfaceBrush,
    rect: &EntryRect,
) -> Result<()> {
    visual.SetBrush(brush)?;
    visual.SetSize(Vector2 {
        X: rect.width,
        Y: rect.height,
    })?;
    visual.SetOffset(Vector3 {
        X: rect.x,
        Y: rect.y,
        Z: 0.0,
    })?;
    clip_to_photo(compositor, visual, rect.width, rect.height, rect.radius)
}

/// Recorte a las esquinas, igual que la foto del instalador.
fn clip_to_photo(
    compositor: &Compositor,
    visual: &SpriteVisual,
    width: f32,
    height: f32,
    radius: f32,
) -> Result<()> {
    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetSize(Vector2 {
        X: width,
        Y: height,
    })?;
    outline.SetCornerRadius(Vector2 {
        X: radius,
        Y: radius,
    })?;
    visual.SetClip(&compositor.CreateGeometricClipWithGeometry(&outline)?)
}
