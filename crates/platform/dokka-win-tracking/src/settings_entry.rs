//! Ventana propia de Preferencias como app del dock.
//!
//! El seguimiento general excluye el proceso propio para no listarse a sí
//! mismo; aquí se incluye SOLO Preferencias, y solo minimizada, para que vaya
//! a la derecha del separador como cualquier ventana minimizada. Nunca dock,
//! barra, paneles, vista grande, Launchpad ni bandeja oculta.

use dokka_model::{AppId, AppInfo, AppWindow, OpenWindow, WindowId};
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextW, IsIconic, IsWindow};

/// Identidad estable de Preferencias en el dock. No es un ejecutable real:
/// así no se puede fijar ni lanzar otra instancia y el icono lo pone el dock.
pub const SETTINGS_APP_ID: &str = "dokka-preferencias";

/// Nombre que muestra el dock y la vista previa.
pub const SETTINGS_APP_NAME: &str = "Preferencias";

/// Describe la ventana de Preferencias si es la esperada y está minimizada.
/// `None` en cualquier otro caso: visible, cerrada u otra ventana propia.
pub(crate) fn describe_settings_window(window: HWND) -> Option<OpenWindow> {
    // SAFETY: consultas de solo lectura; un handle destruido devuelve cero.
    unsafe {
        if !IsWindow(Some(window)).as_bool() {
            return None;
        }
        if !IsIconic(window).as_bool() {
            return None;
        }
    }
    let mut title = [0_u16; 128];
    // SAFETY: Windows no escribe más allá del tamaño del búfer entregado.
    let length = unsafe { GetWindowTextW(window, &mut title) };
    let title = String::from_utf16_lossy(&title[..length.max(0) as usize]);
    if title != SETTINGS_APP_NAME {
        return None;
    }
    Some(OpenWindow {
        app: AppInfo {
            id: AppId::new(SETTINGS_APP_ID),
            name: SETTINGS_APP_NAME.to_owned(),
            executable: PathBuf::from(SETTINGS_APP_ID),
        },
        window: AppWindow {
            id: WindowId::new(window.0 as u64),
            title,
            is_minimized: true,
        },
    })
}
