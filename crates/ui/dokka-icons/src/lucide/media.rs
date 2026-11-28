//! Iconos de música y botones de ventana (vista previa).

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::popup;

use super::{CENTER, glyph};

/// Botón de pista anterior.
pub fn media_previous_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::SkipBack, popup::MEDIA_GLYPH_SIZE, color)
}

/// Botón de reproducir.
pub fn media_play_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Play, popup::MEDIA_GLYPH_SIZE, color)
}

/// Botón de pausa.
pub fn media_pause_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Pause, popup::MEDIA_GLYPH_SIZE, color)
}

/// Botón de pista siguiente.
pub fn media_next_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::SkipForward,
        popup::MEDIA_GLYPH_SIZE,
        color,
    )
}

/// Botón de cerrar la ventana (X).
pub fn preview_close_icon(glyph_color: Color, background: Color, border: Color) -> Drawing {
    window_button(lucide_icons::Icon::X, glyph_color, background, border)
}

/// Botón de fijar la ventana (chincheta).
pub fn preview_pin_icon(glyph_color: Color, background: Color, border: Color) -> Drawing {
    window_button(lucide_icons::Icon::Pin, glyph_color, background, border)
}

/// Botón de ventana fijada: misma chincheta tachada para desfijar.
pub fn preview_pinned_icon(glyph_color: Color, background: Color, border: Color) -> Drawing {
    window_button(lucide_icons::Icon::PinOff, glyph_color, background, border)
}

/// Círculo claro con borde y glifo centrado de 20 en un botón de 28.
fn window_button(
    icon: lucide_icons::Icon,
    glyph_color: Color,
    background: Color,
    border: Color,
) -> Drawing {
    Drawing {
        shapes: vec![
            Shape::Circle {
                x: CENTER,
                y: CENTER,
                radius: CENTER,
                color: border,
            },
            Shape::Circle {
                x: CENTER,
                y: CENTER,
                radius: CENTER - 6.0,
                color: background,
            },
            Shape::Glyph {
                glyph: char::from(icon),
                x: CENTER,
                y: CENTER,
                size: popup::PREVIEW_BUTTON_GLYPH_SIZE,
                color: glyph_color,
            },
        ],
    }
}
