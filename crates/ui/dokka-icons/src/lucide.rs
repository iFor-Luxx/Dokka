//! Iconos de interfaz desde la librería Lucide, por significado.
//!
//! Cada función crea un `Drawing` con un solo glifo que llena el lienzo
//! de 100 (centrado en 50, 50). El tamaño del glifo sale de `dokka-theme`
//! y es el lienzo completo, así que la fuente efectiva iguala al lado con
//! que se muestra el icono (24); el color lo pone quien llama
//! (texto normal o blanco de la fila resaltada).

mod media;
mod settings;
mod traffic;

pub use media::{
    media_next_icon, media_pause_icon, media_play_icon, media_previous_icon, preview_close_icon,
    preview_pin_icon, preview_pinned_icon,
};
pub use settings::{
    general_about_icon, general_datetime_icon, general_language_icon, general_storage_icon,
    general_update_icon, settings_about_icon, settings_appearance_icon, settings_dock_icon,
    settings_general_icon, settings_grip_icon, settings_launchpad_icon, settings_menubar_icon,
    settings_shortcuts_icon, settings_taskbar_icon, storage_apps_icon, storage_documents_icon,
    storage_downloads_icon, storage_images_icon, storage_music_icon, storage_other_icon,
    storage_trash_icon, storage_videos_icon, storage_windows_icon,
};
pub use traffic::{
    traffic_circle, traffic_close_button, traffic_minimize_button, traffic_zoom_button,
};

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::{launchpad, menu, menubar, nav_pill};

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

/// Chevrón de las filas con submenú.
pub fn submenu_chevron_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::ChevronRight,
        menu::CHEVRON_GLYPH_SIZE,
        color,
    )
}

/// Marca de Dokka en la barra superior: icono Lucide a 20, sin dibujo propio.
pub fn menubar_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Dock, menubar::GLYPH_SIZE, color)
}

/// Marca de la opción elegida.
pub fn checkmark_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Check, menu::CHECK_GLYPH_SIZE, color)
}

/// Chevrón izquierdo de la píldora de navegación.
pub fn nav_previous_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::ChevronLeft, nav_pill::GLYPH_SIZE, color)
}

/// Chevrón derecho de la píldora de navegación.
pub fn nav_next_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::ChevronRight,
        nav_pill::GLYPH_SIZE,
        color,
    )
}

/// Lupa de la casilla de búsqueda del Launchpad.
pub fn search_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Search,
        launchpad::SEARCH_GLYPH_SIZE,
        color,
    )
}
