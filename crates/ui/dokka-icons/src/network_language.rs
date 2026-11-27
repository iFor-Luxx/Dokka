//! Iconos Lucide de red e idioma, por significado.
//!
//! Cada función crea un `Drawing` con un solo glifo que llena el lienzo
//! de 100. El color lo pone quien llama: azul sobre el círculo blanco
//! activo y blanco sobre el círculo translúcido apagado; blanco en la barra.

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::network_language as measures;

const CENTER: f32 = 50.0;

fn glyph(icon: lucide_icons::Icon, color: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Glyph {
            glyph: char::from(icon),
            x: CENTER,
            y: CENTER,
            size: measures::GLYPH_SIZE,
            color,
        }],
    }
}

/// Red wifi con conexión.
pub fn network_wifi_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Wifi, color)
}

/// Sin conexión.
pub fn network_offline_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::WifiOff, color)
}

/// Red por cable.
pub fn network_ethernet_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::EthernetPort, color)
}

/// Otro tipo de red.
pub fn network_other_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Globe, color)
}

/// Marca del idioma activo (se mantiene por compatibilidad).
pub fn language_check_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Check, color)
}

/// Engranaje de la zona de ajustes dentro de su círculo.
pub fn tray_settings_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Settings, color)
}

/// Chevrón de la zona de ajustes.
pub fn tray_chevron_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::ChevronRight, color)
}
