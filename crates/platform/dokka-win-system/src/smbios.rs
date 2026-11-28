//! Tabla SMBIOS (`RSMB`) para el número de serie y el modelo base.
//!
//! Los PC montados a mano dejan el relleno de fábrica: se filtra fuera.
//! Si la tabla no se puede leer, todo queda en `None` y el resto sigue.

use windows::Win32::System::SystemInformation::{GetSystemFirmwareTable, RSMB};

use super::placeholders::clean;

/// Serie del equipo (tipo 1) y modelo de la placa (tipo 2), si se leen.
pub(crate) struct Smbios {
    pub system_serial: Option<String>,
    pub system_product: Option<String>,
    pub system_manufacturer: Option<String>,
    pub board_product: Option<String>,
    pub board_manufacturer: Option<String>,
}

/// Lee la tabla SMBIOS completa, o `None` si Windows no la da.
pub(crate) fn read_smbios() -> Option<Smbios> {
    // SAFETY: primera llamada con buffer vacío solo para saber el tamaño.
    let size = unsafe { GetSystemFirmwareTable(RSMB, 0, None) };
    if size == 0 || size > 1024 * 1024 {
        return None;
    }
    let mut buffer = vec![0_u8; size as usize];
    // SAFETY: el buffer reserva `size` bytes recién pedidos.
    let read = unsafe { GetSystemFirmwareTable(RSMB, 0, Some(buffer.as_mut_slice())) };
    if read == 0 || read as usize > buffer.len() {
        return None;
    }
    buffer.truncate(read as usize);
    Some(parse(&buffer))
}

/// Recorre las estructuras SMBIOS y saca los tipos 1 (sistema) y 2 (placa).
fn parse(buffer: &[u8]) -> Smbios {
    let table = skip_header(buffer);
    let mut system_serial = None;
    let mut system_product = None;
    let mut system_manufacturer = None;
    let mut board_product = None;
    let mut board_manufacturer = None;
    let mut offset = 0_usize;
    while offset + 4 <= table.len() {
        let kind = table[offset];
        let length = table[offset + 1] as usize;
        if length < 4 || offset + length > table.len() {
            break;
        }
        let strings = read_strings(table, offset + length);
        let step = structure_len(table, offset);
        match kind {
            1 => {
                system_manufacturer = clean(string_at(&strings, table.get(offset + 4)));
                system_product = clean(string_at(&strings, table.get(offset + 5)));
                system_serial = clean(string_at(&strings, table.get(offset + 7)));
            }
            2 => {
                if board_manufacturer.is_none() {
                    board_manufacturer = clean(string_at(&strings, table.get(offset + 4)));
                }
                if board_product.is_none() {
                    board_product = clean(string_at(&strings, table.get(offset + 5)));
                }
            }
            _ => {}
        }
        if step == 0 {
            break;
        }
        offset += step;
        // El final de la tabla son dos ceros seguidos.
        if offset + 2 <= table.len() && table[offset] == 0 && table[offset + 1] == 0 {
            break;
        }
    }
    Smbios {
        system_serial,
        system_product,
        system_manufacturer,
        board_product,
        board_manufacturer,
    }
}

/// La tabla `RSMB` empieza con 8 bytes propios (método, versiones y tamaño).
/// Si encajan, se saltan; si no, se lee desde el principio.
fn skip_header(buffer: &[u8]) -> &[u8] {
    if buffer.len() > 8 {
        let length = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) as usize;
        if length == buffer.len() - 8 {
            return &buffer[8..];
        }
    }
    buffer
}

/// Cadenas de una estructura: índices desde 1; el 0 es vacío.
fn read_strings(table: &[u8], mut at: usize) -> Vec<String> {
    let mut strings = Vec::new();
    while at < table.len() {
        if table[at] == 0 {
            break;
        }
        let end = table[at..]
            .iter()
            .position(|unit| *unit == 0)
            .map_or(table.len(), |found| at + found);
        let text = String::from_utf8_lossy(&table[at..end]).into_owned();
        strings.push(text);
        at = end + 1;
    }
    strings
}

/// Índice 1-based a la cadena, o `None`.
fn string_at(strings: &[String], index: Option<&u8>) -> Option<String> {
    let position = usize::from(*index?) ;
    if position == 0 {
        return None;
    }
    strings.get(position - 1).cloned()
}

/// Avance desde el inicio de la estructura hasta la siguiente:
/// recorre las cadenas hasta su doble cero final.
fn structure_len(table: &[u8], offset: usize) -> usize {
    let length = table.get(offset + 1).copied().unwrap_or(0) as usize;
    if length < 4 {
        return 0;
    }
    let mut at = offset + length;
    while at + 1 < table.len() {
        if table[at] == 0 && table[at + 1] == 0 {
            return at + 2 - offset;
        }
        at += 1;
    }
    0
}
