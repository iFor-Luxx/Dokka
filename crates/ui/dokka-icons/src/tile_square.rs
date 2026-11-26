//! Cuadrados que ya llenan el cuadro: van a la baldosa sin base clara detrás.
//!
//! Un cuadrado o cuadrado redondeado con un poco de margen transparente se
//! recorta por su caja visible y se escala al lado completo. Las siluetas
//! (círculos, escudos) no entran y siguen sobre la baldosa clara.

use dokka_scene::Bitmap;

use crate::tile_pixels::{box_fill, checked_len, pixel_index, read_pixel, visible_bounds};

/// Proporción mínima de la caja para verla cuadrada.
const ASPECT_MIN: f32 = 0.9;
/// Proporción máxima de la caja para verla cuadrada.
const ASPECT_MAX: f32 = 1.1;
/// Lado mínimo de la caja respecto al lienzo para ver fondo y no glifo.
const MIN_EXTENT: f32 = 0.7;
/// Relleno mínimo de la caja: un círculo llena ~0,785 y queda como silueta.
const MIN_FILL: f32 = 0.88;
/// Lado mínimo de la caja, como en el recorte del marco blanco.
const MIN_BOX_SIDE: u32 = 8;

/// Recorta la caja visible y la escala al lado dado, si es un cuadrado que llena.
/// `None` con siluetas o si ya toca los bordes (eso no se remuestrea).
pub(crate) fn expand_full_bleed(bitmap: &Bitmap, side: u32) -> Option<Bitmap> {
    if side == 0 || bitmap.width == 0 || bitmap.width != bitmap.height {
        return None;
    }
    let bounds = visible_bounds(bitmap)?;
    let (left, top, right, bottom) = bounds;
    let box_w = right.checked_sub(left)?;
    let box_h = bottom.checked_sub(top)?;
    if box_w < MIN_BOX_SIDE || box_h < MIN_BOX_SIDE {
        return None;
    }
    let aspect = box_w as f32 / box_h.max(1) as f32;
    if aspect < ASPECT_MIN || aspect > ASPECT_MAX {
        return None;
    }
    let extent = box_w.min(box_h) as f32 / bitmap.width as f32;
    if extent < MIN_EXTENT {
        return None;
    }
    if box_fill(bitmap, bounds) < MIN_FILL {
        return None;
    }
    let touches = left <= 1
        && top <= 1
        && right >= bitmap.width.saturating_sub(1)
        && bottom >= bitmap.height.saturating_sub(1);
    if touches {
        return None;
    }
    scale_box(bitmap, bounds, side)
}

/// Escala la caja al lado dado con remuestreo suave.
fn scale_box(bitmap: &Bitmap, bounds: (u32, u32, u32, u32), side: u32) -> Option<Bitmap> {
    let (left, top, right, bottom) = bounds;
    let box_w = right.checked_sub(left)?;
    let box_h = bottom.checked_sub(top)?;
    if box_w == 0 || box_h == 0 || side == 0 {
        return None;
    }
    if right > bitmap.width || bottom > bitmap.height {
        return None;
    }
    let mut pixels = vec![0_u8; checked_len(side, side)?];
    let box_w_f = box_w as f32;
    let box_h_f = box_h as f32;
    let side_f = side as f32;
    for y in 0..side {
        for x in 0..side {
            let sample_x = left as f32 + (x as f32 + 0.5) * box_w_f / side_f - 0.5;
            let sample_y = top as f32 + (y as f32 + 0.5) * box_h_f / side_f - 0.5;
            let sample = bilinear(bitmap, bounds, sample_x, sample_y);
            let at = pixel_index(side, x, y)?;
            let slot = pixels.get_mut(at..at.checked_add(4)?)?;
            slot[0] = sample[0].clamp(0.0, 255.0) as u8;
            slot[1] = sample[1].clamp(0.0, 255.0) as u8;
            slot[2] = sample[2].clamp(0.0, 255.0) as u8;
            slot[3] = sample[3].clamp(0.0, 255.0) as u8;
        }
    }
    Some(Bitmap {
        width: side,
        height: side,
        pixels,
    })
}

/// Muestra bilineal limitada a la caja, en BGRA como `f32`.
fn bilinear(bitmap: &Bitmap, bounds: (u32, u32, u32, u32), x: f32, y: f32) -> [f32; 4] {
    let (left, top, right, bottom) = bounds;
    let last_x = right.saturating_sub(1).max(left);
    let last_y = bottom.saturating_sub(1).max(top);
    let clamped_x = x.clamp(left as f32, last_x as f32);
    let clamped_y = y.clamp(top as f32, last_y as f32);
    let base_x = clamped_x.floor() as u32;
    let base_y = clamped_y.floor() as u32;
    let next_x = base_x.saturating_add(1).min(last_x);
    let next_y = base_y.saturating_add(1).min(last_y);
    let frac_x = (clamped_x - base_x as f32).clamp(0.0, 1.0);
    let frac_y = (clamped_y - base_y as f32).clamp(0.0, 1.0);
    let a = read_float(bitmap, base_x, base_y);
    let b = read_float(bitmap, next_x, base_y);
    let c = read_float(bitmap, base_x, next_y);
    let d = read_float(bitmap, next_x, next_y);
    [
        mix(a[0], b[0], c[0], d[0], frac_x, frac_y),
        mix(a[1], b[1], c[1], d[1], frac_x, frac_y),
        mix(a[2], b[2], c[2], d[2], frac_x, frac_y),
        mix(a[3], b[3], c[3], d[3], frac_x, frac_y),
    ]
}

fn mix(a: f32, b: f32, c: f32, d: f32, frac_x: f32, frac_y: f32) -> f32 {
    a * (1.0 - frac_x) * (1.0 - frac_y)
        + b * frac_x * (1.0 - frac_y)
        + c * (1.0 - frac_x) * frac_y
        + d * frac_x * frac_y
}

fn read_float(bitmap: &Bitmap, x: u32, y: u32) -> [f32; 4] {
    let pixel = read_pixel(bitmap, x, y).unwrap_or_default();
    [
        pixel[0] as f32,
        pixel[1] as f32,
        pixel[2] as f32,
        pixel[3] as f32,
    ]
}
