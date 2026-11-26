use dokka_scene::Label;
use dokka_win_text::{CenteredText, TextPainter};
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice};
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::Direct2D::{
    D2D1_ROUNDED_RECT, D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE, ID2D1DeviceContext,
};
use windows::core::Result;
use windows_numerics::{Matrix3x2, Vector2};

use crate::paint_surface::paint_surface;
use crate::solid_brush::solid_brush;

/// Se pinta al doble de resolución para que el texto quede nítido en pantallas escaladas.
const OVERSAMPLE: f32 = 2.0;

/// Pinta la etiqueta y devuelve su superficie y su tamaño lógico.
pub(crate) fn paint_label(
    device: &CompositionGraphicsDevice,
    text: &TextPainter,
    label: &Label,
) -> Result<(CompositionDrawingSurface, Vector2)> {
    let (text_width, text_height) = text.measure(&label.text, label.font_size, false)?;
    let width = (text_width + label.padding_x * 2.0).ceil();
    let body = (text_height + label.padding_y * 2.0).ceil();
    let height = body + label.pointer_height;

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
        paint_background(context, label, width, body)?;
        let line = CenteredText {
            text: &label.text,
            x: width / 2.0,
            y: body / 2.0,
            size: label.font_size,
            is_bold: false,
        };
        text.draw(context, &solid_brush(context, label.text_color)?, line)
    })?;

    let size = Vector2 {
        X: width,
        Y: height,
    };
    Ok((surface, size))
}

fn paint_background(
    context: &ID2D1DeviceContext,
    label: &Label,
    width: f32,
    body: f32,
) -> Result<()> {
    let brush = solid_brush(context, label.background)?;
    let area = D2D1_ROUNDED_RECT {
        rect: D2D_RECT_F {
            left: 0.0,
            top: 0.0,
            right: width,
            bottom: body,
        },
        radiusX: label.corner_radius,
        radiusY: label.corner_radius,
    };
    // SAFETY: el contexto está en pleno dibujo.
    unsafe { context.FillRoundedRectangle(&area, &brush) };

    // Punta triangular hecha de franjas que se estrechan.
    let steps = (label.pointer_height * OVERSAMPLE) as u32;
    for step in 0..steps {
        let top = body + step as f32 / OVERSAMPLE;
        let half = label.pointer_height * (1.0 - step as f32 / steps as f32);
        let strip = D2D_RECT_F {
            left: width / 2.0 - half,
            top,
            right: width / 2.0 + half,
            bottom: top + 1.0 / OVERSAMPLE,
        };
        // SAFETY: el contexto está en pleno dibujo.
        unsafe { context.FillRectangle(&strip, &brush) };
    }
    Ok(())
}
