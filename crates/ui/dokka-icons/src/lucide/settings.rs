//! Iconos blancos de las secciones de Preferencias y de la lista General.

use dokka_scene::{Color, Drawing};
use dokka_theme::settings;

use super::glyph;

/// Icono blanco de la sección General sobre su cuadrado de color.
pub fn settings_general_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Settings,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Dock sobre su cuadrado de color.
pub fn settings_dock_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Dock, settings::ROW_GLYPH_SIZE, color)
}

/// Icono blanco de la sección Launchpad sobre su cuadrado de color.
pub fn settings_launchpad_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::LayoutGrid,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Barra superior sobre su cuadrado de color.
pub fn settings_menubar_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::PanelTop,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Apariencia sobre su cuadrado de color.
pub fn settings_appearance_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Palette, settings::ROW_GLYPH_SIZE, color)
}

/// Icono blanco de la sección Barra de tareas sobre su cuadrado de color.
pub fn settings_taskbar_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::PanelBottom,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Atajos sobre su cuadrado de color.
pub fn settings_shortcuts_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Keyboard,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Acerca de sobre su cuadrado de color.
pub fn settings_about_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Info, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de "Acerca de este PC": monitor propio de la ficha.
pub fn general_about_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Monitor, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de "Actualización de Dokka": flecha circular.
pub fn general_update_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::RefreshCw,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono de "Almacenamiento": disco duro.
pub fn general_storage_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::HardDrive,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono de "Fecha y hora": reloj.
pub fn general_datetime_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Clock, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de "Idioma y región": idiomas.
pub fn general_language_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Languages,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Agarre de las filas reordenables de la barra superior: seis puntos
/// verticales en color secundario, a tamaño de la fila.
pub fn settings_grip_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::GripVertical,
        settings::REORDER_GRIP_GLYPH_SIZE,
        color,
    )
}

/// Icono de la categoría Aplicaciones del almacenamiento: cuadrícula.
pub fn storage_apps_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::LayoutGrid,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono de la categoría Documentos del almacenamiento: texto.
pub fn storage_documents_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::FileText,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono de la categoría Imágenes del almacenamiento: imagen.
pub fn storage_images_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Image, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de la categoría Música del almacenamiento: nota.
pub fn storage_music_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Music, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de la categoría Videos del almacenamiento: película.
pub fn storage_videos_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Film, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de la categoría Descargas del almacenamiento: descarga.
pub fn storage_downloads_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Download,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono de la categoría Papelera del almacenamiento: papelera.
pub fn storage_trash_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Trash2, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de la categoría Windows del almacenamiento: carpeta.
pub fn storage_windows_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Folder, settings::ROW_GLYPH_SIZE, color)
}

/// Icono de "Datos del sistema y otros": base de datos.
pub fn storage_other_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Database,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}
