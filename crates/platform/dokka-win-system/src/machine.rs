//! Nombre del equipo, procesador, memoria y edición de Windows.
//!
//! Todo devuelve `Option`: si algo no se lee, queda en `None`.

use windows::Win32::System::SystemInformation::{
    ComputerNamePhysicalDnsHostname, GetComputerNameExW, GlobalMemoryStatusEx, MEMORYSTATUSEX,
};
use windows::core::PWSTR;

use super::placeholders::clean;
use super::registry::{read_registry_dword, read_registry_string};

/// Nombre del equipo en la red, sin dominio.
pub(crate) fn computer_name() -> Option<String> {
    let mut size: u32 = 0;
    // SAFETY: con buffer vacío solo pide el tamaño necesario.
    let _ = unsafe { GetComputerNameExW(ComputerNamePhysicalDnsHostname, None, &mut size) };
    if size == 0 || size > 256 {
        return None;
    }
    let mut buffer = vec![0_u16; size as usize];
    // SAFETY: el buffer reserva `size` unidades recién pedidas.
    let ok = unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            Some(PWSTR(buffer.as_mut_ptr())),
            &mut size,
        )
    };
    if ok.is_err() {
        return None;
    }
    let used = (size as usize).min(buffer.len());
    let text = String::from_utf16_lossy(&buffer[..used]);
    clean(Some(text))
}

/// Procesador tal como lo anuncia el registro.
pub(crate) fn processor() -> Option<String> {
    clean(read_registry_string(
        r"HARDWARE\DESCRIPTION\System\CentralProcessor\0",
        "ProcessorNameString",
    ))
}

/// Memoria física instalada en bytes.
pub(crate) fn memory_bytes() -> Option<u64> {
    let mut status = MEMORYSTATUSEX {
        dwLength: size_of::<MEMORYSTATUSEX>() as u32,
        dwMemoryLoad: 0,
        ullTotalPhys: 0,
        ullAvailPhys: 0,
        ullTotalPageFile: 0,
        ullAvailPageFile: 0,
        ullTotalVirtual: 0,
        ullAvailVirtual: 0,
        ullAvailExtendedVirtual: 0,
    };
    // SAFETY: `status` declara su tamaño y es escribible.
    let ok = unsafe { GlobalMemoryStatusEx(&mut status) };
    if ok.is_err() || status.ullTotalPhys == 0 {
        return None;
    }
    Some(status.ullTotalPhys)
}

/// Fabricante y modelo del registro del BIOS.
pub(crate) fn bios_model() -> (Option<String>, Option<String>) {
    let path = r"HARDWARE\DESCRIPTION\System\BIOS";
    let manufacturer = clean(read_registry_string(path, "SystemManufacturer"));
    let product = clean(read_registry_string(path, "SystemProductName"));
    (manufacturer, product)
}

/// Edición ("Windows 11 Pro") y versión ("24H2 (compilación ...)").
pub(crate) fn windows_edition_and_version() -> (Option<String>, Option<String>) {
    let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
    let product = clean(read_registry_string(path, "ProductName"));
    let display = clean(read_registry_string(path, "DisplayVersion"));
    let build = read_registry_string(path, "CurrentBuildNumber");
    let ubr = read_registry_dword(path, "UBR");
    let build_number: u32 = build.as_deref().and_then(|text| text.parse().ok()).unwrap_or(0);

    let edition = product.map(|name| {
        // El registro aún dice 10 en Windows 11: a partir de la 22000 es 11.
        if build_number >= 22000 && name.contains("Windows 10") {
            name.replace("Windows 10", "Windows 11")
        } else {
            name
        }
    });

    let version = match (display, build) {
        (Some(release), Some(build)) => {
            let full = match ubr {
                Some(patch) => format!("{build}.{patch}"),
                None => build,
            };
            Some(format!("{release} (compilación {full})"))
        }
        (Some(release), None) => Some(release),
        (None, Some(build)) => {
            let full = match ubr {
                Some(patch) => format!("{build}.{patch}"),
                None => build,
            };
            Some(format!("Compilación {full}"))
        }
        (None, None) => None,
    };
    (edition, version)
}
