use windows::Foundation::Size;
use windows::Graphics::DirectX::{DirectXAlphaMode, DirectXPixelFormat};
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice};
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Direct2D::ID2D1DeviceContext;
use windows::Win32::System::WinRT::Composition::ICompositionDrawingSurfaceInterop;
use windows::core::{Interface, Result};

const BASE_DPI: f32 = 96.0;

/// Crea una superficie transparente y deja que `paint` dibuje en ella.
/// `paint` recibe el desplazamiento en píxeles donde empieza su zona.
pub(crate) fn paint_surface(
    device: &CompositionGraphicsDevice,
    width: u32,
    height: u32,
    paint: impl FnOnce(&ID2D1DeviceContext, POINT) -> Result<()>,
) -> Result<CompositionDrawingSurface> {
    let surface = device.CreateDrawingSurface(
        Size {
            Width: width as f32,
            Height: height as f32,
        },
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        DirectXAlphaMode::Premultiplied,
    )?;
    let interop: ICompositionDrawingSurfaceInterop = surface.cast()?;
    let mut offset = POINT::default();
    // SAFETY: cada `BeginDraw` se cierra con `EndDraw`, también si falla el pintado.
    unsafe {
        let context: ID2D1DeviceContext = interop.BeginDraw(None, &mut offset)?;
        context.SetDpi(BASE_DPI, BASE_DPI);
        context.Clear(None);
        let painted = paint(&context, offset);
        interop.EndDraw()?;
        painted?;
    }
    Ok(surface)
}
