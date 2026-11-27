//! Parte del dock en `preferences.txt`: tamaño, lupa, posición y hábitos.
//!
//! Claves `dock.*`; lo desconocido o mal escrito se ignora y queda de
//! fábrica, los números se limitan a su rango.

use dokka_model::{DOCK_ICON_SCALE_RANGE, DOCK_MAGNIFICATION_RANGE, DockPosition, Preferences};

use super::preferences_file::{format_bool, format_float, parse_bool, parse_float};

/// Aplica una clave del dock; `true` si era del dock (aunque el valor falle).
pub(crate) fn apply_dock(preferences: &mut Preferences, key: &str, value: &str) -> bool {
    match key {
        "dock.icon_scale" => {
            if let Some(parsed) = parse_float(value) {
                preferences.dock.icon_scale = clamp(parsed, DOCK_ICON_SCALE_RANGE);
            }
            true
        }
        "dock.magnification" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.dock.magnification = parsed;
            }
            true
        }
        "dock.magnification_strength" => {
            if let Some(parsed) = parse_float(value) {
                preferences.dock.magnification_strength = clamp(parsed, DOCK_MAGNIFICATION_RANGE);
            }
            true
        }
        "dock.position" => {
            if let Some(parsed) = parse_position(value) {
                preferences.dock.position = parsed;
            }
            true
        }
        "dock.auto_hide" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.dock.auto_hide = parsed;
            }
            true
        }
        "dock.animate_launch" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.dock.animate_launch = parsed;
            }
            true
        }
        "dock.show_indicators" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.dock.show_indicators = parsed;
            }
            true
        }
        "dock.previews" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.dock.previews = parsed;
            }
            true
        }
        _ => false,
    }
}

/// Líneas del dock en orden estable para escribir el archivo.
pub(crate) fn render_dock(preferences: &Preferences) -> Vec<(&'static str, String)> {
    vec![
        (
            "dock.icon_scale",
            format_float(preferences.dock.icon_scale),
        ),
        (
            "dock.magnification",
            format_bool(preferences.dock.magnification),
        ),
        (
            "dock.magnification_strength",
            format_float(preferences.dock.magnification_strength),
        ),
        (
            "dock.position",
            format_position(preferences.dock.position),
        ),
        (
            "dock.auto_hide",
            format_bool(preferences.dock.auto_hide),
        ),
        (
            "dock.animate_launch",
            format_bool(preferences.dock.animate_launch),
        ),
        (
            "dock.show_indicators",
            format_bool(preferences.dock.show_indicators),
        ),
        ("dock.previews", format_bool(preferences.dock.previews)),
    ]
}

fn parse_position(value: &str) -> Option<DockPosition> {
    match value {
        "left" => Some(DockPosition::Left),
        "bottom" => Some(DockPosition::Bottom),
        "right" => Some(DockPosition::Right),
        _ => None,
    }
}

fn format_position(position: DockPosition) -> String {
    match position {
        DockPosition::Left => "left".to_owned(),
        DockPosition::Bottom => "bottom".to_owned(),
        DockPosition::Right => "right".to_owned(),
    }
}

fn clamp(value: f32, range: (f32, f32)) -> f32 {
    value.clamp(range.0, range.1)
}
