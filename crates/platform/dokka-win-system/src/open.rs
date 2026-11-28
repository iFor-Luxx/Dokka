//! Apertura de páginas del sistema desde "Acerca de este PC".
//!
//! Solo abre; no cambia nada. Falla sola: si no se abre, devuelve error.

use dokka_contracts::{ContractError, SystemPage};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, PCWSTR, w};

/// Dirección que se abre para cada botón.
pub(crate) fn address(page: SystemPage) -> &'static str {
    match page {
        SystemPage::Display => "ms-settings:display",
        SystemPage::Storage => "ms-settings:storagesense",
        SystemPage::Report => "msinfo32",
        SystemPage::DateTime => "ms-settings:dateandtime",
        SystemPage::Region => "ms-settings:regionformatting",
    }
}

/// Abre la dirección con el Explorador, como hace `SystemLauncher`.
pub(crate) fn open_address(page: SystemPage) -> Result<(), ContractError> {
    let target = HSTRING::from(address(page));
    // SAFETY: las cadenas viven durante toda la llamada.
    let result = unsafe {
        ShellExecuteW(
            None,
            w!("open"),
            &target,
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };
    if result.0 as usize >= 33 {
        Ok(())
    } else {
        Err(ContractError::Failed(
            "no se pudo abrir la página".to_owned(),
        ))
    }
}
