//! Lupa del buscador desde Lucide, por significado.
//!
//! El dibujo de la barra (blanco) lo usa el icono de la barra superior y el
//! del panel (oscuro) va dentro de la casilla clara. Ambos reutilizan el
//! mismo glifo `Search` sin dibujos propios.

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::search as measures;

const CENTER: f32 = 50.0;

fn glyph(icon: lucide_icons::Icon, size: f32, color: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Glyph {
            glyph: char::from(icon),
            x: CENTER,
            y: CENTER,
            size,
            color,
        }],
    }
}

/// Lupa blanca de la barra superior para abrir el buscador.
pub fn search_bar_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Search,
        measures::BAR_GLYPH_SIZE,
        color,
    )
}

/// Lupa oscura de la casilla grande dentro del panel claro.
pub fn search_field_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Search,
        measures::FIELD_GLYPH_SIZE,
        color,
    )
}
