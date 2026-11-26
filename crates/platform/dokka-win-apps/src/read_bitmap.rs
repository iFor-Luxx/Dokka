use dokka_scene::Bitmap;
use windows::Win32::Graphics::Gdi::{
    BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, DIB_RGB_COLORS, DeleteDC,
    GetDIBits, GetObjectW, HBITMAP,
};
use windows::core::{Error, Result};

/// Copia los píxeles de un bitmap de Windows a memoria propia.
pub(crate) fn read_bitmap(handle: HBITMAP) -> Result<Bitmap> {
    let mut details = BITMAP::default();
    // SAFETY: `details` es escribible y se declara su tamaño exacto.
    let copied = unsafe {
        GetObjectW(
            handle.into(),
            size_of::<BITMAP>() as i32,
            Some((&raw mut details).cast()),
        )
    };
    if copied == 0 {
        return Err(Error::from_thread());
    }

    let width = details.bmWidth.unsigned_abs();
    let height = details.bmHeight.unsigned_abs();
    let mut pixels = vec![0_u8; width as usize * height as usize * 4];
    let mut info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            // Altura negativa: filas de arriba abajo.
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };

    // SAFETY: `pixels` tiene sitio para `height` filas de 32 bits y el DC se libera aquí.
    let lines = unsafe {
        let context = CreateCompatibleDC(None);
        let lines = GetDIBits(
            context,
            handle,
            0,
            height,
            Some(pixels.as_mut_ptr().cast()),
            &mut info,
            DIB_RGB_COLORS,
        );
        let _ = DeleteDC(context);
        lines
    };
    if lines == 0 {
        return Err(Error::from_thread());
    }

    Ok(Bitmap {
        width,
        height,
        pixels,
    })
}
