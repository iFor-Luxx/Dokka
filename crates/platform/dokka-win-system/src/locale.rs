//! Idiomas preferidos, región y formatos desde Windows.
//!
//! Solo lee; nunca cambia nada. Lo que no se lee queda en `None` y se
//! muestra como "No disponible". Los ejemplos (fecha larga, corta y número)
//! usan el formato del usuario con la fecha de hoy.

use dokka_model::LocaleSummary;
use windows::Win32::Globalization::{
    DATE_LONGDATE, DATE_SHORTDATE, ENUM_DATE_FORMATS_FLAGS, GetDateFormatW, GetLocaleInfoW,
    GetNumberFormatW, GetUserDefaultLCID, GetUserPreferredUILanguages, LOCALE_ICALENDARTYPE,
    LOCALE_IMEASURE, LOCALE_SDECIMAL, LOCALE_SLOCALIZEDCOUNTRYNAME, LOCALE_SLOCALIZEDDISPLAYNAME,
    LOCALE_SNAME, LOCALE_SSHORTDATE, LOCALE_STHOUSAND, LOCALE_USER_DEFAULT, MUI_LANGUAGE_NAME,
    LocaleNameToLCID,
};
use windows::core::{HSTRING, PCWSTR, PWSTR};

/// Lee la tarjeta de idioma y región de una vez, sin detenerse en fallos.
pub(crate) fn read_locale() -> LocaleSummary {
    // SAFETY: no recibe argumentos y devuelve el idioma del usuario.
    let lcid = unsafe { GetUserDefaultLCID() };
    let decimal = locale_text(lcid, LOCALE_SDECIMAL);
    let thousand = locale_text(lcid, LOCALE_STHOUSAND);
    let measure = locale_text(lcid, LOCALE_IMEASURE);
    LocaleSummary {
        languages: user_languages(),
        region_name: locale_text(lcid, LOCALE_SLOCALIZEDCOUNTRYNAME)
            .or_else(|| locale_text(lcid, LOCALE_SNAME)),
        calendar_name: calendar_name(lcid),
        temperature_unit: temperature_unit(measure.as_deref()),
        measurement_system: measurement_system(measure.as_deref()),
        date_pattern: locale_text(lcid, LOCALE_SSHORTDATE),
        number_format: number_format(decimal.as_deref(), thousand.as_deref()),
        long_date_example: format_date(DATE_LONGDATE),
        short_date_example: format_date(DATE_SHORTDATE),
        number_example: format_number("1234567.89"),
    }
}

/// Idiomas preferidos del usuario, el primero es el principal.
fn user_languages() -> Option<Vec<String>> {
    let mut count = 0u32;
    let mut size = 0u32;
    // SAFETY: sin búfer solo pide el tamaño necesario; el error de
    // búfer corto se ignora y se usa `size`.
    let _ = unsafe { GetUserPreferredUILanguages(MUI_LANGUAGE_NAME, &mut count, None, &mut size) };
    if count == 0 || size == 0 || size > 4096 {
        return None;
    }
    let mut buffer = vec![0u16; size as usize];
    // SAFETY: el búfer reserva `size` unidades recién pedidas.
    let read = unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut count,
            Some(PWSTR(buffer.as_mut_ptr())),
            &mut size,
        )
    };
    if read.is_err() {
        return None;
    }
    let mut languages = Vec::new();
    for tag in split_multi_sz(&buffer) {
        if tag.is_empty() {
            continue;
        }
        languages.push(display_name(&tag).unwrap_or(tag));
    }
    (!languages.is_empty()).then_some(languages)
}

/// Nombre del idioma en el idioma del usuario ("español (España)").
fn display_name(tag: &str) -> Option<String> {
    let name = HSTRING::from(tag);
    // SAFETY: la cadena vive durante toda la llamada; 0 es etiqueta mala.
    let lcid = unsafe { LocaleNameToLCID(&name, 0) };
    if lcid == 0 {
        return None;
    }
    locale_text(lcid, LOCALE_SLOCALIZEDDISPLAYNAME)
}

/// Calendario del usuario: solo el gregoriano tiene nombre seguro.
fn calendar_name(lcid: u32) -> Option<String> {
    match locale_text(lcid, LOCALE_ICALENDARTYPE).as_deref() {
        Some("1") => Some("Gregoriano".to_owned()),
        _ => None,
    }
}

/// Temperatura desde el sistema de medida: métrico es Celsius.
fn temperature_unit(measure: Option<&str>) -> Option<String> {
    match measure {
        Some("0") => Some("Celsius".to_owned()),
        Some("1") => Some("Fahrenheit".to_owned()),
        _ => None,
    }
}

/// Sistema de medida: métrico (0) o estadounidense (1).
fn measurement_system(measure: Option<&str>) -> Option<String> {
    match measure {
        Some("0") => Some("Métrico".to_owned()),
        Some("1") => Some("EE. UU.".to_owned()),
        _ => None,
    }
}

/// Número con los separadores del usuario, como "1.234,56".
fn number_format(decimal: Option<&str>, thousand: Option<&str>) -> Option<String> {
    let (decimal, thousand) = (decimal?, thousand?);
    if decimal.is_empty() || thousand.is_empty() {
        return None;
    }
    Some(format!("1{thousand}234{decimal}56"))
}

/// Fecha de hoy con el formato del usuario (larga o corta).
fn format_date(flags: ENUM_DATE_FORMATS_FLAGS) -> Option<String> {
    // SAFETY: sin búfer solo pide el tamaño, en caracteres con el nulo final.
    let size = unsafe { GetDateFormatW(LOCALE_USER_DEFAULT, flags.0, None, PCWSTR::null(), None) };
    if size <= 0 || size > 512 {
        return None;
    }
    let mut buffer = vec![0u16; size as usize];
    // SAFETY: el búfer declara su tamaño; Windows no escribe más allá.
    let written = unsafe {
        GetDateFormatW(
            LOCALE_USER_DEFAULT,
            flags.0,
            None,
            PCWSTR::null(),
            Some(buffer.as_mut_slice()),
        )
    };
    if written <= 0 {
        return None;
    }
    widen(&buffer[..written as usize])
}

/// Número con el formato del usuario, como "1.234.567,89".
fn format_number(value: &str) -> Option<String> {
    let text = HSTRING::from(value);
    // SAFETY: sin búfer solo pide el tamaño, en caracteres con el nulo final.
    let size = unsafe { GetNumberFormatW(LOCALE_USER_DEFAULT, 0, &text, None, None) };
    if size <= 0 || size > 64 {
        return None;
    }
    let mut buffer = vec![0u16; size as usize];
    // SAFETY: el búfer declara su tamaño; Windows no escribe más allá.
    let written = unsafe {
        GetNumberFormatW(
            LOCALE_USER_DEFAULT,
            0,
            &text,
            None,
            Some(buffer.as_mut_slice()),
        )
    };
    if written <= 0 {
        return None;
    }
    widen(&buffer[..written as usize])
}

/// Texto de un campo del idioma del usuario, como hace el teclado.
fn locale_text(lcid: u32, field: u32) -> Option<String> {
    // SAFETY: sin búfer solo pide el tamaño, en caracteres con el nulo final.
    let needed = unsafe { GetLocaleInfoW(lcid, field, None) };
    if needed <= 0 || needed > 512 {
        return None;
    }
    let mut buffer = vec![0u16; needed as usize];
    // SAFETY: el búfer declara su tamaño; Windows no escribe más allá.
    if unsafe { GetLocaleInfoW(lcid, field, Some(buffer.as_mut_slice())) } <= 0 {
        return None;
    }
    widen(&buffer)
}

/// Cadenas doblemente terminadas en nulo, sin el nulo final.
fn split_multi_sz(buffer: &[u16]) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    for (index, unit) in buffer.iter().enumerate() {
        if *unit != 0 {
            continue;
        }
        if index > start {
            parts.push(String::from_utf16_lossy(&buffer[start..index]));
        }
        start = index + 1;
    }
    parts
}

/// Texto UTF-16 hasta el primer nulo, sin aire ni vacíos.
fn widen(units: &[u16]) -> Option<String> {
    let end = units.iter().position(|unit| *unit == 0).unwrap_or(units.len());
    let text = String::from_utf16(&units[..end]).ok()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}
