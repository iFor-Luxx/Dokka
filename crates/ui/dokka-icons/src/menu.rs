//! Iconos de las filas de menú desde Lucide, por significado.
//!
//! Cada función crea un `Drawing` con un solo glifo que llena el lienzo
//! de 100 (centrado en 50, 50). El color lo pone quien llama
//! (texto normal de la fila; al resaltar se tiñe en el panel).

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::menu as measures;

const CENTER: f32 = 50.0;

fn glyph(icon: lucide_icons::Icon, color: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Glyph {
            glyph: char::from(icon),
            x: CENTER,
            y: CENTER,
            size: measures::ROW_GLYPH_SIZE,
            color,
        }],
    }
}

/// Abrir la app o el elemento.
pub fn menu_open_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::DoorOpen, color)
}

/// Abrir una ventana nueva del explorador.
pub fn menu_new_window_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::AppWindow, color)
}

/// Abrir el Launchpad.
pub fn menu_launchpad_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::LayoutGrid, color)
}

/// Añadir una app al dock.
pub fn menu_add_app_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Plus, color)
}

/// Añadir un acceso del sistema.
pub fn menu_add_system_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::MonitorCog, color)
}

/// Añadir un separador al dock.
pub fn menu_add_separator_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::SeparatorHorizontal, color)
}

/// Quitar el separador del dock.
pub fn menu_remove_separator_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Minus, color)
}

/// Mostrar la barra de tareas ahora.
pub fn menu_show_taskbar_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Eye, color)
}

/// Ocultar la barra de tareas ahora.
pub fn menu_hide_taskbar_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::EyeOff, color)
}

/// Ejecutar con permisos de administrador.
pub fn menu_admin_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::ShieldCheck, color)
}

/// Mostrar el archivo en su carpeta.
pub fn menu_reveal_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::FolderOpen, color)
}

/// Fijar la app en el dock.
pub fn menu_pin_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Pin, color)
}

/// Quitar la app del dock.
pub fn menu_unpin_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::PinOff, color)
}

/// Cerrar la ventana al frente.
pub fn menu_close_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::X, color)
}

/// Salir de la app.
pub fn menu_quit_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::LogOut, color)
}

/// Abrir la carpeta de descargas.
pub fn menu_downloads_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Download, color)
}

/// Abrir la carpeta de documentos.
pub fn menu_documents_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::FileText, color)
}

/// Mostrar el escritorio.
pub fn menu_desktop_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::House, color)
}

/// Abrir la carpeta de imágenes.
pub fn menu_pictures_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Image, color)
}

/// Abrir la papelera.
pub fn menu_trash_open_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::FolderOpen, color)
}

/// Vaciar la papelera.
pub fn menu_empty_trash_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Trash2, color)
}

/// Abrir Preferencias.
pub fn menu_preferences_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Settings, color)
}

/// Reiniciar Dokka.
pub fn menu_restart_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::RotateCcw, color)
}

/// Salir de Dokka o apagar.
pub fn menu_power_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Power, color)
}

/// Abrir el administrador de tareas.
pub fn menu_task_manager_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Activity, color)
}

/// Abrir los ajustes de la barra de tareas.
pub fn menu_taskbar_settings_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::MonitorCog, color)
}

/// Bloquear la pantalla.
pub fn menu_lock_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Lock, color)
}
