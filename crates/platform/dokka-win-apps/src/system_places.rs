use std::path::PathBuf;

use dokka_model::{AppId, AppInfo};
use windows::Win32::UI::Shell::{SHQUERYRBINFO, SHQueryRecycleBinW};
use windows::core::PCWSTR;

/// El explorador de archivos de Windows.
pub fn explorer_app() -> AppInfo {
    let windows_folder = std::env::var_os("WINDIR").unwrap_or_else(|| "C:\\Windows".into());
    let executable = PathBuf::from(windows_folder).join("explorer.exe");
    AppInfo {
        id: AppId::from_executable(&executable),
        name: "Explorador".to_owned(),
        executable,
    }
}

/// Lugar de Windows que se abre por dirección, como `ms-clock:`.
pub fn system_place(name: &str, address: &str) -> AppInfo {
    let executable = PathBuf::from(address);
    AppInfo {
        id: AppId::from_executable(&executable),
        name: name.to_owned(),
        executable,
    }
}

/// `true` si alguna unidad tiene algo en la papelera.
pub fn recycle_bin_has_items() -> bool {
    let mut info = SHQUERYRBINFO {
        cbSize: size_of::<SHQUERYRBINFO>() as u32,
        ..Default::default()
    };
    // SAFETY: `info` declara su tamaño y es escribible.
    let result = unsafe { SHQueryRecycleBinW(PCWSTR::null(), &mut info) };
    let items = info.i64NumItems;
    result.is_ok() && items > 0
}
