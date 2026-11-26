//! Icono declarado por un `.url`: su `IconFile`/`IconIndex`.
//!
//! Los accesos de juegos apuntan a un protocolo (`steam://`, ...) y Windows
//! solo les da una imagen pequeña sobre fondo blanco. Cuando el `.url`
//! declara dónde está el icono de verdad, se pide ese archivo en grande.

use std::path::{Path, PathBuf};

/// Archivo e índice del icono declarado en el `.url`, si existe.
pub(crate) fn icon_file_for(link: &Path) -> Option<(PathBuf, i32)> {
    if !is_url(link) {
        return None;
    }
    let text = std::fs::read_to_string(link).ok()?;
    let mut file: Option<String> = None;
    let mut index = 0;
    for line in text.lines() {
        let Some((key, value)) = split_setting(line) else {
            continue;
        };
        if key == "iconfile" {
            let cleaned = value
                .trim()
                .trim_matches(|side| side == '"' || side == '\'')
                .trim();
            if !cleaned.is_empty() {
                file = Some(expand_env(cleaned));
            }
        } else if key == "iconindex"
            && let Some(parsed) = value.trim().parse::<i32>().ok()
        {
            index = parsed;
        }
    }
    let file = file?;
    if file.trim().is_empty() {
        return None;
    }
    let candidate = PathBuf::from(&file);
    let path = if candidate.is_absolute() {
        candidate
    } else {
        link.parent().map(|dir| dir.join(candidate))?
    };
    Some((path, index))
}

fn is_url(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase() == "url")
        .unwrap_or(false)
}

/// `clave=valor` de un `.url`, con la clave en minúsculas.
fn split_setting(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with(';') || line.starts_with('#') || line.starts_with('[') {
        return None;
    }
    let (raw_key, raw_value) = line.split_once('=')?;
    let key = raw_key.trim().to_lowercase();
    let value = raw_value.trim().to_owned();
    (!key.is_empty()).then_some((key, value))
}

/// Expande `%VARIABLE%` con el entorno, sin fallar si falta alguna.
fn expand_env(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('%') {
        let (head, tail) = rest.split_at(start);
        out.push_str(head);
        let after = tail.get(1..).unwrap_or("");
        let Some(end) = after.find('%') else {
            out.push('%');
            out.push_str(after);
            break;
        };
        let name = after.get(..end).unwrap_or("");
        let value = std::env::var(name).unwrap_or_default();
        if name.is_empty() {
            out.push_str("%%");
        } else {
            out.push_str(&value);
        }
        rest = after.get(end + 1..).unwrap_or("");
    }
    if !rest.is_empty() {
        out.push_str(rest);
    }
    out
}
