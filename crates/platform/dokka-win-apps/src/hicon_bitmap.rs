//! Convierte un `HICON` en el `Bitmap` propio de Dokka.
//!
//! Se dibuja el icono a un DIB de 32 bits y se premultiplica la alfa, que es
//! como Dokka guarda los píxeles (BGRA, filas de arriba abajo).

use std::ffi::c_void;

use dokka_scene::Bitmap;
use windows::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS,
    DeleteDC, DeleteObject, GetDIBits, SelectObject,
};
use windows::Win32::UI::WindowsAndMessaging::{DI_NORMAL, DrawIconEx, HICON};
use windows::core::{Error, Result};

/// Dibuja el icono al lado pedido y lo devuelve premultiplicado.
pub(crate) fn read_hicon(icon: HICON, side: i32) -> Result<Bitmap> {
    if side <= 0 || side > 512 {
        return Err(Error::from_thread());
    }
    let width = side as u32;
    let height = side as u32;
    let total = (width as usize)
        .checked_mul(height as usize)
        .and_then(|area| area.checked_mul(4))
        .ok_or_else(Error::from_thread)?;
    let mut pixels = vec![0_u8; total];
    let mut info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: side,
            biHeight: -side,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };

    // SAFETY: el DC compatible se crea aquí y se libera al salir.
    let context = unsafe { CreateCompatibleDC(None) };
    // SAFETY: `info` describe un DIB de 32 bits del lado pedido.
    let dib = unsafe {
        let mut bits: *mut c_void = std::ptr::null_mut();
        CreateDIBSection(Some(context), &info, DIB_RGB_COLORS, &mut bits, None, 0)
    }?;
    // SAFETY: el DIB se selecciona en su DC y se restaura antes de liberar.
    let drawn = unsafe {
        let previous = SelectObject(context, dib.into());
        let drawn = DrawIconEx(context, 0, 0, icon, side, side, 0, None, DI_NORMAL);
        let _ = SelectObject(context, previous);
        drawn
    };
    if drawn.is_err() {
        // SAFETY: el DIB y el DC los creó esta función.
        unsafe {
            let _ = DeleteObject(dib.into());
            let _ = DeleteDC(context);
        }
        return Err(Error::from_thread());
    }
    // SAFETY: `pixels` tiene sitio para `height` filas de 32 bits.
    let lines = unsafe {
        GetDIBits(
            context,
            dib,
            0,
            height,
            Some(pixels.as_mut_ptr().cast()),
            &mut info,
            DIB_RGB_COLORS,
        )
    };
    // SAFETY: el DIB y el DC los creó esta función.
    unsafe {
        let _ = DeleteObject(dib.into());
        let _ = DeleteDC(context);
    }
    if lines == 0 {
        return Err(Error::from_thread());
    }
    premultiply(&mut pixels);
    Ok(Bitmap {
        width,
        height,
        pixels,
    })
}

/// Pasa los píxeles de rectos a premultiplicados, en su sitio.
fn premultiply(pixels: &mut [u8]) {
    for chunk in pixels.as_chunks_mut::<4>().0 {
        let alpha = chunk[3] as u32;
        chunk[0] = ((chunk[0] as u32 * alpha + 127) / 255) as u8;
        chunk[1] = ((chunk[1] as u32 * alpha + 127) / 255) as u8;
        chunk[2] = ((chunk[2] as u32 * alpha + 127) / 255) as u8;
    }
}
