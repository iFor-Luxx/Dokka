//! Cambios de fecha, hora y zona en el hilo propio.
//!
//! Solo escribe tras un clic; nunca lee la interfaz. Cada fallo se devuelve
//! como `false` para que el hilo lo registre, relea y la pantalla muestre
//! el valor real. Nada se deshace al cerrar Dokka.

use windows::Win32::Foundation::{ERROR_SUCCESS, WIN32_ERROR};
use windows::Win32::Security::SE_TIME_ZONE_NAME;
use windows::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD, REG_SZ, RegCloseKey, RegOpenKeyExW,
    RegSetValueExW,
};
use windows::Win32::System::Services::{
    CloseServiceHandle, ControlService, OpenSCManagerW, OpenServiceW, SC_MANAGER_CONNECT,
    SERVICE_ALL_ACCESS, SERVICE_CONTROL_PARAMCHANGE, SERVICE_STATUS,
};
use windows::Win32::System::SystemInformation::SetLocalTime;
use windows::Win32::System::Time::{DYNAMIC_TIME_ZONE_INFORMATION, EnumDynamicTimeZoneInformation};
use windows::core::HSTRING;

/// Servidor válido: 1-253 letras, números, puntos y guiones.
pub(crate) fn is_valid_ntp_server(server: &str) -> bool {
    if server.is_empty() || server.len() > 253 {
        return false;
    }
    server.bytes().all(|unit| {
        unit.is_ascii_alphanumeric() || unit == b'.' || unit == b'-'
    })
}

/// Automáticas (`NTP`) o manual (`NoSync`) y aviso al servicio.
pub(crate) fn set_automatic_time(enabled: bool) -> bool {
    let value = if enabled { "NTP" } else { "NoSync" };
    if !write_registry_string(w32time(), "Type", value) {
        return false;
    }
    notify_w32time()
}

/// Servidor NTP validado, con su marca `,0x9`, y aviso al servicio.
pub(crate) fn set_ntp_server(server: &str) -> bool {
    let host = server.trim();
    if !is_valid_ntp_server(host) {
        return false;
    }
    if !write_registry_string(w32time(), "NtpServer", &format!("{host},0x9")) {
        return false;
    }
    notify_w32time()
}

/// Fecha y hora manual; el día ya viene válido de la isla.
pub(crate) fn set_manual_time(year: u16, month: u16, day: u16, hour: u16, minute: u16) -> bool {
    if month == 0 || month > 12 || day == 0 || day > 31 || hour > 23 || minute > 59 {
        return false;
    }
    let time = windows::Win32::Foundation::SYSTEMTIME {
        wYear: year,
        wMonth: month,
        wDayOfWeek: 0,
        wDay: day,
        wHour: hour,
        wMinute: minute,
        wSecond: 0,
        wMilliseconds: 0,
    };
    // SAFETY: `time` es una fecha válida y solo se lee en la llamada.
    unsafe { SetLocalTime(&time).is_ok() }
}

/// Zona automática: `tzautoupdate` en manual (3) o deshabilitado (4).
pub(crate) fn set_automatic_timezone(enabled: bool) -> bool {
    let start: u32 = if enabled { 3 } else { 4 };
    write_registry_dword(tz_auto(), "Start", start)
}

/// Zona por su clave, con `SeTimeZonePrivilege` solo en la llamada.
pub(crate) fn set_timezone(key: &str) -> bool {
    let Some(info) = find_zone(key) else {
        return false;
    };
    super::datetime_elevation::with_privilege(SE_TIME_ZONE_NAME, || {
        // SAFETY: `info` viene de enumerar zonas válidas de Windows.
        unsafe { SetDynamicTimeZoneInformation(&info).is_ok() }
    })
}

/// Clave de parámetros de `W32Time`.
fn w32time() -> &'static str {
    r"SYSTEM\CurrentControlSet\Services\W32Time\Parameters"
}

/// Clave del servicio de zona automática.
fn tz_auto() -> &'static str {
    r"SYSTEM\CurrentControlSet\Services\tzautoupdate"
}

/// Avisa a `W32Time` de que cambió su configuración.
fn notify_w32time() -> bool {
    // SAFETY: solo conecta con el gestor local y cierra lo abierto abajo.
    let manager = unsafe {
        match OpenSCManagerW(None, None, SC_MANAGER_CONNECT) {
            Ok(manager) => manager,
            Err(_) => return false,
        }
    };
    // SAFETY: el gestor sigue válido y el servicio se cierra abajo.
    let service = unsafe {
        match OpenServiceW(manager, HSTRING::from("W32Time"), SERVICE_ALL_ACCESS) {
            Ok(service) => service,
            Err(_) => {
                let _ = CloseServiceHandle(manager);
                return false;
            }
        }
    };
    let mut status = SERVICE_STATUS::default();
    // SAFETY: el servicio se abrió arriba y el estado es válido.
    let notified = unsafe { ControlService(service, SERVICE_CONTROL_PARAMCHANGE, &mut status).is_ok() };
    // SAFETY: ambos se abrieron arriba y ya no se usan.
    unsafe {
        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(manager);
    }
    notified
}

/// Busca la zona con esta clave enumerando las de Windows.
fn find_zone(key: &str) -> Option<DYNAMIC_TIME_ZONE_INFORMATION> {
    let mut index: u32 = 0;
    loop {
        let mut info = DYNAMIC_TIME_ZONE_INFORMATION::default();
        // SAFETY: `info` declara su tamaño y Windows la rellena.
        let done = unsafe { EnumDynamicTimeZoneInformation(index, &mut info) };
        if done != ERROR_SUCCESS.0 {
            return None;
        }
        if zone_key(&info.TimeZoneKeyName) == Some(key.to_owned()) {
            return Some(info);
        }
        index = index.saturating_add(1);
        if index > 512 {
            return None;
        }
    }
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

/// Escribe una cadena `REG_SZ` en `HKLM`. `false` si no se pudo.
fn write_registry_string(path: &str, value: &str, data: &str) -> bool {
    let subkey = HSTRING::from(path);
    let mut key = HKEY(std::ptr::null_mut());
    // SAFETY: abre la clave existente solo para escribir y se cierra abajo.
    let opened = unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, &subkey, None, KEY_SET_VALUE, &mut key) };
    if opened != ERROR_SUCCESS {
        return false;
    }
    let name = HSTRING::from(value);
    let wide: Vec<u16> = data.encode_utf16().chain([0]).collect();
    let bytes = wide_bytes(&wide);
    // SAFETY: `bytes` son los bytes UTF-16 con nulo de `data`.
    let written: WIN32_ERROR = unsafe {
        RegSetValueExW(
            key,
            &name,
            None,
            REG_SZ,
            Some(bytes),
        )
    };
    // SAFETY: la clave se abrió arriba y ya no se usa.
    unsafe {
        let _ = RegCloseKey(key);
    }
    written == ERROR_SUCCESS
}

/// Escribe un `REG_DWORD` en `HKLM`. `false` si no se pudo.
fn write_registry_dword(path: &str, value: &str, data: u32) -> bool {
    let subkey = HSTRING::from(path);
    let mut key = HKEY(std::ptr::null_mut());
    // SAFETY: abre la clave existente solo para escribir y se cierra abajo.
    let opened = unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, &subkey, None, KEY_SET_VALUE, &mut key) };
    if opened != ERROR_SUCCESS {
        return false;
    }
    let name = HSTRING::from(value);
    let bytes = data.to_le_bytes();
    // SAFETY: `bytes` son los 4 bytes del `u32` en orden local.
    let written: WIN32_ERROR = unsafe {
        RegSetValueExW(key, &name, None, REG_DWORD, Some(&bytes))
    };
    // SAFETY: la clave se abrió arriba y ya no se usa.
    unsafe {
        let _ = RegCloseKey(key);
    }
    written == ERROR_SUCCESS
}

/// Vista de `u16` como bytes para `RegSetValueExW`.
fn wide_bytes(wide: &[u16]) -> &[u8] {
    // SAFETY: `u16` y `u8` son planos y el corte usa su tamaño real.
    unsafe {
        std::slice::from_raw_parts(
            wide.as_ptr().cast::<u8>(),
            wide.len() * size_of::<u16>(),
        )
    }
}
