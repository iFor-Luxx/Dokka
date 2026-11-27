//! Iconos del panel de volumen desde la librería Lucide, por significado.
//!
//! Cada función crea un `Drawing` con un solo glifo que llena el lienzo
//! de 100 (centrado en 50, 50). El tamaño del glifo sale de `dokka-theme`
//! y el color lo pone quien llama (oscuro dentro del panel claro,
//! blanco en la barra superior).

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::volume as measures;

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

/// Altavoz silenciado: nivel con silencio activado.
pub fn volume_muted_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::VolumeX,
        measures::SPEAKER_GLYPH_SIZE,
        color,
    )
}

/// Altavoz sin ondas: nivel a cero sin silenciar.
pub fn volume_off_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::VolumeOff,
        measures::SPEAKER_GLYPH_SIZE,
        color,
    )
}

/// Altavoz bajo: nivel a medias.
pub fn volume_low_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Volume1,
        measures::SPEAKER_GLYPH_SIZE,
        color,
    )
}

/// Altavoz alto: nivel por encima de la mitad.
pub fn volume_high_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Volume2,
        measures::SPEAKER_GLYPH_SIZE,
        color,
    )
}

/// Chevrón de la fila de ajustes.
pub fn volume_settings_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::ChevronRight,
        measures::SETTINGS_GLYPH_SIZE,
        color,
    )
}
