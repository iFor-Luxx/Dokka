//! Tamaño de una carpeta sin bloquear ni seguir enlaces.
//!
//! Suma solo tamaños, sin leer contenido de archivos. No sigue enlaces
//! ni uniones (se saltan los puntos de reanálisis) e ignora lo que no se
//! puede leer. Nunca registra rutas ni nombres. El trabajo se reparte en
//! pasos para poder cancelarlo al salir del apartado.

use std::path::PathBuf;

use windows::Win32::Storage::FileSystem::{FILE_ATTRIBUTE_REPARSE_POINT, GetFileAttributesW};
use windows::core::HSTRING;

/// Cuántas entradas se visitan antes de mirar si se canceló.
const CANCEL_CHECK_EVERY: usize = 512;

/// Suma los tamaños de la carpeta; `None` si no se pudo ni abrir.
/// Devuelve lo sumado aunque se cancele a medias.
pub(crate) fn folder_size(root: &std::path::Path, should_stop: &dyn Fn() -> bool) -> Option<u64> {
    if is_link(root) {
        return Some(0);
    }
    let mut total: u64 = 0;
    let mut pending = vec![root.to_owned()];
    let mut visited: usize = 0;
    let mut opened = false;
    while let Some(dir) = pending.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(entries) => {
                opened = true;
                entries
            }
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            visited += 1;
            if visited.is_multiple_of(CANCEL_CHECK_EVERY) && should_stop() {
                return Some(total);
            }
            push_entry(&entry.path(), &mut pending, &mut total);
        }
    }
    opened.then_some(total)
}

/// Clasifica una entrada: suma el archivo o encola la carpeta.
fn push_entry(path: &PathBuf, pending: &mut Vec<PathBuf>, total: &mut u64) {
    if is_link(path) {
        return;
    }
    let kind = match std::fs::symlink_metadata(path) {
        Ok(kind) => kind,
        Err(_) => return,
    };
    if kind.is_dir() {
        pending.push(path.clone());
    } else if kind.is_file() {
        *total = total.saturating_add(kind.len());
    }
}

/// `true` si la ruta es un enlace o una unión: no se entra ni se suma.
fn is_link(path: &std::path::Path) -> bool {
    if std::fs::symlink_metadata(path)
        .map(|kind| kind.file_type().is_symlink())
        .unwrap_or(false)
    {
        return true;
    }
    let name = HSTRING::from(path.as_os_str());
    // SAFETY: la cadena vive durante la llamada; solo lee atributos.
    let attributes = unsafe { GetFileAttributesW(&name) };
    attributes != u32::MAX && attributes & FILE_ATTRIBUTE_REPARSE_POINT.0 != 0
}
