use dokka_scene::Bitmap;
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice};
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_RECT_F, D2D_SIZE_U, D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_PIXEL_FORMAT,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_BITMAP_PROPERTIES1, D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC, ID2D1DeviceContext,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::core::Result;

use crate::paint_surface::paint_surface;

pub(crate) fn paint_bitmap(
    device: &CompositionGraphicsDevice,
    bitmap: &Bitmap,
) -> Result<CompositionDrawingSurface> {
    paint_surface(device, bitmap.width, bitmap.height, |context, offset| {
        copy_pixels(context, bitmap, offset)
    })
}

fn copy_pixels(context: &ID2D1DeviceContext, bitmap: &Bitmap, offset: POINT) -> Result<()> {
    let properties = D2D1_BITMAP_PROPERTIES1 {
        pixelFormat: D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT_B8G8R8A8_UNORM,
            alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
        },
        dpiX: 96.0,
        dpiY: 96.0,
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
