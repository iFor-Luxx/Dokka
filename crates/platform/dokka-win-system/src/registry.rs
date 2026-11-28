//! Lectura de cadenas del registro de Windows.
//!
//! Solo lee; nunca escribe.

use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_VALUE_TYPE, RegCloseKey, RegOpenKeyExW,
    RegQueryValueExW,
};
use windows::core::HSTRING;

/// Lee una cadena `REG_SZ` del registro local. `None` si no existe o no es texto.
pub(crate) fn read_registry_string(path: &str, value: &str) -> Option<String> {
    let subkey = HSTRING::from(path);
    let mut key = HKEY(std::ptr::null_mut());
    // SAFETY: abre una clave existente solo para leer; se cierra abajo.
    let opened = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            &subkey,
            None,
            KEY_READ,
            &mut key,
        )
    };
    if opened != ERROR_SUCCESS {
        return None;
    }
    let name = HSTRING::from(value);
    let mut kind = REG_VALUE_TYPE(0);
    let mut size: u32 = 0;
    // SAFETY: primera llamada solo para saber el tamaño, sin buffer.
    let sized = unsafe {
        RegQueryValueExW(
            key,
            &name,
            None,
            Some(&mut kind),
            None,
            Some(&mut size),
        )
    };
    if sized != ERROR_SUCCESS || size == 0 || size > 8192 {
        // SAFETY: la clave se abrió arriba y sigue válida.
        unsafe {
            let _ = RegCloseKey(key);
        }
        return None;
    }
    let count = (size as usize / 2).max(1);
    let mut buffer = vec![0_u16; count];
    let byte_len = (buffer.len() * 2) as u32;
    let mut actual = byte_len;
    // SAFETY: el buffer reserva `actual` bytes y se relee su tamaño.
    let read = unsafe {
        RegQueryValueExW(
            key,
            &name,
            None,
            Some(&mut kind),
            Some(buffer.as_mut_ptr() as *mut u8),
            Some(&mut actual),
        )
    };
    // SAFETY: la clave ya no se usa.
    unsafe {
        let _ = RegCloseKey(key);
    }
    if read != ERROR_SUCCESS {
        return None;
    }
    let used = (actual as usize / 2).min(buffer.len());
    let slice = &buffer[..used];
    let end = slice.iter().position(|unit| *unit == 0).unwrap_or(slice.len());
    String::from_utf16(&slice[..end]).ok()
}

/// Lee un `u32` del registro (`REG_DWORD`). `None` si no existe.
pub(crate) fn read_registry_dword(path: &str, value: &str) -> Option<u32> {
    let subkey = HSTRING::from(path);
    let mut key = HKEY(std::ptr::null_mut());
    // SAFETY: abre una clave existente solo para leer; se cierra abajo.
    let opened = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            &subkey,
            None,
            KEY_READ,
            &mut key,
        )
    };
    if opened != ERROR_SUCCESS {
        return None;
    }
    let name = HSTRING::from(value);
    let mut kind = REG_VALUE_TYPE(0);
    let mut data: u32 = 0;
    let mut size = size_of::<u32>() as u32;
    // SAFETY: `data` es un `u32` válido y `size` dice su tamaño.
    let read = unsafe {
        RegQueryValueExW(
            key,
            &name,
            None,
            Some(&mut kind),
            Some((&mut data as *mut u32) as *mut u8),
            Some(&mut size),
        )
    };
    // SAFETY: la clave ya no se usa.
    unsafe {
        let _ = RegCloseKey(key);
    }
    if read != ERROR_SUCCESS {
        return None;
    }
    Some(data)
}
