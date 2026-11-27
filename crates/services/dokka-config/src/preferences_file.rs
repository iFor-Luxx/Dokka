//! Preferencias del usuario en un archivo de texto, una por línea.
//!
//! Formato `clave=valor`, sin dependencias: las claves desconocidas o los
//! valores mal escritos se ignoran y queda el valor de fábrica; los números
//! se limitan a su rango. La escritura es atómica (borrador y renombre).

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use dokka_contracts::{ContractError, PreferencesStore};
use dokka_model::{DOCK_ICON_SCALE_RANGE, DOCK_MAGNIFICATION_RANGE, Preferences};

/// Preferencias guardadas en `%APPDATA%\Dokka\preferences.txt`.
#[derive(Debug)]
pub struct PreferencesFile {
    path: PathBuf,
}

impl PreferencesFile {
    /// Usa la carpeta de datos del usuario.
    pub fn in_user_folder() -> Result<Self, ContractError> {
        let folder = std::env::var_os("APPDATA").ok_or(ContractError::NotFound)?;
        let path = PathBuf::from(folder).join("Dokka").join("preferences.txt");
        Ok(Self { path })
    }
}

impl PreferencesStore for PreferencesFile {
    fn load(&self) -> Result<Preferences, ContractError> {
        let text = match fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return Ok(Preferences::default());
            }
            Err(error) => return Err(failed(&error)),
        };
        Ok(parse(&text))
    }

    fn save(&self, preferences: &Preferences) -> Result<(), ContractError> {
        if let Some(folder) = self.path.parent() {
            fs::create_dir_all(folder).map_err(|error| failed(&error))?;
        }
        let text = render(preferences);
        // Se escribe aparte y se renombra para no dejar un archivo a medias.
        let draft = self.path.with_extension("tmp");
        fs::write(&draft, text).map_err(|error| failed(&error))?;
        fs::rename(&draft, &self.path).map_err(|error| failed(&error))
    }
}

/// Lee las claves conocidas; lo demás se ignora y queda de fábrica.
fn parse(text: &str) -> Preferences {
    let mut preferences = Preferences::default();
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        apply(&mut preferences, key.trim(), value.trim());
    }
    preferences
}

fn apply(preferences: &mut Preferences, key: &str, value: &str) {
    match key {
        "dock.icon_scale" => {
            if let Some(parsed) = parse_float(value) {
                preferences.dock.icon_scale = clamp(parsed, DOCK_ICON_SCALE_RANGE);
            }
        }
        "dock.magnification" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.dock.magnification = parsed;
            }
        }
        "dock.magnification_strength" => {
            if let Some(parsed) = parse_float(value) {
                preferences.dock.magnification_strength = clamp(parsed, DOCK_MAGNIFICATION_RANGE);
            }
        }
        "dock.previews" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.dock.previews = parsed;
            }
        }
        "menubar.volume" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.volume = parsed;
            }
        }
        "menubar.network" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.network = parsed;
            }
        }
        "menubar.music" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.music = parsed;
            }
        }
        "menubar.language" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.language = parsed;
            }
        }
        "menubar.search" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.search = parsed;
            }
        }
        "menubar.control_center" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.control_center = parsed;
            }
        }
        "menubar.background_apps" => {
            if let Some(parsed) = parse_bool(value) {
                preferences.menubar.background_apps = parsed;
            }
        }
        _ => {}
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn parse_float(value: &str) -> Option<f32> {
    value.parse::<f32>().ok().filter(|parsed| parsed.is_finite())
}

fn clamp(value: f32, range: (f32, f32)) -> f32 {
    value.clamp(range.0, range.1)
}

/// Escribe las claves conocidas, una por línea.
fn render(preferences: &Preferences) -> String {
    let mut text = String::new();
    push(&mut text, "dock.icon_scale", &format_float(preferences.dock.icon_scale));
    push(&mut text, "dock.magnification", &format_bool(preferences.dock.magnification));
    push(
        &mut text,
        "dock.magnification_strength",
        &format_float(preferences.dock.magnification_strength),
    );
    push(&mut text, "dock.previews", &format_bool(preferences.dock.previews));
    push(&mut text, "menubar.volume", &format_bool(preferences.menubar.volume));
    push(
        &mut text,
        "menubar.network",
        &format_bool(preferences.menubar.network),
    );
    push(&mut text, "menubar.music", &format_bool(preferences.menubar.music));
    push(
        &mut text,
        "menubar.language",
        &format_bool(preferences.menubar.language),
    );
    push(&mut text, "menubar.search", &format_bool(preferences.menubar.search));
    push(
        &mut text,
        "menubar.control_center",
        &format_bool(preferences.menubar.control_center),
    );
    push(
        &mut text,
        "menubar.background_apps",
        &format_bool(preferences.menubar.background_apps),
    );
    text
}

fn push(text: &mut String, key: &str, value: &str) {
    text.push_str(key);
    text.push('=');
    text.push_str(value);
    text.push('\n');
}

fn format_bool(value: bool) -> String {
    if value { "true".to_owned() } else { "false".to_owned() }
}

fn format_float(value: f32) -> String {
    format!("{value:.4}")
}

fn failed(error: &std::io::Error) -> ContractError {
    ContractError::Failed(error.to_string())
}
