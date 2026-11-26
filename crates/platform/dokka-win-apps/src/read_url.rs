use std::path::Path;

/// Protocolo de un `.url` que no sea web ni archivo, como juegos de Steam.
pub(crate) fn game_protocol(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.len() < 5 || !line.to_ascii_lowercase().starts_with("url=") {
            continue;
        }
        let address = line.get(4..)?.trim();
        let first = address.split("://").next()?;
        let raw = first.split(':').next()?;
        let protocol = raw.trim().to_lowercase();
        if protocol.is_empty() {
            return None;
        }
        if protocol == "http" || protocol == "https" || protocol == "file" {
            return None;
        }
        return Some(protocol);
    }
    None
}
