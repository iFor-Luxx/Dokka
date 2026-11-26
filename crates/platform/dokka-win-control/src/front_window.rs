use dokka_model::WindowId;
use windows::Win32::UI::WindowsAndMessaging::{GW_HWNDNEXT, GetTopWindow, GetWindow};

/// La primera de `windows` en el orden Z de Windows, si alguna sigue abierta.
///
/// Recorre de arriba abajo con `GetTopWindow`/`GetWindow` y devuelve la
/// primera que esté en la lista, sin filtrar por visibilidad: solo el orden.
/// `None` si la lista está vacía o ninguna sigue en el orden Z.
pub(crate) fn front_of(windows: &[WindowId]) -> Option<WindowId> {
    if windows.is_empty() {
        return None;
    }
    // SAFETY: solo recorre el orden Z; un identificador destruido
    // simplemente no coincide y la cadena termina sola.
    unsafe {
        let mut current = GetTopWindow(None).ok()?;
        loop {
            let id = WindowId::new(current.0 as u64);
            if windows.contains(&id) {
                return Some(id);
            }
            current = GetWindow(current, GW_HWNDNEXT).ok()?;
        }
    }
}
