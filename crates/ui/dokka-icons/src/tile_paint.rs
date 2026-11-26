//! Pinta la baldosa redondeada y el glifo centrado con borde suave.

use dokka_scene::{Bitmap, Color};
use dokka_theme::launchpad as measures;

use crate::tile_frame::content_bounds;
use crate::tile_pixels::{checked_len, pixel_index, read_pixel};

/// Radio en píxeles para el lado dado, desde la medida del tema.
pub(crate) fn tile_radius_px(side: u32) -> f32 {
    let side_f = side as f32;
    let radius = measures::TILE_RADIUS * side_f / measures::TILE_REFERENCE_SIDE;
    radius.clamp(0.0, side_f / 2.0)
}

/// Cobertura del rectángulo redondeado en el centro del píxel.
fn rounded_coverage(x: u32, y: u32, side: f32, radius: f32) -> f32 {
    let px = x as f32 + 0.5;
    let py = y as f32 + 0.5;
    let half = side / 2.0;
    let qx = (px - half).abs() - (half - radius);
    let qy = (py - half).abs() - (half - radius);
    let ax = qx.max(0.0);
    let ay = qy.max(0.0);
    let outside = (ax * ax + ay * ay).sqrt() + qx.max(qy).min(0.0) - radius;
    (0.5 - outside).clamp(0.0, 1.0)
}

fn dim(channel: u8, coverage: f32) -> u8 {
    (channel as f32 * coverage + 0.5).clamp(0.0, 255.0) as u8
}

/// Solo recorta las esquinas, con el borde suavizado.
pub(crate) fn rounded_mask(source: &Bitmap, radius: f32) -> Option<Bitmap> {
    if source.width == 0 || source.width != source.height {
        return None;
    }
    let total = checked_len(source.width, source.height)?;
    if source.pixels.len() != total {
        return None;
    }
    let mut pixels = vec![0_u8; total];
    let side_f = source.width as f32;
    for y in 0..source.height {
        for x in 0..source.width {
            let at = pixel_index(source.width, x, y)?;
            let from = source.pixels.get(at..at + 4)?;
            let coverage = rounded_coverage(x, y, side_f, radius);
            let slot = pixels.get_mut(at..at + 4)?;
            slot[0] = dim(from[0], coverage);
            slot[1] = dim(from[1], coverage);
            slot[2] = dim(from[2], coverage);
            slot[3] = dim(from[3], coverage);
        }
    }
    Some(Bitmap {
        width: source.width,
        height: source.height,
        pixels,
    })
}

/// Silueta centrada sobre la baldosa clara, con las esquinas recortadas.
pub(crate) fn tile_with_content(side: u32, content: &Bitmap, tile: Color) -> Bitmap {
    let fallback = filled_tile(side, tile);
    let Some((left, top, right, bottom)) = content_bounds(content) else {
        return fallback;
    };
    let box_w = right.saturating_sub(left).max(1) as f32;
    let box_h = bottom.saturating_sub(top).max(1) as f32;
    let side_f = side as f32;
    let target = side_f * measures::TILE_CONTENT_SCALE.clamp(0.3, 0.85);
    let fit = (target / box_w).min(target / box_h);
    let dest_w = (box_w * fit).max(1.0);
    let dest_h = (box_h * fit).max(1.0);
    let dest_x = (side_f - dest_w) / 2.0;
    let dest_y = (side_f - dest_h) / 2.0;
    let radius = tile_radius_px(side);
    let Some(total) = checked_len(side, side) else {
        return fallback;
    };
    let mut pixels = vec![0_u8; total];
    for y in 0..side {
        for x in 0..side {
            let Some(at) = pixel_index(side, x, y) else {
                continue;
            };
            let Some(slot) = pixels.get_mut(at..at + 4) else {
                continue;
            };
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let mut red = tile.red as f32;
            let mut green = tile.green as f32;
            let mut blue = tile.blue as f32;
            let mut alpha = 255.0;
            let inside =
                px >= dest_x && px < dest_x + dest_w && py >= dest_y && py < dest_y + dest_h;
            if inside {
                let sample = bilinear(
                    content,
                    left as f32 + (px - dest_x) / fit,
                    top as f32 + (py - dest_y) / fit,
                );
                let ink_a = sample[3] / 255.0;
                blue = sample[0] + blue * (1.0 - ink_a);
                green = sample[1] + green * (1.0 - ink_a);
                red = sample[2] + red * (1.0 - ink_a);
                alpha = sample[3] + 255.0 * (1.0 - ink_a);
            }
            let coverage = rounded_coverage(x, y, side_f, radius);
            slot[0] = (blue * coverage + 0.5).clamp(0.0, 255.0) as u8;
            slot[1] = (green * coverage + 0.5).clamp(0.0, 255.0) as u8;
            slot[2] = (red * coverage + 0.5).clamp(0.0, 255.0) as u8;
            slot[3] = (alpha * coverage + 0.5).clamp(0.0, 255.0) as u8;
        }
    }
    Bitmap {
        width: side,
        height: side,
        pixels,
    }
}

/// Baldosa lisa, ya con las esquinas recortadas.
fn filled_tile(side: u32, tile: Color) -> Bitmap {
    let radius = tile_radius_px(side);
    let side_f = side as f32;
    let total = checked_len(side, side).unwrap_or_default();
    let mut pixels = vec![0_u8; total];
    for y in 0..side {
        for x in 0..side {
            if let Some(at) = pixel_index(side, x, y)
                && let Some(slot) = pixels.get_mut(at..at + 4)
            {
                let coverage = rounded_coverage(x, y, side_f, radius);
                slot[0] = dim(tile.blue, coverage);
                slot[1] = dim(tile.green, coverage);
                slot[2] = dim(tile.red, coverage);
                slot[3] = dim(255, coverage);
            }
        }
    }
    Bitmap {
        width: side,
        height: side,
        pixels,
    }
}

/// Muestra bilineal sobre píxeles premultiplicados, en BGRA como `f32`.
fn bilinear(bitmap: &Bitmap, x: f32, y: f32) -> [f32; 4] {
    let width = bitmap.width as f32;
    let height = bitmap.height as f32;
    let clamped_x = x.clamp(0.0, (width - 1.0).max(0.0));
    let clamped_y = y.clamp(0.0, (height - 1.0).max(0.0));
    let x0 = clamped_x.floor() as u32;
    let y0 = clamped_y.floor() as u32;
    let x1 = (x0 + 1).min(bitmap.width.saturating_sub(1));
    let y1 = (y0 + 1).min(bitmap.height.saturating_sub(1));
    let fx = clamped_x - x0 as f32;
    let fy = clamped_y - y0 as f32;
    let a = read_float(bitmap, x0, y0);
    let b = read_float(bitmap, x1, y0);
    let c = read_float(bitmap, x0, y1);
    let d = read_float(bitmap, x1, y1);
    [
        mix(a[0], b[0], c[0], d[0], fx, fy),
        mix(a[1], b[1], c[1], d[1], fx, fy),
        mix(a[2], b[2], c[2], d[2], fx, fy),
        mix(a[3], b[3], c[3], d[3], fx, fy),
    ]
}

fn mix(a: f32, b: f32, c: f32, d: f32, fx: f32, fy: f32) -> f32 {
    a * (1.0 - fx) * (1.0 - fy) + b * fx * (1.0 - fy) + c * (1.0 - fx) * fy + d * fx * fy
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
