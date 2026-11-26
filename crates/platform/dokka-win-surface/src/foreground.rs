use windows::Win32::Foundation::HWND;
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, SetForegroundWindow,
};

/// Lleva la ventana al frente aunque Dokka esté en segundo plano.
///
/// El clic en el dock no activa a Dokka (su ventana no se activa), así que
/// Windows suele negar el primer plano al hilo en segundo plano: se une la
/// entrada al hilo de la ventana activa, se toma el primer plano y se desune.
pub(crate) fn foreground_window(window: HWND) {
    // SAFETY: consulta y cambio de foco de ventanas del mismo escritorio.
    unsafe {
        let current = GetForegroundWindow();
        if current == window {
            return;
        }
        let foreground_thread = GetWindowThreadProcessId(current, None);
        let own_thread = GetCurrentThreadId();
        let attached = foreground_thread != 0
            && foreground_thread != own_thread
            && AttachThreadInput(foreground_thread, own_thread, true).as_bool();
        let _ = SetForegroundWindow(window);
        if attached {
            let _ = AttachThreadInput(foreground_thread, own_thread, false);
        }
    }
}
