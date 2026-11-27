use dokka_contracts::{ContractError, WindowSource};
use dokka_model::OpenWindow;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
use windows::core::BOOL;

use crate::describe_window::describe_window;
use crate::filter_window::belongs_in_dock;
use crate::settings_entry::describe_settings_window;

/// Ventanas del escritorio actual que merecen un icono en el dock.
#[derive(Debug, Default)]
pub struct SystemWindows {
    /// Ventana propia de Preferencias, si ya se conoce. Solo ella se incluye
    /// del proceso propio, y solo minimizada: el resto (dock, barra, paneles,
    /// vista grande, Launchpad) nunca va al dock.
    settings: Option<HWND>,
}

impl SystemWindows {
    /// Recuerda la ventana propia de Preferencias para incluirla minimizada.
    /// Sin ella, el seguimiento sigue como antes (solo apps externas).
    pub fn set_settings_window(&mut self, window: HWND) {
        self.settings = Some(window);
    }

    fn settings_entry(&self) -> Option<OpenWindow> {
        let window = self.settings?;
        describe_settings_window(window)
    }
}

impl WindowSource for SystemWindows {
    fn open_windows(&self) -> Result<Vec<OpenWindow>, ContractError> {
        let mut handles: Vec<HWND> = Vec::new();
        // SAFETY: `handles` vive durante toda la enumeración, que es síncrona.
        unsafe { EnumWindows(Some(collect), LPARAM(&raw mut handles as isize)) }
            .map_err(|error| ContractError::Failed(error.to_string()))?;

        let mut open: Vec<OpenWindow> = handles
            .into_iter()
            .filter(|&window| belongs_in_dock(window))
            .filter_map(describe_window)
            .collect();
        // Preferencias minimizada va a la derecha del separador, como
        // cualquier ventana minimizada. No cuenta como app que ocupa la
        // pantalla: solo se incluye minimizada.
        if let Some(entry) = self.settings_entry() {
            open.push(entry);
        }
        Ok(open)
    }
}

unsafe extern "system" fn collect(window: HWND, data: LPARAM) -> BOOL {
    // SAFETY: `data` es el puntero al vector que entregó `open_windows`.
    let handles = unsafe { &mut *(data.0 as *mut Vec<HWND>) };
    handles.push(window);
    true.into()
}
