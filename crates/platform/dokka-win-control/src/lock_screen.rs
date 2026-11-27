use windows::Win32::System::Shutdown::LockWorkStation;

/// Bloquea la sesión de Windows (pide la contraseña al volver).
pub fn lock_screen() {
    // SAFETY: no recibe argumentos; el fallo se ignora como en `set_taskbar_visible`.
    unsafe {
        let _ = LockWorkStation();
    }
}
