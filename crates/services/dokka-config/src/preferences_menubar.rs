//! Parte de la barra superior en `preferences.txt`: qué iconos se ven.
//!
//! Claves `menubar.*`; lo desconocido o mal escrito se ignora y queda de
//! fábrica.

use dokka_model::Preferences;

use super::preferences_file::{format_bool, parse_bool};

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
    ]
}
