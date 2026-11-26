use dokka_scene::{Color, DRAWING_SIDE, Drawing, Shape};
use dokka_win_text::{CenteredText, TextPainter};
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice};
use windows::Win32::Graphics::Direct2D::Common::{D2D_RECT_F, D2D1_COLOR_F};
use windows::Win32::Graphics::Direct2D::{
    D2D1_ELLIPSE, D2D1_ROUNDED_RECT, D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE, ID2D1DeviceContext,
    ID2D1SolidColorBrush,
};
use windows::core::Result;
use windows_numerics::{Matrix3x2, Vector2};

use crate::paint_surface::paint_surface;

/// Resolución a la que se pintan los dibujos.
const DRAWING_PIXELS: u32 = 128;

pub(crate) fn paint_drawing(
    device: &CompositionGraphicsDevice,
    text: &TextPainter,
    drawing: &Drawing,
) -> Result<CompositionDrawingSurface> {
    paint_surface(device, DRAWING_PIXELS, DRAWING_PIXELS, |context, offset| {
        let scale = DRAWING_PIXELS as f32 / DRAWING_SIDE;
        let transform = Matrix3x2 {
            M11: scale,
            M12: 0.0,
            M21: 0.0,
            M22: scale,
            M31: offset.x as f32,
            M32: offset.y as f32,
        };
        // SAFETY: el contexto está en pleno dibujo.
        unsafe {
            context.SetTransform(&transform);
            context.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE);
        }
        for shape in &drawing.shapes {
            paint_shape(context, text, shape)?;
        }
        Ok(())
    })
}

fn paint_shape(context: &ID2D1DeviceContext, text: &TextPainter, shape: &Shape) -> Result<()> {
    // SAFETY: el contexto está en pleno dibujo y los argumentos viven durante cada llamada.
    unsafe {
        match *shape {
            Shape::RoundedRect {
                x,
                y,
                width,
                height,
                radius,
                color,
            } => {
                let area = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: x,
                        top: y,
                        right: x + width,
                        bottom: y + height,
                    },
                    radiusX: radius,
                    radiusY: radius,
                };
                context.FillRoundedRectangle(&area, &brush(context, color)?);
            }
            Shape::Circle {
                x,
                y,
                radius,
                color,
            } => {
                let circle = D2D1_ELLIPSE {
                    point: Vector2 { X: x, Y: y },
                    radiusX: radius,
                    radiusY: radius,
                };
                context.FillEllipse(&circle, &brush(context, color)?);
            }
            Shape::Line {
                from,
                to,
                width,
                color,
            } => {
                let start = Vector2 {
                    X: from.0,
                    Y: from.1,
                };
                let end = Vector2 { X: to.0, Y: to.1 };
                context.DrawLine(start, end, &brush(context, color)?, width, None);
            }
            Shape::Text {
                text: ref content,
                x,
                y,
                size,
                is_bold,
                color,
            } => {
                let line = CenteredText {
                    text: content,
                    x,
                    y,
                    size,
                    is_bold,
                };
                text.draw(context, &brush(context, color)?, line)?;
            }
        }
    }
    Ok(())
}

fn brush(context: &ID2D1DeviceContext, color: Color) -> Result<ID2D1SolidColorBrush> {
    let channel = |value: u8| f32::from(value) / 255.0;
    let color = D2D1_COLOR_F {
        r: channel(color.red),
        g: channel(color.green),
        b: channel(color.blue),
        a: channel(color.alpha),
    };
    // SAFETY: `color` vive durante la llamada.
    unsafe { context.CreateSolidColorBrush(&color, None) }
}
