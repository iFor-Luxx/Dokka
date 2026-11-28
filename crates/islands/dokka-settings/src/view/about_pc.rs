//! Dibujo propio de un PC: monitor, pie y base en el lienzo de 100.

use dokka_scene::{Drawing, Icon};
use dokka_theme::Theme;

/// Monitor con pie y base, en grises de la tarjeta.
pub(crate) fn pc_drawing(theme: &Theme) -> Drawing {
    use dokka_scene::Shape;
    let frame = theme.settings.card_text;
    let screen = theme.settings.card;
    Drawing {
        shapes: vec![
            Shape::RoundedRect {
                x: 8.0,
                y: 6.0,
                width: 84.0,
                height: 56.0,
                radius: 6.0,
                color: frame,
            },
            Shape::RoundedRect {
                x: 12.0,
                y: 10.0,
                width: 76.0,
                height: 44.0,
                radius: 3.0,
                color: screen,
            },
            Shape::RoundedRect {
                x: 46.0,
                y: 62.0,
                width: 8.0,
                height: 16.0,
                radius: 2.0,
                color: frame,
            },
            Shape::RoundedRect {
                x: 30.0,
                y: 78.0,
                width: 40.0,
                height: 6.0,
                radius: 3.0,
                color: frame,
            },
        ],
    }
}

/// Icono oculto fuera de la vista, para no reconstruir capas.
pub(crate) fn hidden_icon() -> Icon {
    Icon {
        x: -1000.0,
        y: -1000.0,
        side: 16.0,
        image: None,
        placeholder: dokka_scene::Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
        placeholder_radius: 0.0,
    }
}
