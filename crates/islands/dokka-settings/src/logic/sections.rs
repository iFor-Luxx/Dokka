use dokka_scene::{Color, DRAWING_SIDE, Drawing, Shape};
use dokka_theme::settings;

/// Nombres de las secciones en el orden de la barra lateral.
pub const SECTION_NAMES: [&str; 8] = [
    "General",
    "Dock",
    "Launchpad",
    "Barra superior",
    "Apariencia",
    "Barra de tareas",
    "Atajos",
    "Acerca de",
];

/// Índice de cada sección con contenido real.
pub const SECTION_DOCK: usize = 1;
pub const SECTION_MENUBAR: usize = 3;
pub const SECTION_TASKBAR: usize = 5;

type SectionIcon = fn(Color) -> Drawing;

/// Icono Lucide de cada sección, en su orden.
const SECTION_ICONS: [SectionIcon; 8] = [
    dokka_icons::settings_general_icon,
    dokka_icons::settings_dock_icon,
    dokka_icons::settings_launchpad_icon,
    dokka_icons::settings_menubar_icon,
    dokka_icons::settings_appearance_icon,
    dokka_icons::settings_taskbar_icon,
    dokka_icons::settings_shortcuts_icon,
    dokka_icons::settings_about_icon,
];

/// Nombres como `String` para las etiquetas.
pub(crate) fn section_names() -> Vec<String> {
    SECTION_NAMES.iter().map(|name| name.to_string()).collect()
}

/// Dibujo blanco del icono de la sección.
pub(crate) fn section_drawing(index: usize, color: Color) -> Drawing {
    SECTION_ICONS[index](color)
}

/// Cuadrado de color de la fila como imagen, para que al filtrar cada capa
/// repinte su color: el marcador de un icono no cambia al actualizarse.
pub(crate) fn badge_drawing(color: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::RoundedRect {
            x: 0.0,
            y: 0.0,
            width: DRAWING_SIDE,
            height: DRAWING_SIDE,
            radius: settings::BADGE_CANVAS_RADIUS,
            color,
        }],
    }
}
