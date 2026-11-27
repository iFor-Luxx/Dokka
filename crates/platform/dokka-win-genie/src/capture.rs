// Adaptado de https://github.com/crafter-station/Muelle (commit 37e843d).
// MIT License
//
// Copyright (c) 2026 Muelle contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Copia de pantalla con GDI y version reducida para restaurar.
//!
//! La copia cuesta decenas de milisegundos vaya el tamano que vaya, asi que
//! quien llama la ejecuta en un hilo aparte y sigue mostrando la foto fija.

use std::ffi::c_void;
use std::mem::size_of;

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BitBlt, CAPTUREBLT, CreateCompatibleDC, CreateDIBSection,
    DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC, ReleaseDC, SRCCOPY, SelectObject,
};

use crate::geometry::PixelRect;

/// Foto BGRA opaca, fila de arriba primero.
#[derive(Debug, Clone)]
pub struct GenieImage {
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
}

/// Copia este rectangulo de pantalla. `None` si no tiene tamano o falla GDI.
pub(crate) fn capture_rect(rect: PixelRect) -> Option<GenieImage> {
    let (width, height) = (rect.width(), rect.height());
    if width <= 0 || height <= 0 {
        return None;
    }
    let info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };
    // SAFETY: cada objeto GDI creado aqui se libera antes de volver y `bits`
    // apunta a `width * height * 4` bytes de la seccion DIB mientras vive.
    unsafe {
        let screen = GetDC(None);
        let memory = CreateCompatibleDC(Some(screen));
        let mut bits: *mut c_void = std::ptr::null_mut();
        let bitmap = CreateDIBSection(Some(screen), &info, DIB_RGB_COLORS, &mut bits, None, 0);
        let image = match bitmap {
            Ok(bitmap) if !bits.is_null() => {
                let previous = SelectObject(memory, bitmap.into());
                let copied = BitBlt(
                    memory,
                    0,
                    0,
                    width,
                    height,
                    Some(screen),
                    rect.left,
                    rect.top,
                    SRCCOPY | CAPTUREBLT,
                );
                let image = copied.is_ok().then(|| {
                    let len = (width * height * 4) as usize;
                    let mut bgra = std::slice::from_raw_parts(bits.cast::<u8>(), len).to_vec();
                    bgra.as_chunks_mut::<4>().0.iter_mut().for_each(|pixel| {
                        pixel[3] = 255;
                    });
                    GenieImage {
                        width: width as u32,
                        height: height as u32,
                        bgra,
                    }
                });
                SelectObject(memory, previous);
                let _ = DeleteObject(bitmap.into());
                image
            }
            _ => None,
        };
        let _ = DeleteDC(memory);
        ReleaseDC(None, screen);
        image
    }
}

/// Captura con `RECT` de Windows. Envoltorio fino sobre `capture_rect`.
pub(crate) fn capture_win(rect: RECT) -> Option<GenieImage> {
    capture_rect(PixelRect::from_win(rect))
}

/// Promedia cada bloque de 2x2 pixeles en uno solo.
pub(crate) fn halved(image: &GenieImage) -> GenieImage {
    let (width, height) = ((image.width / 2).max(1), (image.height / 2).max(1));
    let stride = image.width as usize * 4;
    let mut bgra = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height as usize {
        let upper = (y * 2).min(image.height as usize - 1) * stride;
        let lower = (y * 2 + 1).min(image.height as usize - 1) * stride;
        for x in 0..width as usize {
            let left = (x * 2).min(image.width as usize - 1) * 4;
            let right = (x * 2 + 1).min(image.width as usize - 1) * 4;
            for channel in 0..4 {
                let sum = u16::from(image.bgra[upper + left + channel])
                    + u16::from(image.bgra[upper + right + channel])
                    + u16::from(image.bgra[lower + left + channel])
                    + u16::from(image.bgra[lower + right + channel]);
                bgra.push((sum / 4) as u8);
            }
        }
    }
    GenieImage {
        width,
        height,
        bgra,
    }
}
