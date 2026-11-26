use std::collections::HashMap;
use std::ffi::c_void;
use std::path::{Path, PathBuf};

use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
};
use windows::core::HSTRING;

/// Nombres descriptivos ya consultados, como "Google Chrome".
#[derive(Debug, Default)]
pub struct NameCache {
    names: HashMap<PathBuf, Option<String>>,
}

impl NameCache {
    pub fn name_for(&mut self, executable: &Path) -> Option<String> {
        self.names
            .entry(executable.to_path_buf())
            .or_insert_with(|| file_description(executable))
            .clone()
    }
}

/// Descripción que el programa declara en sus datos de versión.
fn file_description(executable: &Path) -> Option<String> {
    let path = HSTRING::from(executable.as_os_str());
    // SAFETY: `path` vive durante la llamada.
    let length = unsafe { GetFileVersionInfoSizeW(&path, None) };
    if length == 0 {
        return None;
    }
    let mut block = vec![0_u8; length as usize];
    // SAFETY: `block` tiene exactamente el tamaño que se declara.
    unsafe { GetFileVersionInfoW(&path, None, length, block.as_mut_ptr().cast()) }.ok()?;

    let (translation, bytes) = query(&block, "\\VarFileInfo\\Translation")?;
    if bytes < 4 {
        return None;
    }
    // SAFETY: hay al menos dos enteros de 16 bits: idioma y página de códigos.
    let (language, code_page) = unsafe {
        let pair = translation.cast::<u16>();
        (pair.read_unaligned(), pair.add(1).read_unaligned())
    };

    let key = format!("\\StringFileInfo\\{language:04x}{code_page:04x}\\FileDescription");
    let (value, characters) = query(&block, &key)?;
    // SAFETY: Windows indica cuántos caracteres de 16 bits hay en `value`.
    let text = unsafe { std::slice::from_raw_parts(value.cast::<u16>(), characters as usize) };
    let name = String::from_utf16_lossy(text);
    let name = name.trim_end_matches('\0').trim();
    (!name.is_empty()).then(|| name.to_owned())
}

/// Puntero a un valor dentro de `block` y su longitud.
fn query(block: &[u8], key: &str) -> Option<(*const c_void, u32)> {
    let mut value = std::ptr::null_mut();
    let mut length = 0;
    // SAFETY: `block` viene de `GetFileVersionInfoW`; los punteros de salida son válidos.
    let found = unsafe {
        VerQueryValueW(
            block.as_ptr().cast(),
            &HSTRING::from(key),
            &mut value,
            &mut length,
        )
    };
    (found.as_bool() && length > 0).then_some((value.cast_const(), length))
}
