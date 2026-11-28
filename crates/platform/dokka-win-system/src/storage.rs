//! Disco del sistema: letra, libres y total.
//!
//! Sale de la carpeta de Windows y de `GetDiskFreeSpaceExW`.
//! Si no se lee, los tres quedan en `None`.

use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
use windows::Win32::System::SystemInformation::GetWindowsDirectoryW;
use windows::core::HSTRING;

/// Letra del disco del sistema ("C:"), libres y total en bytes.
pub(crate) fn system_drive() -> (Option<String>, Option<u64>, Option<u64>) {
    let mut buffer = [0_u16; 260];
    // SAFETY: el buffer es escribible y declara su tamaño por el corte.
    let length = unsafe { GetWindowsDirectoryW(Some(&mut buffer)) };
    if length == 0 || length as usize > buffer.len() {
        return (None, None, None);
    }
    let path = String::from_utf16_lossy(&buffer[..length as usize]);
    let drive = path.get(..2).unwrap_or("C:").to_owned();
    let root = HSTRING::from(format!("{drive}\\"));
    let mut free: u64 = 0;
    let mut total: u64 = 0;
    // SAFETY: dos salidas `u64` válidas; la tercera no se pide.
    let ok = unsafe { GetDiskFreeSpaceExW(&root, Some(&mut free), Some(&mut total), None) };
    if ok.is_err() || total == 0 {
        return (Some(drive), None, None);
    }
    (Some(drive), Some(free), Some(total))
}
