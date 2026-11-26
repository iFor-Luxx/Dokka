use dokka_scene::Bitmap;
use windows::Foundation::Size;
use windows::Graphics::DirectX::{DirectXAlphaMode, DirectXPixelFormat};
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice};
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_RECT_F, D2D_SIZE_U, D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_PIXEL_FORMAT,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_BITMAP_PROPERTIES1, D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC, ID2D1DeviceContext,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::System::WinRT::Composition::ICompositionDrawingSurfaceInterop;
use windows::core::{Interface, Result};

const BASE_DPI: f32 = 96.0;

/// Crea una superficie del tamaño de la imagen y la pinta en ella.
pub(crate) fn paint_bitmap(
    device: &CompositionGraphicsDevice,
    bitmap: &Bitmap,
) -> Result<CompositionDrawingSurface> {
    let surface = device.CreateDrawingSurface(
        Size {
            Width: bitmap.width as f32,
            Height: bitmap.height as f32,
        },
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        DirectXAlphaMode::Premultiplied,
    )?;
    let interop: ICompositionDrawingSurfaceInterop = surface.cast()?;
    let mut offset = POINT::default();
    // SAFETY: cada `BeginDraw` se cierra con `EndDraw`, también si falla el pintado.
    unsafe {
        let context: ID2D1DeviceContext = interop.BeginDraw(None, &mut offset)?;
        let painted = paint(&context, bitmap, offset);
        interop.EndDraw()?;
        painted?;
    }
    Ok(surface)
}

fn paint(context: &ID2D1DeviceContext, bitmap: &Bitmap, offset: POINT) -> Result<()> {
    let properties = D2D1_BITMAP_PROPERTIES1 {
        pixelFormat: D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT_B8G8R8A8_UNORM,
            alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
        },
        dpiX: BASE_DPI,
        dpiY: BASE_DPI,
        ..Default::default()
    };
    // La superficie puede ser parte de una textura mayor: hay que respetar el desplazamiento.
    let target = D2D_RECT_F {
        left: offset.x as f32,
        top: offset.y as f32,
        right: offset.x as f32 + bitmap.width as f32,
        bottom: offset.y as f32 + bitmap.height as f32,
    };
    // SAFETY: `pixels` contiene `height` filas de `width * 4` bytes (invariante de `Bitmap`).
    unsafe {
        context.SetDpi(BASE_DPI, BASE_DPI);
        context.Clear(None);
        let image = context.CreateBitmap(
            D2D_SIZE_U {
                width: bitmap.width,
                height: bitmap.height,
            },
            Some(bitmap.pixels.as_ptr().cast()),
            bitmap.width * 4,
            &properties,
        )?;
        context.DrawBitmap(
            &image,
            Some(&target),
            1.0,
            D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC,
            None,
            None,
        );
    }
    Ok(())
}
