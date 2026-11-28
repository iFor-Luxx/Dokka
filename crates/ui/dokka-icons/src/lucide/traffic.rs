//! Semáforos de Preferencias: círculos de color con su símbolo Lucide.

use dokka_scene::{Color, DRAWING_SIDE, Drawing, Shape};
use dokka_theme::settings;

use super::CENTER;

/// Círculo liso de un semáforo, sin símbolo: en color con foco y en gris
/// sin él. Llena el lienzo para pintarse nítido a su diámetro real.
pub fn traffic_circle(circle: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Circle {
            x: CENTER,
            y: CENTER,
            radius: CENTER,
            color: circle,
        }],
    }
}

/// Semáforo completo: círculo con su símbolo oscuro centrado al pasar el
/// cursor. El trazo de Lucide es muy fino a este tamaño: el mismo glifo se
/// repite apenas desplazado en cruz para engrosarlo sin dibujar uno propio.
fn traffic_button(circle: Color, icon: lucide_icons::Icon, symbol: Color) -> Drawing {
    let to_canvas = DRAWING_SIDE / settings::TRAFFIC_DIAMETER;
    let spread = settings::TRAFFIC_GLYPH_SPREAD * to_canvas;
    let glyphs = [
        (0.0, 0.0),
        (-spread, 0.0),
        (spread, 0.0),
        (0.0, -spread),
        (0.0, spread),
    ]
    .into_iter()
    .map(|(dx, dy)| Shape::Glyph {
        glyph: char::from(icon),
        x: CENTER + dx,
        y: CENTER + dy,
        size: settings::TRAFFIC_GLYPH_SIDE * to_canvas,
        color: symbol,
    });
    let mut shapes = vec![Shape::Circle {
        x: CENTER,
        y: CENTER,
        radius: CENTER,
        color: circle,
    }];
    shapes.extend(glyphs);
    Drawing { shapes }
}

/// Círculo rojo con su × oscura al pasar el cursor.
pub fn traffic_close_button(circle: Color, symbol: Color) -> Drawing {
    traffic_button(circle, lucide_icons::Icon::X, symbol)
}

/// Círculo amarillo con su − oscura al pasar el cursor.
pub fn traffic_minimize_button(circle: Color, symbol: Color) -> Drawing {
    traffic_button(circle, lucide_icons::Icon::Minus, symbol)
}

/// Círculo verde con sus flechas diagonales (pantalla completa) al pasar el cursor.
pub fn traffic_zoom_button(circle: Color, symbol: Color) -> Drawing {
    traffic_button(circle, lucide_icons::Icon::Maximize2, symbol)
}
