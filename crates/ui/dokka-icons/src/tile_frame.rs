//! Decide si el icono llena el cuadro o es una silueta sobre marco.
//!
//! El marco blanco de algunos accesos pequeños se detecta como fondo blanco
//! opaco alrededor del dibujo y se recorta antes de componer la baldosa.
//! El marco fino de otro color se quita por su hueco interior.

use dokka_scene::Bitmap;

use crate::tile_pixels::{
    box_fill, checked_len, copy_rect, pixel_index, read_pixel, transparent_share, visible_bounds,
};

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

/// Cobertura mínima de cada lado del borde para verlo como marco fino.
const RING_SIDE_COVERAGE: f32 = 0.85;
/// Hueco mínimo justo dentro del anillo para ver marco y no fondo.
const INNER_GAP_CLEAR: f32 = 0.75;
/// Grosor del aire que se mira pegado al anillo.
const INNER_GAP_WIDTH: u32 = 2;

/// Quita el marco fino alrededor de una imagen pequeña y deja lo de dentro (no toca fondos opacos).
pub(crate) fn strip_thin_frame(bitmap: &Bitmap) -> Option<Bitmap> {
    let mut current = strip_once(bitmap)?;
    for _ in 1..4 {
        let Some(next) = strip_once(&current) else {
            break;
        };
        current = next;
    }
    Some(current)
}

/// Quita un solo anillo de 1–2 px si bordea lo visible y por dentro hay hueco.
fn strip_once(bitmap: &Bitmap) -> Option<Bitmap> {
    let bounds = visible_bounds(bitmap)?;
    let (left, top, right, bottom) = bounds;
    for thickness in [1, 2] {
        if !ring_closed(bitmap, bounds, thickness) || !gap_inside(bitmap, bounds, thickness) {
            continue;
        }
        let inner = copy_rect(
            bitmap,
            left.checked_add(thickness)?,
            top.checked_add(thickness)?,
            right.checked_sub(thickness)?,
            bottom.checked_sub(thickness)?,
        )?;
        // El anillo era todo el dibujo: no queda nada dentro.
        visible_bounds(&inner)?;
        return Some(inner);
    }
    None
}

/// `true` si cada lado del borde de la caja está casi todo visible.
fn ring_closed(bitmap: &Bitmap, bounds: (u32, u32, u32, u32), thickness: u32) -> bool {
    let (left, top, right, bottom) = bounds;
    let bands = [
        (left, top, right, top.saturating_add(thickness)),
        (left, bottom.saturating_sub(thickness), right, bottom),
        (left, top, left.saturating_add(thickness), bottom),
        (right.saturating_sub(thickness), top, right, bottom),
    ];
    bands
        .into_iter()
        .all(|band| box_fill(bitmap, band) >= RING_SIDE_COVERAGE)
}

/// `true` si justo dentro del anillo hay sobre todo hueco: es marco, no fondo.
fn gap_inside(bitmap: &Bitmap, bounds: (u32, u32, u32, u32), thickness: u32) -> bool {
    let (left, top, right, bottom) = bounds;
    let ileft = left.saturating_add(thickness);
    let itop = top.saturating_add(thickness);
    let iright = right.saturating_sub(thickness);
    let ibottom = bottom.saturating_sub(thickness);
    if iright <= ileft || ibottom <= itop {
        return false;
    }
    let air = INNER_GAP_WIDTH;
    let bands = [
        (ileft, itop, iright, itop.saturating_add(air)),
        (ileft, ibottom.saturating_sub(air), iright, ibottom),
        (ileft, itop, ileft.saturating_add(air), ibottom),
        (iright.saturating_sub(air), itop, iright, ibottom),
    ];
    let share: f32 = bands
        .into_iter()
        .map(|band| transparent_share(bitmap, band))
        .sum::<f32>()
        / 4.0;
    share >= INNER_GAP_CLEAR
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
