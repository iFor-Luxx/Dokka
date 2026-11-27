//! Preferencias del usuario en un archivo de texto, una por línea.
//!
//! Formato `clave=valor`, sin dependencias: las claves desconocidas o los
//! valores mal escritos se ignoran y queda el valor de fábrica; los números
//! se limitan a su rango. La escritura es atómica (borrador y renombre).
//! El detalle del dock y de la barra vive en sus módulos.

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use dokka_contracts::{ContractError, PreferencesStore};
use dokka_model::Preferences;

use super::preferences_dock::{apply_dock, render_dock};
use super::preferences_menubar::{apply_menubar, render_menubar};

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
    // Una clave es de una sola parte: si no es del dock, se prueba la barra.
    if !apply_dock(preferences, key, value) {
        apply_menubar(preferences, key, value);
    }
}

/// Escribe las claves conocidas, una por línea.
fn render(preferences: &Preferences) -> String {
    let mut text = String::new();
    for (key, value) in render_dock(preferences)
        .into_iter()
        .chain(render_menubar(preferences))
    {
        push(&mut text, key, &value);
    }
    text
}

fn push(text: &mut String, key: &str, value: &str) {
    text.push_str(key);
    text.push('=');
    text.push_str(value);
    text.push('\n');
}

pub(crate) fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

pub(crate) fn parse_float(value: &str) -> Option<f32> {
    value
        .parse::<f32>()
        .ok()
        .filter(|parsed| parsed.is_finite())
}

pub(crate) fn format_bool(value: bool) -> String {
    if value {
        "true".to_owned()
    } else {
        "false".to_owned()
    }
}

pub(crate) fn format_float(value: f32) -> String {
    format!("{value:.4}")
}

fn failed(error: &std::io::Error) -> ContractError {
    ContractError::Failed(error.to_string())
}
