use dokka_scene::Text;
use dokka_win_text::{CenteredText, TextPainter};
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice};
use windows::Win32::Graphics::Direct2D::D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE;
use windows::core::Result;
use windows_numerics::{Matrix3x2, Vector2};

use crate::paint_surface::paint_surface;
use crate::solid_brush::solid_brush;

/// Se pinta al doble de resolución para que quede nítido en pantallas escaladas.
const OVERSAMPLE: f32 = 2.0;

/// Pinta un texto suelto y devuelve su superficie y su tamaño lógico.
pub(crate) fn paint_text(
    device: &CompositionGraphicsDevice,
    painter: &TextPainter,
    text: &Text,
) -> Result<(CompositionDrawingSurface, Vector2)> {
    let (measured_width, measured_height) =
        painter.measure(&text.content, text.size, text.is_bold)?;
    let width = measured_width.ceil() + 2.0;
    let height = measured_height.ceil();

    let pixels = |value: f32| (value * OVERSAMPLE) as u32;
    let surface = paint_surface(device, pixels(width), pixels(height), |context, offset| {
        let transform = Matrix3x2 {
            M11: OVERSAMPLE,
            M12: 0.0,
            M21: 0.0,
            M22: OVERSAMPLE,
            M31: offset.x as f32,
            M32: offset.y as f32,
        };
        // SAFETY: el contexto está en pleno dibujo.
        unsafe {
            context.SetTransform(&transform);
            context.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE);
        }
        let line = CenteredText {
            text: &text.content,
            x: width / 2.0,
            y: height / 2.0,
            size: text.size,
            is_bold: text.is_bold,
        };
        painter.draw(context, &solid_brush(context, text.color)?, line)
    })?;

    let size = Vector2 {
        X: width,
        Y: height,
    };
    Ok((surface, size))
}
