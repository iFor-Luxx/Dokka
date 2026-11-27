//! Nota musical de la barra superior, desde la librería Lucide.
//!
//! El glifo llena el lienzo de 100 (centrado en 50, 50) como los demás
//! iconos de interfaz; el color lo pone quien llama (blanco en la barra,
//! oscuro dentro de paneles claros).

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::music_panel as measures;

const CENTER: f32 = 50.0;

/// Nota musical para la barra superior.
pub fn music_bar_icon(color: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Glyph {
            glyph: char::from(lucide_icons::Icon::Music),
            x: CENTER,
            y: CENTER,
            size: measures::BAR_GLYPH_SIZE,
            color,
        }],
    }
}
