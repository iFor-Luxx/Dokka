use dokka_contracts::{ContractError, WindowSource};
use dokka_model::OpenWindow;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
use windows::core::BOOL;

use crate::describe_window::describe_window;
use crate::filter_window::belongs_in_dock;

/// Ventanas del escritorio actual que merecen un icono en el dock.
#[derive(Debug, Default)]
pub struct SystemWindows;

impl WindowSource for SystemWindows {
    fn open_windows(&self) -> Result<Vec<OpenWindow>, ContractError> {
        let mut handles: Vec<HWND> = Vec::new();
        // SAFETY: `handles` vive durante toda la enumeración, que es síncrona.
        unsafe { EnumWindows(Some(collect), LPARAM(&raw mut handles as isize)) }
            .map_err(|error| ContractError::Failed(error.to_string()))?;

        Ok(handles
            .into_iter()
            .filter(|&window| belongs_in_dock(window))
            .filter_map(describe_window)
            .collect())
    }
}

unsafe extern "system" fn collect(window: HWND, data: LPARAM) -> BOOL {
    // SAFETY: `data` es el puntero al vector que entregó `open_windows`.
    let handles = unsafe { &mut *(data.0 as *mut Vec<HWND>) };
    handles.push(window);
    true.into()
}
