//! Tinte del fondo de la baldosa a partir de los colores del icono.
//!
//! La mitad superior y la inferior dan un color medio cada una; mezclados con
//! la baldosa clara del tema forman el degradado suave del fondo. Cuando hay
//! bastante color, los grises pesan menos y manda el color del logo; un logo
//! sin color queda casi como la baldosa base.

use dokka_scene::{Bitmap, Color};
use dokka_theme::launchpad as measures;

use crate::tile_pixels::read_pixel;

/// Alfa mínimo para contar un píxel como dibujo.
const MIN_ALPHA: u8 = 16;
/// Diferencia mínima entre canales para ver un píxel con color.
const MIN_CHROMA: f32 = 32.0;
/// Parte del peso con color para preferirlo sobre grises y blancos.
const COLOR_WEIGHT_SHARE: f32 = 0.12;

/// Colores del fondo ya mezclados con la base: arriba y abajo.
pub(crate) fn tile_gradient(content: &Bitmap, base: Color) -> (Color, Color) {
    let share = measures::TILE_TINT_SHARE.clamp(0.0, 1.0);
    let top = half_average(content, true).map_or(base, |tint| mix_tint(base, tint, share));
    let bottom = half_average(content, false).map_or(base, |tint| mix_tint(base, tint, share));
    (top, bottom)
}

/// Fondo del degradado a la altura de la fila, en BGRA como `f32` sin alfa.
pub(crate) fn gradient_row(top: Color, bottom: Color, y: u32, side: u32) -> [f32; 3] {
    let share = if side <= 1 {
        0.0
    } else {
        (y as f32 / (side - 1) as f32).clamp(0.0, 1.0)
    };
    let keep = 1.0 - share;
    [
        top.blue as f32 * keep + bottom.blue as f32 * share,
        top.green as f32 * keep + bottom.green as f32 * share,
        top.red as f32 * keep + bottom.red as f32 * share,
    ]
}

/// Color medio despremultiplicado de una mitad, ponderado por alfa.
fn half_average(content: &Bitmap, upper: bool) -> Option<Color> {
    if content.width == 0 || content.height == 0 {
        return None;
    }
    let split = content.height / 2;
    let (first, last) = if upper {
        (0, split)
    } else {
        (split, content.height)
    };
    let mut all = [0.0_f32; 3];
    let mut vivid = [0.0_f32; 3];
    let mut all_weight = 0.0_f32;
    let mut vivid_weight = 0.0_f32;
    for y in first..last {
        for x in 0..content.width {
            let Some(pixel) = read_pixel(content, x, y) else {
                continue;
            };
            if pixel[3] <= MIN_ALPHA {
                continue;
            }
            // BGRA premultiplicado: se despremultiplica antes de promediar.
            let alpha = pixel[3] as f32;
            let red = (pixel[2] as f32 * 255.0 / alpha).clamp(0.0, 255.0);
            let green = (pixel[1] as f32 * 255.0 / alpha).clamp(0.0, 255.0);
            let blue = (pixel[0] as f32 * 255.0 / alpha).clamp(0.0, 255.0);
            let weight = alpha / 255.0;
            all[0] += blue * weight;
            all[1] += green * weight;
            all[2] += red * weight;
            all_weight += weight;
            let brightest = red.max(green).max(blue);
            let dullest = red.min(green).min(blue);
            if brightest - dullest >= MIN_CHROMA {
                vivid[0] += blue * weight;
                vivid[1] += green * weight;
                vivid[2] += red * weight;
                vivid_weight += weight;
            }
        }
    }
    let (sums, weight) = if vivid_weight >= all_weight * COLOR_WEIGHT_SHARE {
        (vivid, vivid_weight)
    } else {
        (all, all_weight)
    };
    if weight <= 0.0 {
        return None;
    }
    Some(Color {
        red: (sums[2] / weight + 0.5).clamp(0.0, 255.0) as u8,
        green: (sums[1] / weight + 0.5).clamp(0.0, 255.0) as u8,
        blue: (sums[0] / weight + 0.5).clamp(0.0, 255.0) as u8,
        alpha: 255,
    })
}

/// Mezcla el color del icono con la base clara: `share` es la parte del icono.
fn mix_tint(base: Color, tint: Color, share: f32) -> Color {
    let keep = 1.0 - share;
    Color {
        red: (base.red as f32 * keep + tint.red as f32 * share + 0.5).clamp(0.0, 255.0) as u8,
        green: (base.green as f32 * keep + tint.green as f32 * share + 0.5).clamp(0.0, 255.0) as u8,
        blue: (base.blue as f32 * keep + tint.blue as f32 * share + 0.5).clamp(0.0, 255.0) as u8,
        alpha: 255,
    }
}
