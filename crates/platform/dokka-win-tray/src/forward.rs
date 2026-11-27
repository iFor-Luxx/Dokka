//! Reenvío a la barra real de Windows.
//!
//! Todo lo dirigido a la bandeja también lo atiende el Explorador: se le
//! reenvía con espera corta y se devuelve su respuesta, así Windows sigue
//! funcionando igual con nuestra ventana por delante.

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, GetWindowThreadProcessId, SMTO_ABORTIFHUNG, SendMessageTimeoutW,
};
use windows::core::w;

/// Espera máxima del reenvío; con una app colgada no se bloquea la bandeja.
const FORWARD_TIMEOUT_MS: u32 = 2000;

/// La barra del Explorador: mismo nombre, de otro proceso y con su zona de avisos
/// (`TrayNotifyWnd`). Así no se reenvía a otra barra alternativa, que podría
/// devolvernos el mensaje. `None` si no hay barra real.
pub(crate) fn real_tray() -> Option<HWND> {
    // SAFETY: buscar ventanas del shell y leer su proceso no cambia nada.
    unsafe {
        let own = GetCurrentProcessId();
        let mut after: Option<HWND> = None;
        while let Ok(found) = FindWindowExW(None, after, w!("Shell_TrayWnd"), None) {
            if found.is_invalid() {
                return None;
            }
            let mut owner = 0;
            GetWindowThreadProcessId(found, Some(&mut owner));
            if owner != own && FindWindowExW(Some(found), None, w!("TrayNotifyWnd"), None).is_ok() {
                return Some(found);
            }
            after = Some(found);
        }
        None
    }
}

/// Reenvía el mensaje a la barra real; `None` si no hay a quién reenviar.
pub(crate) fn forward(message: u32, wparam: WPARAM, lparam: LPARAM) -> Option<LRESULT> {
    let target = real_tray()?;
    // SAFETY: espera corta y sin colgarse; el resultado lo pone Windows.
    unsafe {
        let mut result = 0_usize;
        SendMessageTimeoutW(
            target,
            message,
            wparam,
            lparam,
            SMTO_ABORTIFHUNG,
            FORWARD_TIMEOUT_MS,
            Some(&mut result as *mut usize),
        );
        Some(LRESULT(result as isize))
    }
}
