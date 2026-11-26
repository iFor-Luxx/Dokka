//! Decide si el icono llena el cuadro o es una silueta sobre marco.
//!
//! El marco blanco de algunos accesos pequeños se detecta como fondo blanco
//! opaco alrededor del dibujo y se recorta antes de componer la baldosa.

use dokka_scene::Bitmap;

use crate::tile_pixels::{checked_len, pixel_index, read_pixel};

/// `true` si es blanco opaco de marco o fondo de documento.
fn is_frame_white(pixel: [u8; 4]) -> bool {
    pixel[3] > 200 && pixel[0] > 225 && pixel[1] > 225 && pixel[2] > 225
}

/// `true` si aporta dibujo: visible y no blanco de marco.
fn is_ink(pixel: [u8; 4]) -> bool {
    pixel[3] > 16 && !is_frame_white(pixel)
}

/// Caja del dibujo no blanco, en `[izquierda, derecha)` y `[arriba, abajo)`.
pub(crate) fn content_bounds(bitmap: &Bitmap) -> Option<(u32, u32, u32, u32)> {
    let mut left = bitmap.width;
    let mut top = bitmap.height;
    let mut right = 0_u32;
    let mut bottom = 0_u32;
    let mut found = false;
    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            let pixel = read_pixel(bitmap, x, y)?;
            if is_ink(pixel) {
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

/// Un marco encierra sobre todo vacío. Un icono con fondo blanco propio no lo es:
/// recortarlo le quitaría su fondo.
fn is_mostly_transparent(bitmap: &Bitmap) -> bool {
    // El cuarto canal de cada píxel BGRA es la transparencia.
    let alphas = bitmap.pixels.iter().skip(3).step_by(4);
    let total = alphas.clone().count();
    let clear = alphas.filter(|&&alpha| alpha <= 16).count();
    clear * 100 > total * 40
}

/// Recorta el marco blanco y deja el contenido con un filo de aire.
pub(crate) fn crop_white_frame(bitmap: &Bitmap) -> Option<Bitmap> {
    if bitmap.width == 0 || bitmap.width != bitmap.height || !is_mostly_transparent(bitmap) {
        return None;
    }
    let (left, top, right, bottom) = content_bounds(bitmap)?;
    let kept_w = right.saturating_sub(left);
    let kept_h = bottom.saturating_sub(top);
    if kept_w < 8 || kept_h < 8 {
        return None;
    }
    let side = bitmap.width;
    let area = (kept_w as u64) * (kept_h as u64);
    let full = (side as u64) * (side as u64);
    if area * 100 >= full * 95 {
        return None;
    }
    let from_x = left.saturating_sub(2);
    let from_y = top.saturating_sub(2);
    let to_x = (right + 2).min(side);
    let to_y = (bottom + 2).min(side);
    let width = to_x.saturating_sub(from_x);
    let height = to_y.saturating_sub(from_y);
    let mut pixels = vec![0_u8; checked_len(width, height)?];
    if width == 0 || height == 0 {
        return None;
    }
    for y in 0..height {
        for x in 0..width {
            let from = pixel_index(bitmap.width, from_x + x, from_y + y)?;
            let into = pixel_index(width, x, y)?;
            let kept = bitmap.pixels.get(from..from + 4)?;
            let slot = pixels.get_mut(into..into + 4)?;
            slot.copy_from_slice(kept);
        }
    }
    Some(Bitmap {
        width,
        height,
        pixels,
    })
}

/// Parte del anillo exterior que es opaca: los cuadradones llenan, y no las siluetas.
pub(crate) fn border_fill(bitmap: &Bitmap) -> f32 {
    let side = bitmap.width;
    if side < 4 || bitmap.width != bitmap.height {
        return 0.0;
    }
    let mut opaque = 0_u32;
    let mut total = 0_u32;
    for y in 0..side {
        for x in 0..side {
            let edge = x < 2 || y < 2 || x + 2 >= side || y + 2 >= side;
            if !edge {
                continue;
            }
            total += 1;
            if let Some(pixel) = read_pixel(bitmap, x, y)
                && pixel[3] > 40
            {
                opaque += 1;
            }
        }
    }
    if total == 0 {
        return 0.0;
    }
    opaque as f32 / total as f32
}
