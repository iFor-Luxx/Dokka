//! Entrada de desinstalación en `HKLM\...\Uninstall\Dokka`.
//!
//! Solo esa clave: lo que muestra "Agregar o quitar programas" y el comando
//! que ejecuta `Desinstalar.exe --uninstall`. Al escribirla se guarda lo que
//! hubiera para devolverlo si la instalación falla.

use std::path::Path;

use dokka_contracts::InstallFailure;
use windows::Win32::System::Registry::HKEY_LOCAL_MACHINE;

use crate::registry::{
    OpenKey, delete_tree, read_dword_value, read_string_value, write_dword_value,
    write_string_value,
};

/// Clave de la entrada de Dokka.
const UNINSTALL_SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Dokka";

/// Nombres de los textos de la entrada.
const STRING_NAMES: [&str; 6] = [
    "DisplayName",
    "DisplayIcon",
    "DisplayVersion",
    "Publisher",
    "InstallLocation",
    "UninstallString",
];

/// Nombres de los números de la entrada.
const DWORD_NAMES: [&str; 2] = ["NoModify", "NoRepair"];

/// Lo que había en la entrada antes de escribirla.
pub(crate) struct UninstallBackup {
    existed: bool,
    strings: Vec<(String, String)>,
    dwords: Vec<(String, u32)>,
}

/// Escribe la entrada de desinstalación. Guarda lo anterior.
pub(crate) fn write_uninstall_entry(
    dir: &Path,
    exe: &Path,
    uninstaller: &Path,
) -> Result<UninstallBackup, InstallFailure> {
    let backup = read_uninstall_entry();
    let key = OpenKey::create(HKEY_LOCAL_MACHINE, UNINSTALL_SUBKEY)?;
    let handle = key.handle();
    write_string_value(handle, "DisplayName", "Dokka")?;
    write_string_value(handle, "DisplayIcon", &path_text(exe)?)?;
    write_string_value(handle, "DisplayVersion", env!("CARGO_PKG_VERSION"))?;
    write_string_value(handle, "Publisher", "Dokka")?;
    write_string_value(handle, "InstallLocation", &path_text(dir)?)?;
    let quoted = quoted_path(uninstaller)?;
    write_string_value(handle, "UninstallString", &format!("{quoted} --uninstall"))?;
    write_dword_value(handle, "NoModify", 1)?;
    write_dword_value(handle, "NoRepair", 1)?;
    Ok(backup)
}

/// Quita la entrada de desinstalación. Si no existe, ya está.
pub(crate) fn remove_uninstall_entry() -> Result<(), InstallFailure> {
    delete_tree(HKEY_LOCAL_MACHINE, UNINSTALL_SUBKEY)
}

/// Devuelve la entrada a como estaba: lo anterior o nada.
pub(crate) fn restore_uninstall_entry(backup: &UninstallBackup) {
    if !backup.existed {
        let _ = delete_tree(HKEY_LOCAL_MACHINE, UNINSTALL_SUBKEY);
        return;
    }
    if let Ok(key) = OpenKey::create(HKEY_LOCAL_MACHINE, UNINSTALL_SUBKEY) {
        for (name, value) in &backup.strings {
            let _ = write_string_value(key.handle(), name, value);
        }
        for (name, value) in &backup.dwords {
            let _ = write_dword_value(key.handle(), name, *value);
        }
    }
}

/// Lee la entrada actual para poder devolverla.
fn read_uninstall_entry() -> UninstallBackup {
    let existed = OpenKey::open(
        HKEY_LOCAL_MACHINE,
        UNINSTALL_SUBKEY,
        windows::Win32::System::Registry::KEY_READ,
    )
    .is_ok();
    let mut strings = Vec::new();
    for name in STRING_NAMES {
        if let Some(value) = read_string_value(HKEY_LOCAL_MACHINE, UNINSTALL_SUBKEY, name) {
            strings.push((name.to_owned(), value));
        }
    }
    let mut dwords = Vec::new();
    for name in DWORD_NAMES {
        if let Some(value) = read_dword_value(HKEY_LOCAL_MACHINE, UNINSTALL_SUBKEY, name) {
            dwords.push((name.to_owned(), value));
        }
    }
    UninstallBackup {
        existed,
        strings,
        dwords,
    }
}

/// Ruta como texto para el registro.
fn path_text(path: &Path) -> Result<String, InstallFailure> {
    path.as_os_str().to_str().map(str::to_owned).ok_or_else(|| {
        InstallFailure::Other("la ruta de Dokka no se puede escribir en el registro".to_owned())
    })
}

/// Ruta entre comillas para `UninstallString`.
fn quoted_path(path: &Path) -> Result<String, InstallFailure> {
    path_text(path).map(|text| format!("\"{text}\""))
}
