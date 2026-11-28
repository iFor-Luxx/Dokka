//! Abrir, leer y escribir valores del registro para el instalador.
//!
//! Solo estas operaciones; cada clave se cierra con `OpenKey` al salir.

use dokka_contracts::InstallFailure;
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    HKEY, KEY_READ, REG_DWORD, REG_SAM_FLAGS, REG_SZ, RegCloseKey, RegCreateKeyW, RegDeleteTreeW,
    RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
};
use windows::core::HSTRING;

use crate::install_error::from_win32;

/// Clave abierta que se cierra sola al salir.
pub(crate) struct OpenKey(HKEY);

impl OpenKey {
    /// Crea la clave (o abre la que haya) para escribir.
    pub(crate) fn create(root: HKEY, subkey: &str) -> Result<Self, InstallFailure> {
        let subkey = HSTRING::from(subkey);
        let mut key = HKEY(std::ptr::null_mut());
        // SAFETY: la subclave vive en la llamada y `key` se cierra en `Drop`.
        let created = unsafe { RegCreateKeyW(root, &subkey, &mut key) };
        if created != ERROR_SUCCESS {
            return Err(from_win32(created));
        }
        Ok(Self(key))
    }

    /// Abre una clave existente con el acceso pedido.
    pub(crate) fn open(
        root: HKEY,
        subkey: &str,
        access: REG_SAM_FLAGS,
    ) -> Result<Self, InstallFailure> {
        let subkey = HSTRING::from(subkey);
        let mut key = HKEY(std::ptr::null_mut());
        // SAFETY: la subclave vive en la llamada y `key` se cierra en `Drop`.
        let opened = unsafe { RegOpenKeyExW(root, &subkey, None, access, &mut key) };
        if opened != ERROR_SUCCESS {
            return Err(from_win32(opened));
        }
        Ok(Self(key))
    }

    pub(crate) fn handle(&self) -> HKEY {
        self.0
    }
}

impl Drop for OpenKey {
    fn drop(&mut self) {
        // SAFETY: la clave se abrió al crear y ya no se usa.
        unsafe {
            let _ = RegCloseKey(self.0);
        }
    }
}

/// Lee una cadena `REG_SZ`. `None` si no existe o no es texto.
pub(crate) fn read_string_value(root: HKEY, subkey: &str, name: &str) -> Option<String> {
    let key = OpenKey::open(root, subkey, KEY_READ).ok()?;
    let name = HSTRING::from(name);
    let mut kind = windows::Win32::System::Registry::REG_VALUE_TYPE(0);
    let mut size: u32 = 0;
    // SAFETY: primera llamada solo para saber el tamaño, sin buffer.
    let sized = unsafe {
        RegQueryValueExW(
            key.handle(),
            &name,
            None,
            Some(&mut kind),
            None,
            Some(&mut size),
        )
    };
    if sized != ERROR_SUCCESS || size == 0 || size > 8192 {
        return None;
    }
    let count = size as usize / 2;
    let mut buffer = vec![0_u16; count];
    let mut actual = size;
    // SAFETY: el buffer reserva `actual` bytes y se relee su tamaño.
    let read = unsafe {
        RegQueryValueExW(
            key.handle(),
            &name,
            None,
            Some(&mut kind),
            Some(buffer.as_mut_ptr() as *mut u8),
            Some(&mut actual),
        )
    };
    if read != ERROR_SUCCESS || kind != REG_SZ {
        return None;
    }
    let used = (actual as usize / 2).min(buffer.len());
    let slice = &buffer[..used];
    let end = slice
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(slice.len());
    let text = String::from_utf16(&slice[..end]).ok()?;
    (!text.is_empty()).then_some(text)
}

/// Lee un `REG_DWORD`. `None` si no existe.
pub(crate) fn read_dword_value(root: HKEY, subkey: &str, name: &str) -> Option<u32> {
    let key = OpenKey::open(root, subkey, KEY_READ).ok()?;
    let name = HSTRING::from(name);
    let mut kind = windows::Win32::System::Registry::REG_VALUE_TYPE(0);
    let mut data: u32 = 0;
    let mut size = size_of::<u32>() as u32;
    // SAFETY: `data` es un `u32` válido y `size` dice su tamaño.
    let read = unsafe {
        RegQueryValueExW(
            key.handle(),
            &name,
            None,
            Some(&mut kind),
            Some((&mut data as *mut u32) as *mut u8),
            Some(&mut size),
        )
    };
    if read != ERROR_SUCCESS || kind != REG_DWORD {
        return None;
    }
    Some(data)
}

/// Escribe una cadena `REG_SZ` con su nulo final.
pub(crate) fn write_string_value(key: HKEY, name: &str, data: &str) -> Result<(), InstallFailure> {
    let name = HSTRING::from(name);
    let wide: Vec<u16> = data.encode_utf16().chain([0]).collect();
    // SAFETY: son los bytes UTF-16 con nulo de `data`.
    let bytes = unsafe {
        std::slice::from_raw_parts(wide.as_ptr().cast::<u8>(), std::mem::size_of_val(&wide[..]))
    };
    // SAFETY: `bytes` vive durante la llamada y solo se lee.
    let written = unsafe { RegSetValueExW(key, &name, None, REG_SZ, Some(bytes)) };
    if written != ERROR_SUCCESS {
        return Err(from_win32(written));
    }
    Ok(())
}

/// Escribe un `REG_DWORD`.
pub(crate) fn write_dword_value(key: HKEY, name: &str, data: u32) -> Result<(), InstallFailure> {
    let name = HSTRING::from(name);
    let bytes = data.to_le_bytes();
    // SAFETY: `bytes` son los 4 bytes del `u32` y solo se leen.
    let written = unsafe { RegSetValueExW(key, &name, None, REG_DWORD, Some(&bytes)) };
    if written != ERROR_SUCCESS {
        return Err(from_win32(written));
    }
    Ok(())
}

/// Borra un valor de una clave abierta. Si no existe, ya está.
pub(crate) fn delete_value(key: HKEY, name: &str) -> Result<(), InstallFailure> {
    let name = HSTRING::from(name);
    // SAFETY: `name` vive durante la llamada y la clave sigue abierta.
    let deleted = unsafe { RegDeleteValueW(key, &name) };
    if deleted == ERROR_SUCCESS || deleted == ERROR_FILE_NOT_FOUND {
        Ok(())
    } else {
        Err(from_win32(deleted))
    }
}

/// Borra una clave con todo lo que contenga. Si no existe, ya está.
pub(crate) fn delete_tree(root: HKEY, subkey: &str) -> Result<(), InstallFailure> {
    let subkey = HSTRING::from(subkey);
    // SAFETY: `subkey` vive durante la llamada.
    let deleted = unsafe { RegDeleteTreeW(root, &subkey) };
    if deleted == ERROR_SUCCESS || deleted == ERROR_FILE_NOT_FOUND {
        Ok(())
    } else {
        Err(from_win32(deleted))
    }
}
