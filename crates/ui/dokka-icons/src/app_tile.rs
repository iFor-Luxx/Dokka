//! Baldosa uniforme del Launchpad: cuadrado muy redondeado estilo macOS.
//!
//! Si el icono llena el cuadro solo se recortan las esquinas; si es una
//! silueta se centra, algo más pequeña, sobre la baldosa clara del tema.

use dokka_scene::Bitmap;
use dokka_theme::Theme;

use crate::tile_frame::{border_fill, crop_white_frame};
use crate::tile_paint::{rounded_mask, tile_radius_px, tile_with_content};
use crate::tile_pixels::center_square;

/// Pasa un icono de app a la baldosa uniforme. `None` si no se puede leer.
pub fn app_tile(source: &Bitmap, theme: &Theme) -> Option<Bitmap> {
    let square = center_square(source)?;
    let side = square.width;
    if side == 0 {
        return None;
    }
    if let Some(cropped) = crop_white_frame(&square) {
        return Some(tile_with_content(side, &cropped, theme.launchpad.tile));
    }
    if border_fill(&square) > 0.55 {
        return rounded_mask(&square, tile_radius_px(side));
    }
    Some(tile_with_content(side, &square, theme.launchpad.tile))
}
