//! Parte general en `preferences.txt`: formato de hora y primer día de semana.
//!
//! Claves `general.*`; lo desconocido o mal escrito se ignora y queda de
//! fábrica. Son ajustes propios de Dokka: lo que es de Windows se muestra
//! como texto y se cambia con un botón que abre su página.

use dokka_model::Preferences;

use super::preferences_file::{format_bool, parse_bool};

/// Aplica una clave general; `true` si era general (aunque el valor falle).
pub(crate) fn apply_general(preferences: &mut Preferences, key: &str, value: &str) -> bool {
    match key {
        "general.hour_24" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.general.hour_24 = parsed;
            }
            true
        }
        "general.week_starts_monday" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.general.week_starts_monday = parsed;
            }
            true
        }
        _ => false,
    }
}

/// Líneas generales en orden estable para escribir el archivo.
pub(crate) fn render_general(preferences: &Preferences) -> Vec<(&'static str, String)> {
    vec![
        ("general.hour_24", format_bool(preferences.general.hour_24)),
        (
            "general.week_starts_monday",
            format_bool(preferences.general.week_starts_monday),
        ),
    ]
}
