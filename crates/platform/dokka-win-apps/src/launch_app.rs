use dokka_contracts::{AppLauncher, ContractError};
use dokka_model::AppInfo;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, PCWSTR, w};

/// Por debajo de este valor, `ShellExecuteW` informa un error.
const FIRST_SUCCESS_VALUE: usize = 33;

/// Abre apps como lo haría el Explorador.
#[derive(Debug, Default)]
pub struct SystemLauncher;

impl AppLauncher for SystemLauncher {
    fn launch(&self, app: &AppInfo) -> Result<(), ContractError> {
        let path = HSTRING::from(app.executable.as_os_str());
        // SAFETY: las cadenas viven durante toda la llamada.
        let result = unsafe {
            ShellExecuteW(
                None,
                w!("open"),
                &path,
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOWNORMAL,
            )
        };
        if result.0 as usize >= FIRST_SUCCESS_VALUE {
            Ok(())
        } else {
            let reason = format!("no se pudo abrir {}", app.name);
            Err(ContractError::Failed(reason))
        }
    }
}
