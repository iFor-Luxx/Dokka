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

/// Alfa mínimo para contar un píxel como dibujo: ve marcos grises o semitransparentes.
pub(crate) const VISIBLE_ALPHA: u8 = 16;

/// Caja de lo visible (alfa por encima del umbral), en `[izquierda, derecha)` y `[arriba, abajo)`.
pub(crate) fn visible_bounds(bitmap: &Bitmap) -> Option<(u32, u32, u32, u32)> {
    let mut left = bitmap.width;
    let mut top = bitmap.height;
    let mut right = 0_u32;
    let mut bottom = 0_u32;
    let mut found = false;
    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            let pixel = read_pixel(bitmap, x, y)?;
            if pixel[3] > VISIBLE_ALPHA {
                left = left.min(x);
                top = top.min(y);
                right = right.max(x + 1);
                bottom = bottom.max(y + 1);
                found = true;
            }
        }
    }
    found.then_some((left, top, right, bottom))
}

/// Parte visible de la caja: un cuadrado la llena casi toda, un círculo ~0,785.
pub(crate) fn box_fill(bitmap: &Bitmap, bounds: (u32, u32, u32, u32)) -> f32 {
    let (left, top, right, bottom) = bounds;
    if right <= left || bottom <= top || right > bitmap.width || bottom > bitmap.height {
        return 0.0;
    }
    let mut visible = 0_u64;
    for y in top..bottom {
        for x in left..right {
            if let Some(pixel) = read_pixel(bitmap, x, y)
                && pixel[3] > VISIBLE_ALPHA
            {
                visible += 1;
            }
        }
    }
    let area = u64::from(right - left) * u64::from(bottom - top);
    if area == 0 {
        0.0
    } else {
        visible as f32 / area as f32
    }
}

/// Parte transparente de la caja: distingue el hueco tras un marco del fondo opaco.
pub(crate) fn transparent_share(bitmap: &Bitmap, bounds: (u32, u32, u32, u32)) -> f32 {
    let (left, top, right, bottom) = bounds;
    if right <= left || bottom <= top || right > bitmap.width || bottom > bitmap.height {
        return 0.0;
    }
    let mut clear = 0_u64;
    for y in top..bottom {
        for x in left..right {
            if let Some(pixel) = read_pixel(bitmap, x, y)
                && pixel[3] <= VISIBLE_ALPHA
            {
                clear += 1;
            }
        }
    }
    let area = u64::from(right - left) * u64::from(bottom - top);
    if area == 0 {
        0.0
    } else {
        clear as f32 / area as f32
    }
}

/// Copia el rectángulo `[izquierda, derecha)` × `[arriba, abajo)` tal cual.
pub(crate) fn copy_rect(
    bitmap: &Bitmap,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
) -> Option<Bitmap> {
    if right <= left || bottom <= top || right > bitmap.width || bottom > bitmap.height {
        return None;
    }
    let width = right - left;
    let height = bottom - top;
    let mut pixels = vec![0_u8; checked_len(width, height)?];
    for y in 0..height {
        for x in 0..width {
            let from = pixel_index(bitmap.width, left.checked_add(x)?, top.checked_add(y)?)?;
            let into = pixel_index(width, x, y)?;
            let kept = bitmap.pixels.get(from..from.checked_add(4)?)?;
            let slot = pixels.get_mut(into..into.checked_add(4)?)?;
            slot.copy_from_slice(kept);
        }
    }
    Some(Bitmap {
        width,
        height,
        pixels,
    })
}
