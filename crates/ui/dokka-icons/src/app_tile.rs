//! Baldosa uniforme del Launchpad: cuadrado muy redondeado estilo macOS.
//!
//! Si el icono llena el cuadro (aunque traiga margen transparente) ocupa toda
//! la baldosa; si es una silueta se centra, algo más pequeña, sobre un
//! degradado claro sacado de los colores del propio icono. El marco fino de
//! las imágenes pequeñas se quita antes.

use dokka_scene::Bitmap;
use dokka_theme::Theme;

use crate::tile_color::tile_gradient;

use crate::tile_frame::{border_fill, crop_white_frame, strip_thin_frame};
use crate::tile_paint::{rounded_mask, tile_radius_px, tile_with_content};
use crate::tile_pixels::center_square;
use crate::tile_square::expand_full_bleed;

/// Pasa un icono de app a la baldosa uniforme. `None` si no se puede leer.
pub fn app_tile(source: &Bitmap, theme: &Theme) -> Option<Bitmap> {
    let square = center_square(source)?;
    let side = square.width;
    if side == 0 {
        return None;
    }
    // El marco fino sale antes: luego se decide con lo de dentro.
    let working = match strip_thin_frame(&square) {
        Some(deframed) => deframed,
        None => square,
    };
    // Cuadrado con margen transparente: llena la baldosa sin base clara.
    if let Some(expanded) = expand_full_bleed(&working, side) {
        return rounded_mask(&expanded, tile_radius_px(side));
    }
    if let Some(cropped) = crop_white_frame(&working) {
        let (top, bottom) = tile_gradient(&cropped, theme.launchpad.tile);
        return Some(tile_with_content(side, &cropped, top, bottom));
    }
    if border_fill(&working) > 0.55 {
        return rounded_mask(&working, tile_radius_px(side));
    }
    let (top, bottom) = tile_gradient(&working, theme.launchpad.tile);
    Some(tile_with_content(side, &working, top, bottom))
}
