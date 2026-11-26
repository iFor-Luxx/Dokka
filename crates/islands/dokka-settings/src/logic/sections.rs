use dokka_scene::{Color, Drawing};

/// Nombres de las secciones en el orden de la barra lateral.
pub const SECTION_NAMES: [&str; 7] = [
    "General",
    "Dock",
    "Launchpad",
    "Apariencia",
    "Barra de tareas",
    "Atajos",
    "Acerca de",
];

type SectionIcon = fn(Color) -> Drawing;

/// Icono Lucide de cada sección, en su orden.
const SECTION_ICONS: [SectionIcon; 7] = [
    dokka_icons::settings_general_icon,
    dokka_icons::settings_dock_icon,
    dokka_icons::settings_launchpad_icon,
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
