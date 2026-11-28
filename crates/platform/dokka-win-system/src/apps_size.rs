//! Tamaño instalado de las aplicaciones del registro.
//!
//! Suma `EstimatedSize` (en KB) de los programas de las ramas de
//! desinstalación, en las dos vistas de 32/64 bits y por usuario y equipo.
//! Solo lee; nunca escribe. Lo que no se puede leer se ignora.

use windows::Win32::Foundation::{ERROR_NO_MORE_ITEMS, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, REG_VALUE_TYPE, RegCloseKey,
    RegEnumKeyExW, RegOpenKeyExW, RegQueryValueExW,
};
use windows::core::{HSTRING, PWSTR};

/// Ramas de desinstalación que se suman.
const UNINSTALL_ROOTS: [(HKEY, &str); 4] = [
    (
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
    ),
    (
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ),
    (
        HKEY_CURRENT_USER,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
    ),
    (
        HKEY_CURRENT_USER,
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ),
];

/// Suma instalada en bytes; `None` si no se leyó ninguna rama.
pub(crate) fn installed_apps_size() -> Option<u64> {
    let mut total_kb: u64 = 0;
    let mut read_any = false;
    for (hive, root) in UNINSTALL_ROOTS {
        let Some(names) = subkey_names(hive, root) else {
            continue;
        };
        read_any = true;
        for name in names {
            let key = format!(r"{root}\{name}");
            total_kb = total_kb.saturating_add(estimated_kb(hive, &key));
        }
    }
    read_any.then(|| total_kb.saturating_mul(1024))
}

/// Nombres de las subclaves de la rama; `None` si no se abre.
fn subkey_names(hive: HKEY, path: &str) -> Option<Vec<String>> {
    let subkey = HSTRING::from(path);
    let mut key = HKEY(std::ptr::null_mut());
    // SAFETY: abre una clave existente solo para leer; se cierra abajo.
    let opened = unsafe { RegOpenKeyExW(hive, &subkey, None, KEY_READ, &mut key) };
    if opened != ERROR_SUCCESS {
        return None;
    }
    let mut names = Vec::new();
    let mut index = 0;
    loop {
        let mut name = [0_u16; 256];
        let mut length = name.len() as u32;
        // SAFETY: el buffer reserva `length` unidades y se relee su tamaño.
        let status = unsafe {
            RegEnumKeyExW(
                key,
                index,
                PWSTR(name.as_mut_ptr()),
                &mut length as *mut u32,
                None,
                PWSTR::null(),
                None,
                None,
            )
        };
        if status == ERROR_NO_MORE_ITEMS {
            break;
        }
        if status != ERROR_SUCCESS {
            break;
        }
        let used = (length as usize).min(name.len());
        if let Ok(text) = String::from_utf16(&name[..used])
            && !text.trim().is_empty()
        {
            names.push(text);
        }
        index += 1;
    }
    // SAFETY: la clave se abrió arriba y ya no se usa.
    unsafe {
        let _ = RegCloseKey(key);
    }
    Some(names)
}

/// `EstimatedSize` de un programa en KB; 0 si no lo declara.
fn estimated_kb(hive: HKEY, path: &str) -> u64 {
    let subkey = HSTRING::from(path);
    let mut key = HKEY(std::ptr::null_mut());
    // SAFETY: abre una clave existente solo para leer; se cierra abajo.
    let opened = unsafe { RegOpenKeyExW(hive, &subkey, None, KEY_READ, &mut key) };
    if opened != ERROR_SUCCESS {
        return 0;
    }
    let name = HSTRING::from("EstimatedSize");
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
    if read == ERROR_SUCCESS {
        u64::from(data)
    } else {
        0
    }
}
