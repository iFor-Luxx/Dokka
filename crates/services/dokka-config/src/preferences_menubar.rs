//! Parte de la barra superior en `preferences.txt`: qué iconos se ven y en
//! qué orden (`menubar.order=volume,network,...`).
//!
//! Claves `menubar.*`; lo desconocido o mal escrito se ignora y queda de
//! fábrica. El orden incompleto, repetido o mal escrito conserva los válidos
//! en su orden y añade los que falten en el orden de fábrica.

use dokka_model::{MenubarItem, Preferences};

use super::preferences_file::{format_bool, parse_bool};

/// Nombre guardado de cada icono, en minúsculas y sin espacios.
pub(crate) fn item_name(item: MenubarItem) -> &'static str {
    match item {
        MenubarItem::Volume => "volume",
        MenubarItem::Network => "network",
        MenubarItem::Music => "music",
        MenubarItem::BackgroundApps => "background_apps",
        MenubarItem::Language => "language",
        MenubarItem::Search => "search",
        MenubarItem::ControlCenter => "control_center",
    }
}

/// Icono con este nombre guardado, si está bien escrito.
fn item_from_name(name: &str) -> Option<MenubarItem> {
    match name {
        "volume" => Some(MenubarItem::Volume),
        "network" => Some(MenubarItem::Network),
        "music" => Some(MenubarItem::Music),
        "background_apps" => Some(MenubarItem::BackgroundApps),
        "language" => Some(MenubarItem::Language),
        "search" => Some(MenubarItem::Search),
        "control_center" => Some(MenubarItem::ControlCenter),
        _ => None,
    }
}

/// Orden leído de `menubar.order`: los válidos en su orden y los que falten
/// en el orden de fábrica. Lo repetido vale una vez; lo mal escrito se
/// ignora y nunca falla.
pub(crate) fn parse_order(value: &str) -> [MenubarItem; 7] {
    let mut kept: Vec<MenubarItem> = Vec::with_capacity(7);
    for part in value.split(',') {
        let Some(item) = item_from_name(part.trim()) else {
            continue;
        };
        if !kept.contains(&item) {
            kept.push(item);
        }
    }
    for item in MenubarItem::FACTORY_ORDER {
        if !kept.contains(&item) {
            kept.push(item);
        }
    }
    let mut order = MenubarItem::FACTORY_ORDER;
    for (slot, item) in kept.into_iter().take(7).enumerate() {
        order[slot] = item;
    }
    order
}

/// Orden para escribir: siete nombres separados por comas.
fn format_order(order: &[MenubarItem; 7]) -> String {
    order
        .iter()
        .copied()
        .map(item_name)
        .collect::<Vec<_>>()
        .join(",")
}

/// Aplica una clave de la barra; `true` si era de la barra.
pub(crate) fn apply_menubar(preferences: &mut Preferences, key: &str, value: &str) -> bool {
    match key {
        "menubar.volume" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.volume = parsed;
            }
            true
        }
        "menubar.network" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.network = parsed;
            }
            true
        }
        "menubar.music" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.music = parsed;
            }
            true
        }
        "menubar.language" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.language = parsed;
            }
            true
        }
        "menubar.search" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.search = parsed;
            }
            true
        }
        "menubar.control_center" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.control_center = parsed;
            }
            true
        }
        "menubar.background_apps" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.background_apps = parsed;
            }
            true
        }
        "menubar.order" => {
            preferences.menubar.order = parse_order(value);
            true
        }
        _ => false,
    }
}

/// Líneas de la barra en orden estable para escribir el archivo.
pub(crate) fn render_menubar(preferences: &Preferences) -> Vec<(&'static str, String)> {
    vec![
        ("menubar.volume", format_bool(preferences.menubar.volume)),
        ("menubar.network", format_bool(preferences.menubar.network)),
        ("menubar.music", format_bool(preferences.menubar.music)),
        (
            "menubar.language",
            format_bool(preferences.menubar.language),
        ),
        ("menubar.search", format_bool(preferences.menubar.search)),
        (
            "menubar.control_center",
            format_bool(preferences.menubar.control_center),
        ),
        (
            "menubar.background_apps",
            format_bool(preferences.menubar.background_apps),
        ),
        ("menubar.order", format_order(&preferences.menubar.order)),
    ]
}
