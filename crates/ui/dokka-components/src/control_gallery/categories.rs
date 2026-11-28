//! Categorías de la galería y nombres visibles en español.
//!
//! Puro y sin estado: la isla guarda la elegida y lo escrito; aquí solo se
//! dice qué controles lleva cada categoría y si un nombre coincide con la
//! búsqueda (sin distinguir mayúsculas ni tildes, como el buscador).

use dokka_model::ControlItem;

/// Categorías en el orden de la columna: primero todas.
pub const CATEGORY_COUNT: usize = 10;

/// Nombre visible de cada categoría.
pub fn category_name(index: usize) -> &'static str {
    match index {
        1 => "Ahora suena",
        2 => "Captura de pantalla",
        3 => "Conectividad",
        4 => "Escritorio",
        5 => "Pantalla bloqueada",
        6 => "Pantalla y brillo",
        7 => "Reloj",
        8 => "Sonidos",
        9 => "Utilidades",
        _ => "Todos los controles",
    }
}

/// Controles de cada categoría; Pantalla solo con brillo DDC.
pub fn category_items(index: usize, with_brightness: bool) -> Vec<ControlItem> {
    let mut items = match index {
        1 => vec![ControlItem::Music],
        2 => vec![ControlItem::ScreenClip],
        3 => vec![ControlItem::Network],
        4 => vec![ControlItem::Desktop, ControlItem::TaskView],
        5 => vec![ControlItem::Lock],
        6 => vec![ControlItem::Project, ControlItem::Brightness],
        7 => vec![ControlItem::Clock],
        8 => vec![ControlItem::Sound],
        9 => vec![
            ControlItem::Calculator,
            ControlItem::TaskManager,
            ControlItem::Settings,
        ],
        _ => vec![
            ControlItem::Music,
            ControlItem::ScreenClip,
            ControlItem::Network,
            ControlItem::Desktop,
            ControlItem::TaskView,
            ControlItem::Lock,
            ControlItem::Brightness,
            ControlItem::Project,
            ControlItem::Clock,
            ControlItem::Sound,
            ControlItem::Calculator,
            ControlItem::TaskManager,
            ControlItem::Settings,
        ],
    };
    if !with_brightness {
        items.retain(|item| *item != ControlItem::Brightness);
    }
    items
}

/// Categoría a la que pertenece cada control.
pub fn item_category(item: ControlItem) -> usize {
    match item {
        ControlItem::Music => 1,
        ControlItem::ScreenClip => 2,
        ControlItem::Network => 3,
        ControlItem::Desktop | ControlItem::TaskView => 4,
        ControlItem::Lock => 5,
        ControlItem::Brightness | ControlItem::Project => 6,
        ControlItem::Clock => 7,
        ControlItem::Sound => 8,
        _ => 9,
    }
}

/// Nombre visible bajo cada control.
pub fn item_name(item: ControlItem) -> &'static str {
    match item {
        ControlItem::Network => "Red",
        ControlItem::Music => "Ahora suena",
        ControlItem::Desktop => "Escritorio",
        ControlItem::Lock => "Bloquear",
        ControlItem::Settings => "Preferencias",
        ControlItem::TaskView => "Vista de tareas",
        ControlItem::Project => "Proyectar",
        ControlItem::Brightness => "Brillo",
        ControlItem::Sound => "Sonido",
        ControlItem::Calculator => "Calculadora",
        ControlItem::ScreenClip => "Recorte",
        ControlItem::Clock => "Reloj",
        ControlItem::TaskManager => "Administrador de tareas",
    }
}

/// Controles a mostrar: con búsqueda se filtra por nombre en todas las
/// categorías; sin ella, solo la elegida (todas si es la primera).
pub fn visible_items(
    selected: usize,
    query: &str,
    with_brightness: bool,
) -> Vec<(usize, ControlItem)> {
    let wanted = normalized(query);
    if !wanted.is_empty() {
        return category_items(0, with_brightness)
            .into_iter()
            .filter(|item| normalized(item_name(*item)).contains(&wanted))
            .map(|item| (item_category(item), item))
            .collect();
    }
    category_items(selected.min(CATEGORY_COUNT - 1), with_brightness)
        .into_iter()
        .map(|item| {
            let category = if selected == 0 {
                item_category(item)
            } else {
                selected
            };
            (category, item)
        })
        .collect()
}

/// Minúsculas sin tildes para comparar, como el buscador de la barra.
fn normalized(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|letter| match letter {
            'á' | 'à' | 'ä' | 'â' | 'ã' | 'å' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' | 'õ' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ý' | 'ÿ' => 'y',
            'ç' => 'c',
            'ñ' => 'n',
            rest => rest,
        })
        .collect()
}
