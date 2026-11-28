//! Lista de zonas horarias con su nombre visible traducido.
//!
//! Solo lee; nunca cambia nada. Enumera con `EnumDynamicTimeZoneInformation`
//! y traduce `Display` con `SHLoadIndirectString`. Ordenadas por nombre
//! visible para el desplegable.

use dokka_contracts::TimezoneOption;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_VALUE_TYPE, RegCloseKey, RegOpenKeyExW,
    RegQueryValueExW,
};
use windows::Win32::System::Time::{DYNAMIC_TIME_ZONE_INFORMATION, EnumDynamicTimeZoneInformation};
use windows::Win32::UI::Shell::SHLoadIndirectString;
use windows::core::HSTRING;

/// Todas las zonas con su nombre visible, ordenadas por visible.
pub(crate) fn list_timezones() -> Vec<TimezoneOption> {
    let mut zones = Vec::new();
    let mut index: u32 = 0;
    loop {
        let mut info = DYNAMIC_TIME_ZONE_INFORMATION::default();
        // SAFETY: `info` declara su tamaño y Windows la rellena.
        let done = unsafe { EnumDynamicTimeZoneInformation(index, &mut info) };
        if done != ERROR_SUCCESS.0 {
            break;
        }
        let Some(key) = zone_key(&info.TimeZoneKeyName) else {
            index = index.saturating_add(1);
            if index > 512 {
                break;
            }
            continue;
        };
        let display = zone_display(&key).unwrap_or_else(|| key.clone());
        zones.push(TimezoneOption { key, display });
        index = index.saturating_add(1);
        if index > 512 {
            break;
        }
    }
    zones.sort_by(|left, right| {
        left.display
            .to_lowercase()
            .cmp(&right.display.to_lowercase())
    });
    zones
}

/// Clave en inglés de una zona enumerada, hasta el primer nulo.
fn zone_key(units: &[u16]) -> Option<String> {
    let end = units
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(units.len());
    let text = String::from_utf16(&units[..end]).ok()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

/// Nombre visible traducido de la clave, o `None` si no se lee.
fn zone_display(key: &str) -> Option<String> {
    let path = format!(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Time Zones\{key}"
    );
    let raw = read_registry_string(&path, "MUI_Display")
        .or_else(|| read_registry_string(&path, "Display"))?;
    let text = translate(&raw).unwrap_or(raw);
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

/// Traduce `@tzres.dll,-1120` con `SHLoadIndirectString`.
fn translate(source: &str) -> Option<String> {
    if !source.starts_with('@') {
        return Some(source.to_owned());
    }
    let origin = HSTRING::from(source);
    let mut out = vec![0_u16; 512];
    // SAFETY: `out` reserva 512 unidades y solo se lee lo traducido.
    let translated = unsafe { SHLoadIndirectString(&origin, &mut out, None).is_ok() };
    if !translated {
        return None;
    }
    let end = out.iter().position(|unit| *unit == 0).unwrap_or(out.len());
    String::from_utf16(&out[..end]).ok()
}

/// Lee una cadena del registro local. `None` si no existe o no es texto.
fn read_registry_string(path: &str, value: &str) -> Option<String> {
    let subkey = HSTRING::from(path);
    let mut key = HKEY(std::ptr::null_mut());
    // SAFETY: abre una clave existente solo para leer; se cierra abajo.
    let opened = unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, &subkey, None, KEY_READ, &mut key) };
    if opened != ERROR_SUCCESS {
        return None;
    }
    let name = HSTRING::from(value);
    let mut kind = REG_VALUE_TYPE(0);
    let mut size: u32 = 0;
    // SAFETY: primera llamada solo para saber el tamaño, sin buffer.
    let sized =
        unsafe { RegQueryValueExW(key, &name, None, Some(&mut kind), None, Some(&mut size)) };
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
    let end = slice
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(slice.len());
    String::from_utf16(&slice[..end]).ok()
}
