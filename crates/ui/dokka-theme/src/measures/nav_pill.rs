//! Medidas compartidas de la píldora segmentada de navegación.
//!
//! Cápsula con dos chevrones Lucide y una línea corta en el centro.
//! Cada contexto fija su tamaño (calendario en `popup`, Preferencias
//! en `settings`); el radio siempre es la mitad del alto.

use dokka_scene::DRAWING_SIDE;

/// Lado de cada chevrón, centrado en su mitad.
pub const ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100, así la fuente
/// efectiva es 20 al mostrarse a 20 de lado.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Grosor del borde luminoso de la cápsula.
pub const BORDER_WIDTH: f32 = 1.0;
/// Grosor de la línea vertical del centro.
pub const DIVIDER_WIDTH: f32 = 1.0;
/// Fracción del alto de la píldora que ocupa la línea del centro.
pub const DIVIDER_FRACTION: f32 = 0.5;

/// Radio de la cápsula: extremos totalmente redondos.
pub fn radius(pill_height: f32) -> f32 {
    pill_height / 2.0
}

/// Alto de la línea vertical del centro, corta a propósito.
pub fn divider_height(pill_height: f32) -> f32 {
    pill_height * DIVIDER_FRACTION
}
