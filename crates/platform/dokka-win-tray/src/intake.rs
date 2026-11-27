//! Entrada de avisos: registra el icono sin impedir nunca el reenvío.
//!
//! Todo con límites comprobados y sin el candado tomado al convertir: la
//! app puede destruir su `HICON` en cuanto vuelva el mensaje. Si algo
//! falla, simplemente no se registra.

use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use dokka_win_apps::tray_icon_bitmap;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::System::DataExchange::COPYDATASTRUCT;
use windows::Win32::UI::WindowsAndMessaging::HICON;

use crate::notify;
use crate::state::{IconUpdate, Shared};

/// Registra el icono si el mensaje es `Shell_NotifyIcon` válido.
pub(crate) fn note_shell_icon(shared: &Arc<Mutex<Shared>>, lparam: LPARAM) {
    let Some(parsed) = shell_parsed(lparam) else {
        return;
    };
    let icon = convert_icon(&parsed);
    if let Ok(mut state) = shared.lock() {
        state.apply(&parsed, icon);
    }
}

/// `true` si el mensaje trae un aviso de icono (para acusar sin barra real).
pub(crate) fn is_shell_notify(lparam: LPARAM) -> bool {
    if lparam.0 == 0 {
        return false;
    }
    // SAFETY: lectura del primer campo durante `WM_COPYDATA`.
    unsafe {
        (lparam.0 as *const COPYDATASTRUCT)
            .as_ref()
            .is_some_and(|info| info.dwData == 1)
    }
}

/// Extrae el aviso con límites comprobados; `None` si no es icono válido.
fn shell_parsed(lparam: LPARAM) -> Option<notify::Parsed> {
    if lparam.0 == 0 {
        return None;
    }
    // SAFETY: Windows entrega un `COPYDATASTRUCT` válido en este mensaje.
    let (kind, count, pointer) = unsafe {
        let info = (lparam.0 as *const COPYDATASTRUCT).as_ref()?;
        (info.dwData, info.cbData, info.lpData)
    };
    if kind != 1 || pointer.is_null() {
        return None;
    }
    if !(24..=4096).contains(&count) {
        return None;
    }
    // SAFETY: `pointer` apunta a `count` bytes mientras se atiende el mensaje.
    let bytes = unsafe { core::slice::from_raw_parts(pointer as *const u8, count as usize) };
    notify::parse_bytes(bytes)
}

/// Convierte el icono al recibirlo, sin el candado tomado. `None` conserva
/// el anterior si Windows no lo pudo dibujar.
fn convert_icon(parsed: &notify::Parsed) -> IconUpdate {
    let Some(raw) = parsed.icon_raw else {
        return IconUpdate::Keep;
    };
    if raw == 0 {
        return IconUpdate::Set(None);
    }
    let icon = HICON(raw as usize as *mut c_void);
    match tray_icon_bitmap(icon) {
        Some(bitmap) => IconUpdate::Set(Some(Arc::new(bitmap))),
        None => IconUpdate::Keep,
    }
}
