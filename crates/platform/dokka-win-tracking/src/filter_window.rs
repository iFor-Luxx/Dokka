use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::{DWMWA_CLOAKED, DwmGetWindowAttribute};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{
    GW_OWNER, GWL_EXSTYLE, GetClassNameW, GetWindow, GetWindowLongPtrW, GetWindowTextLengthW,
    GetWindowThreadProcessId, IsWindowVisible, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
};

/// Mismo criterio general que usa la barra de tareas.
pub(crate) fn belongs_in_dock(window: HWND) -> bool {
    // SAFETY: son consultas de solo lectura; un handle ya destruido devuelve cero.
    unsafe {
        if !IsWindowVisible(window).as_bool() || GetWindowTextLengthW(window) == 0 {
            return false;
        }
        // Las ventanas del propio Dokka (como Preferencias) no son apps abiertas.
        let mut owner_process = 0;
        GetWindowThreadProcessId(window, Some(&mut owner_process));
        if owner_process == GetCurrentProcessId() {
            return false;
        }
        let style = GetWindowLongPtrW(window, GWL_EXSTYLE) as u32;
        let forced = style & WS_EX_APPWINDOW.0 != 0;
        let has_owner = GetWindow(window, GW_OWNER).is_ok_and(|owner| !owner.is_invalid());
        if !forced && (has_owner || style & WS_EX_TOOLWINDOW.0 != 0) {
            return false;
        }
    }
    !is_cloaked(window) && !is_desktop(window)
}

/// Las ventanas ocultas por Windows (otro escritorio virtual, apps suspendidas).
fn is_cloaked(window: HWND) -> bool {
    let mut cloaked = 0_u32;
    // SAFETY: `cloaked` es escribible y se declara su tamaño exacto.
    let result = unsafe {
        DwmGetWindowAttribute(
            window,
            DWMWA_CLOAKED,
            (&raw mut cloaked).cast(),
            size_of::<u32>() as u32,
        )
    };
    result.is_ok() && cloaked != 0
}

fn is_desktop(window: HWND) -> bool {
    let mut name = [0_u16; 64];
    // SAFETY: Windows no escribe más allá del tamaño del búfer entregado.
    let length = unsafe { GetClassNameW(window, &mut name) };
    let class = String::from_utf16_lossy(&name[..length.max(0) as usize]);
    class == "Progman" || class == "WorkerW"
}
