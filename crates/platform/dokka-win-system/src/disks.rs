//! Otros discos fijos y peso de la papelera.
//!
//! Los otros discos salen de las letras fijas (`DRIVE_FIXED`) distintas del
//! disco del sistema. La papelera sale de `SHQueryRecycleBinW`: `None` cuando
//! está vacía o no se lee. Nunca se registra nada.

use dokka_model::OtherDisk;
use windows::Win32::Storage::FileSystem::{
    DRIVE_FIXED, GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDriveStringsW,
};
use windows::Win32::UI::Shell::{SHQUERYRBINFO, SHQueryRecycleBinW};
use windows::core::{HSTRING, PCWSTR};

/// Discos fijos que no son el del sistema, con libres y total.
pub(crate) fn other_disks(system_drive: Option<&str>) -> Vec<OtherDisk> {
    let mut buffer = [0_u16; 128];
    // SAFETY: el buffer es escribible y declara su tamaño.
    let length = unsafe { GetLogicalDriveStringsW(Some(&mut buffer)) };
    if length == 0 {
        return Vec::new();
    }
    let end = (length as usize).min(buffer.len());
    let mut disks = Vec::new();
    for root in split_roots(&buffer[..end]) {
        let drive = root.get(..2).unwrap_or("").to_owned();
        if drive.is_empty() || Some(drive.as_str()) == system_drive {
            continue;
        }
        if !is_fixed(&root) {
            continue;
        }
        if let Some((free, total)) = drive_space(&root) {
            disks.push(OtherDisk {
                drive,
                free_bytes: free,
                total_bytes: total,
            });
        }
    }
    disks.sort_by(|left, right| left.drive.cmp(&right.drive));
    disks
}

/// Peso en bytes de la papelera si tiene elementos.
pub(crate) fn recycle_size() -> Option<u64> {
    let mut info = SHQUERYRBINFO {
        cbSize: size_of::<SHQUERYRBINFO>() as u32,
        ..Default::default()
    };
    // SAFETY: `info` declara su tamaño y es escribible.
    let result = unsafe { SHQueryRecycleBinW(PCWSTR::null(), &mut info) };
    if result.is_err() || info.i64NumItems <= 0 {
        return None;
    }
    u64::try_from(info.i64Size).ok()
}

/// Libres y total de una raíz como `C:\`.
fn drive_space(root: &str) -> Option<(u64, u64)> {
    let name = HSTRING::from(root);
    let mut free: u64 = 0;
    let mut total: u64 = 0;
    // SAFETY: dos salidas `u64` válidas; la tercera no se pide.
    let ok = unsafe { GetDiskFreeSpaceExW(&name, Some(&mut free), Some(&mut total), None) };
    if ok.is_err() || total == 0 {
        return None;
    }
    Some((free, total))
}

/// `true` si la raíz es un disco fijo local.
fn is_fixed(root: &str) -> bool {
    let name = HSTRING::from(root);
    // SAFETY: la cadena vive durante la llamada; solo lee el tipo.
    unsafe { GetDriveTypeW(&name) == DRIVE_FIXED }
}

/// Corta la lista de raíces (`C:\` + nulo, una tras otra) en cada raíz.
fn split_roots(buffer: &[u16]) -> Vec<String> {
    let mut roots = Vec::new();
    let mut start = 0;
    for (index, unit) in buffer.iter().enumerate() {
        if *unit != 0 {
            continue;
        }
        if index > start {
            let text = String::from_utf16_lossy(&buffer[start..index]);
            if !text.trim().is_empty() {
                roots.push(text);
            }
        }
        start = index + 1;
    }
    roots
}
