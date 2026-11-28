//! Fecha, hora y zona horaria desde Windows.
//!
//! Solo lee; nunca cambia nada. Lo que no se lee queda en `None` y se
//! muestra como "No disponible".

use dokka_model::DateTimeSummary;
use windows::Win32::System::Time::{DYNAMIC_TIME_ZONE_INFORMATION, GetDynamicTimeZoneInformation};

use super::placeholders::clean;
use super::registry::{read_registry_dword, read_registry_string};

/// Lee la tarjeta de fecha y hora de una vez, sin detenerse en fallos.
pub(crate) fn read_datetime() -> DateTimeSummary {
    DateTimeSummary {
        auto_time: auto_time(),
        ntp_server: ntp_server(),
        auto_timezone: auto_timezone(),
        timezone_name: timezone_name(),
    }
}

/// `true` con la hora automática: `Type` vale "NTP", en manual "NoSync".
fn auto_time() -> Option<bool> {
    match read_registry_string(w32time(), "Type").as_deref() {
        Some("NTP") => Some(true),
        Some("NoSync") => Some(false),
        _ => None,
    }
}

/// Servidor NTP, sin el ",0x9" que añade Windows al final.
fn ntp_server() -> Option<String> {
    let raw = read_registry_string(w32time(), "NtpServer")?;
    let host = raw.split([',', ' ']).next().unwrap_or("").trim();
    if host.is_empty() {
        return None;
    }
    clean(Some(host.to_owned()))
}

/// `true` con la zona automática: el servicio `tzautoupdate` en manual (3)
/// la pone sola; deshabilitado (4) queda fija.
fn auto_timezone() -> Option<bool> {
    match read_registry_dword(r"SYSTEM\CurrentControlSet\Services\tzautoupdate", "Start") {
        Some(3) => Some(true),
        Some(4) => Some(false),
        _ => None,
    }
}

/// Nombre de la zona actual, como "SA Pacific Standard Time".
fn timezone_name() -> Option<String> {
    // SAFETY: la estructura declara su tamaño y Windows la rellena.
    let info = unsafe {
        let mut info = DYNAMIC_TIME_ZONE_INFORMATION::default();
        // `u32::MAX` es `TIME_ZONE_ID_INVALID`: sin zona válida no hay nombre.
        if GetDynamicTimeZoneInformation(&mut info) == u32::MAX {
            return None;
        }
        info
    };
    widen(&info.TimeZoneKeyName)
        .or_else(|| widen(&info.StandardName))
        .and_then(|name| clean(Some(name)))
}

/// Texto UTF-16 hasta el primer nulo, sin aire ni vacíos.
fn widen(units: &[u16]) -> Option<String> {
    let end = units
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(units.len());
    let text = String::from_utf16(&units[..end]).ok()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

fn w32time() -> &'static str {
    r"SYSTEM\CurrentControlSet\Services\W32Time\Parameters"
}
