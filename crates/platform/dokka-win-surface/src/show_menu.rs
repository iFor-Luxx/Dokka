use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, DestroyMenu, GetCursorPos, GetForegroundWindow, MF_STRING,
    SetForegroundWindow, TPM_BOTTOMALIGN, TPM_CENTERALIGN, TPM_RETURNCMD, TrackPopupMenu,
};
use windows::core::HSTRING;

use crate::{Surface, SurfaceError};

impl Surface {
    /// Muestra un menú de Windows sobre el cursor; devuelve la opción elegida.
    pub fn show_menu(&self, options: &[&str]) -> Result<Option<usize>, SurfaceError> {
        let mut cursor = POINT::default();
        // SAFETY: la ventana es propia y el menú se destruye antes de salir.
        // Hay que tomar el foco para que el menú se cierre al pulsar fuera; después se devuelve.
        let chosen = unsafe {
            GetCursorPos(&mut cursor)?;
            let menu = CreatePopupMenu()?;
            for (index, option) in options.iter().enumerate() {
                AppendMenuW(menu, MF_STRING, index + 1, &HSTRING::from(*option))?;
            }
            let previous = GetForegroundWindow();
            let _ = SetForegroundWindow(self.window);
            let chosen = TrackPopupMenu(
                menu,
                TPM_RETURNCMD | TPM_BOTTOMALIGN | TPM_CENTERALIGN,
                cursor.x,
                cursor.y,
                None,
                self.window,
                None,
            );
            let _ = SetForegroundWindow(previous);
            DestroyMenu(menu)?;
            chosen.0
        };
        // Cero significa que se cerró sin elegir.
        Ok((chosen as usize).checked_sub(1))
    }
}
