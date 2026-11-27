use dokka_model::{AppId, AppInfo};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use crate::find_executable::find_executable;

/// App de la ventana activa (`name` puede ser el nombre del ejecutable).
/// `None` si la activa es de Dokka o no se puede saber; el escritorio cuenta
/// como el Explorador (su ventana pertenece a `explorer.exe`).
pub fn foreground_app() -> Option<AppInfo> {
    // SAFETY: consulta sin argumentos; el handle se valida antes de usarse.
    let window = unsafe { GetForegroundWindow() };
    if window.is_invalid() {
        return None;
    }
    let mut owner = 0;
    // SAFETY: `owner` es escribible durante la llamada.
    unsafe { GetWindowThreadProcessId(window, Some(&mut owner)) };
    // SAFETY: solo consulta el identificador del proceso actual.
    if owner == unsafe { GetCurrentProcessId() } {
        return None;
    }
    let executable = find_executable(window)?;
    let name = executable
        .file_stem()?
        .to_string_lossy()
        .into_owned();
    if name.is_empty() {
        return None;
    }
    Some(AppInfo {
        id: AppId::from_executable(&executable),
        name,
        executable,
    })
}
