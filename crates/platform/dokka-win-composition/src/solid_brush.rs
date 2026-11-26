use dokka_scene::Color;
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;
use windows::Win32::Graphics::Direct2D::{ID2D1DeviceContext, ID2D1SolidColorBrush};
use windows::core::Result;

/// Pincel de Direct2D de un solo color.
pub(crate) fn solid_brush(
    context: &ID2D1DeviceContext,
    color: Color,
) -> Result<ID2D1SolidColorBrush> {
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
