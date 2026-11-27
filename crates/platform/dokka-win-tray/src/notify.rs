//! Lee el aviso `Shell_NotifyIcon` que llega por `WM_COPYDATA`.
//!
//! Diseño propio con el formato que usan las barras alternativas: firma de
//! 32 bits, mensaje (`NIM_ADD`, `MODIFY`, `DELETE`, `SETFOCUS`, `SETVERSION`)
//! y un `NOTIFYICONDATAW` con los identificadores en 32 bits. Solo lee con
//! límites comprobados; lo corto o raro se ignora sin leer fuera.

/// Firma que abre el aviso de la bandeja.
const SIGNATURE: u32 = 0x3475_3423;
/// La estructura completa no pasa de este tamaño; lo mayor se ignora.
const LARGEST: usize = 4096;
/// Mínimo para leer la firma, el mensaje y las señas hasta `uFlags`.
const SMALLEST: usize = 24;

/// Posiciones absolutas (tras firma y mensaje) de cada campo.
const HWND_AT: usize = 12;
const UID_AT: usize = 16;
const FLAGS_AT: usize = 20;
const CALLBACK_AT: usize = 24;
const ICON_AT: usize = 28;
const TIP_AT: usize = 32;
/// Caracteres UTF-16 del consejo flotante.
const TIP_COUNT: usize = 128;
const STATE_AT: usize = 288;
const MASK_AT: usize = 292;
const VERSION_AT: usize = 808;
const GUID_AT: usize = 944;

/// La app recibirá el clic en este mensaje.
const NIF_MESSAGE: u32 = 1;
/// Trae icono nuevo.
const NIF_ICON: u32 = 2;
/// Trae consejo flotante nuevo.
const NIF_TIP: u32 = 4;
/// Trae estado nuevo (visible u oculto).
const NIF_STATE: u32 = 8;
/// La identidad es el GUID y no la ventana.
const NIF_GUID: u32 = 32;
/// Marca de icono oculto dentro del estado.
const NIS_HIDDEN: u32 = 1;

/// Qué pide la app con su aviso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    Add,
    Modify,
    Delete,
    Focus,
    Version,
}

/// Aviso ya leído, sin punteros: seguro de guardar y comparar.
#[derive(Debug, Clone)]
pub(crate) struct Parsed {
    pub(crate) action: Action,
    pub(crate) hwnd_raw: u32,
    pub(crate) uid: u32,
    pub(crate) callback: Option<u32>,
    pub(crate) tip: Option<String>,
    pub(crate) hidden: Option<bool>,
    pub(crate) version: Option<u32>,
    pub(crate) guid: Option<[u8; 16]>,
    pub(crate) icon_raw: Option<u32>,
}

/// Lee el aviso; `None` si es corto, raro o de otro tipo.
pub(crate) fn parse_bytes(bytes: &[u8]) -> Option<Parsed> {
    if bytes.len() < SMALLEST || bytes.len() > LARGEST {
        return None;
    }
    if read_u32(bytes, 0)? != SIGNATURE {
        return None;
    }
    let action = match read_u32(bytes, 4)? {
        0 => Action::Add,
        1 => Action::Modify,
        2 => Action::Delete,
        3 => Action::Focus,
        4 => Action::Version,
        _ => return None,
    };
    let hwnd_raw = read_u32(bytes, HWND_AT)?;
    let uid = read_u32(bytes, UID_AT)?;
    let flags = read_u32(bytes, FLAGS_AT)?;
    let callback = if flags & NIF_MESSAGE != 0 {
        Some(read_u32(bytes, CALLBACK_AT)?)
    } else {
        None
    };
    let icon_raw = if flags & NIF_ICON != 0 {
        Some(read_u32(bytes, ICON_AT)?)
    } else {
        None
    };
    let tip = if flags & NIF_TIP != 0 {
        Some(decode_tip(bytes))
    } else {
        None
    };
    let hidden = if flags & NIF_STATE != 0 {
        let state = read_u32(bytes, STATE_AT)?;
        let mask = read_u32(bytes, MASK_AT)?;
        Some(mask & NIS_HIDDEN != 0 && state & NIS_HIDDEN != 0)
    } else {
        None
    };
    // Solo `SETVERSION` trae versión; en los demás el campo es un tiempo.
    let version = if action == Action::Version {
        Some(read_u32(bytes, VERSION_AT)?)
    } else {
        None
    };
    let guid = if flags & NIF_GUID != 0 {
        let end = GUID_AT.checked_add(16)?;
        let slice = bytes.get(GUID_AT..end)?;
        let array: [u8; 16] = slice.try_into().ok()?;
        Some(array)
    } else {
        None
    };
    Some(Parsed {
        action,
        hwnd_raw,
        uid,
        callback,
        tip,
        hidden,
        version,
        guid,
        icon_raw,
    })
}

/// Consejo flotante en UTF-16 hasta el primer cero o el fin disponible.
fn decode_tip(bytes: &[u8]) -> String {
    let mut units = Vec::new();
    for index in 0..TIP_COUNT {
        let offset = TIP_AT.saturating_add(index.saturating_mul(2));
        let Some(end) = offset.checked_add(2) else {
            break;
        };
        let Some(slice) = bytes.get(offset..end) else {
            break;
        };
        let Ok(array) = slice.try_into() else {
            break;
        };
        let unit = u16::from_le_bytes(array);
        if unit == 0 {
            break;
        }
        units.push(unit);
    }
    String::from_utf16_lossy(&units)
}

/// Entero de 4 bytes en orden de Windows, si cabe entero.
fn read_u32(bytes: &[u8], at: usize) -> Option<u32> {
    let end = at.checked_add(4)?;
    let slice = bytes.get(at..end)?;
    let array: [u8; 4] = slice.try_into().ok()?;
    Some(u32::from_le_bytes(array))
}
