//! Carátula de la canción actual desde Windows.
//!
//! Se lee la miniatura que publica la app (`Thumbnail`), se decodifica con
//! las APIs de Windows (`BitmapDecoder`) y se deja en el `Bitmap` propio de
//! Dokka, cuadrado y reducido. Vive en el hilo del monitor, nunca en el de
//! la interfaz.

use std::sync::Arc;

use dokka_scene::Bitmap;
use windows::Graphics::Imaging::{
    BitmapAlphaMode, BitmapBounds, BitmapDecoder, BitmapInterpolationMode, BitmapPixelFormat,
    BitmapTransform, ColorManagementMode, ExifOrientationMode,
};
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager as Manager;

use crate::read_sessions::each_session;

/// Lado en píxeles de la carátula guardada.
const ARTWORK_SIDE: u32 = 128;

/// Radio de las esquinas en píxeles, proporcional al marcador del panel
/// (radio 4 a lado 56 en el tema): redondea la foto para que no tape el
/// cristal con esquinas cuadradas.
const CORNER_RADIUS: u32 = 9;

/// Miniatura de la sesión pedida, recortada al centro, reducida y con las
/// esquinas redondeadas. `None` si la app no publica ninguna o falla algo.
pub(crate) fn read_artwork(manager: &Manager, source: &str) -> Option<Arc<Bitmap>> {
    let session = each_session(manager)
        .into_iter()
        .find(|session| session.SourceAppUserModelId().is_ok_and(|id| id == source))?;
    let properties = session.TryGetMediaPropertiesAsync().ok()?.join().ok()?;
    let reference = properties.Thumbnail().ok()?;
    let stream = reference.OpenReadAsync().ok()?.join().ok()?;
    let decoder = BitmapDecoder::CreateAsync(&stream).ok()?.join().ok()?;
    let frame = decoder.GetFrameAsync(0).ok()?.join().ok()?;
    let width = frame.PixelWidth().ok()?;
    let height = frame.PixelHeight().ok()?;
    if width == 0 || height == 0 {
        return None;
    }
    // Windows escala primero y recorta después, así que el recorte va en las
    // medidas ya reducidas: el lado corto queda en `ARTWORK_SIDE`.
    let short = u64::from(width.min(height));
    let scale = |length: u32| (u64::from(length) * u64::from(ARTWORK_SIDE) / short) as u32;
    let scaled_width = scale(width).max(ARTWORK_SIDE);
    let scaled_height = scale(height).max(ARTWORK_SIDE);
    let transform = BitmapTransform::new().ok()?;
    transform.SetScaledWidth(scaled_width).ok()?;
    transform.SetScaledHeight(scaled_height).ok()?;
    transform
        .SetBounds(BitmapBounds {
            X: (scaled_width - ARTWORK_SIDE) / 2,
            Y: (scaled_height - ARTWORK_SIDE) / 2,
            Width: ARTWORK_SIDE,
            Height: ARTWORK_SIDE,
        })
        .ok()?;
    transform
        .SetInterpolationMode(BitmapInterpolationMode::Fant)
        .ok()?;
    let provider = frame
        .GetPixelDataTransformedAsync(
            BitmapPixelFormat::Bgra8,
            BitmapAlphaMode::Premultiplied,
            &transform,
            ExifOrientationMode::RespectExifOrientation,
            ColorManagementMode::DoNotColorManage,
        )
        .ok()?
        .join()
        .ok()?;
    let detached = provider.DetachPixelData().ok()?;
    let mut pixels = detached.as_slice().to_vec();
    let expected = ARTWORK_SIDE as usize * ARTWORK_SIDE as usize * 4;
    if pixels.len() != expected {
        return None;
    }
    round_corners(&mut pixels, ARTWORK_SIDE, CORNER_RADIUS);
    Some(Arc::new(Bitmap {
        width: ARTWORK_SIDE,
        height: ARTWORK_SIDE,
        pixels,
    }))
}

/// Pone alfa a cero fuera del rectángulo redondeado, en su sitio.
/// Los píxeles ya vienen premultiplicados, así que basta con borrarlos.
fn round_corners(pixels: &mut [u8], side: u32, radius: u32) {
    let side = side as usize;
    let radius = radius as usize;
    if side == 0 || radius == 0 {
        return;
    }
    for y in 0..side {
        for x in 0..side {
            let nearest_x = x.min(side - 1 - x);
            let nearest_y = y.min(side - 1 - y);
            if nearest_x < radius && nearest_y < radius {
                let dx = radius - nearest_x;
                let dy = radius - nearest_y;
                if dx * dx + dy * dy > radius * radius {
                    let start = (y * side + x) * 4;
                    pixels[start..start + 4].fill(0);
                }
            }
        }
    }
}
