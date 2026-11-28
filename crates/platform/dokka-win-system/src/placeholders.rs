//! Textos de relleno de los PC montados a mano.
//!
//! Si el fabricante no rellena el SMBIOS o el registro, Windows devuelve
//! estos textos. Se tratan como no disponibles.

/// `true` si el texto es un relleno sin valor real.
pub(crate) fn is_placeholder(text: &str) -> bool {
    let folded = text.trim().to_ascii_lowercase();
    matches!(
        folded.as_str(),
        ""
            | "unknown"
            | "none"
            | "not specified"
            | "not available"
            | "no disponible"
            | "default string"
            | "system product name"
            | "system serial number"
            | "system manufacturer"
            | "system version"
            | "to be filled by o.e.m."
            | "to be filled by oem"
            | "oem"
            | "serial number"
            | "chassis serial number"
    )
}

/// Limpia un texto leído: recorta y convierte el relleno en `None`.
pub(crate) fn clean(value: Option<String>) -> Option<String> {
    let text = value?;
    let trimmed = text.trim().to_owned();
    if trimmed.is_empty() || is_placeholder(&trimmed) {
        return None;
    }
    Some(trimmed)
}
