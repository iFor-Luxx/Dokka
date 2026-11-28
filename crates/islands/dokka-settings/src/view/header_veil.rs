//! Franja difuminada bajo la cabecera de Preferencias, estilo Tahoe.
//!
//! Describe el rectángulo de la zona de contenido con el tinte del fondo,
//! el radio y la intensidad según el desplazamiento, y lleva por encima la
//! píldora (cristal y resaltado), sus líneas, sus iconos y el título,
//! nítidos. Sin desplazar la intensidad es 0 y no se nota.

use dokka_scene::{Divider, HeaderVeil, Icon, Panel, Rect, Text};
use dokka_theme::{Theme, settings as measures};

/// Piezas de la cabecera que van nítidas sobre la franja.
pub(crate) struct HeaderRequest {
    pub pill_panel: Panel,
    pub pill_flat: Panel,
    pub header_divider: Divider,
    pub pill_divider: Divider,
    pub pill_icons: Vec<Icon>,
    pub title: Text,
    pub scroll: f32,
    pub window_width: f32,
}

/// Franja con sus piezas por encima; siempre existe para no reconstruir al
/// empezar a desplazar: arriba del todo su intensidad es 0 y no se nota.
pub(crate) fn header_veil(theme: &Theme, request: HeaderRequest) -> HeaderVeil {
    let width = measures::header_veil_width_for(request.window_width);
    HeaderVeil {
        bounds: Rect {
            x: measures::HEADER_VEIL_LEFT,
            y: measures::HEADER_VEIL_TOP,
            width,
            height: measures::header_veil_height(),
        },
        tint: theme.settings.window,
        blur_radius: measures::HEADER_VEIL_BLUR_RADIUS,
        strength: measures::header_veil_strength(request.scroll),
        panels: vec![request.pill_panel],
        flats: vec![request.pill_flat],
        dividers: vec![request.header_divider, request.pill_divider],
        icons: request.pill_icons,
        texts: vec![request.title],
    }
}
