//! Acceso a los píxeles de un `Bitmap` sin salirse del búfer.

use dokka_scene::Bitmap;

/// Cuadrado centrado; si ya lo es, una copia.
pub(crate) fn center_square(source: &Bitmap) -> Option<Bitmap> {
    let total = checked_len(source.width, source.height)?;
    if source.width == 0 || source.height == 0 || source.pixels.len() != total {
        return None;
    }
    if source.width == source.height {
        return Some(Bitmap {
            width: source.width,
            height: source.height,
            pixels: source.pixels.clone(),
        });
    }
    let side = source.width.min(source.height);
    let left = (source.width - side) / 2;
    let top = (source.height - side) / 2;
    let mut pixels = vec![0_u8; checked_len(side, side)?];
    for y in 0..side {
        for x in 0..side {
            let from = pixel_index(source.width, left + x, top + y)?;
            let into = pixel_index(side, x, y)?;
            let kept = source.pixels.get(from..from + 4)?;
            let slot = pixels.get_mut(into..into + 4)?;
            slot.copy_from_slice(kept);
        }
    }
    Some(Bitmap {
        width: side,
        height: side,
        pixels,
    })
}

/// Longitud esperada de píxeles, si no desborda.
pub(crate) fn checked_len(width: u32, height: u32) -> Option<usize> {
    (width as usize)
        .checked_mul(height as usize)?
        .checked_mul(4)
}

/// Índice del primer byte del píxel, si cabe en el búfer.
pub(crate) fn pixel_index(width: u32, x: u32, y: u32) -> Option<usize> {
    let at = (y as usize)
        .checked_mul(width as usize)?
        .checked_add(x as usize)?
        .checked_mul(4)?;
    Some(at)
}

/// Píxel BGRA en (`x`, `y`), si existe.
pub(crate) fn read_pixel(bitmap: &Bitmap, x: u32, y: u32) -> Option<[u8; 4]> {
    let at = pixel_index(bitmap.width, x, y)?;
    let bytes = bitmap.pixels.get(at..at + 4)?;
    Some([bytes[0], bytes[1], bytes[2], bytes[3]])
}
