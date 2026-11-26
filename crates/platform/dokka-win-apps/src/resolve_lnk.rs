use std::path::Path;

use windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW;
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, IPersistFile, STGM_READ,
};
use windows::Win32::UI::Shell::{IShellLinkW, SLGP_UNCPRIORITY, ShellLink};
use windows::core::{HSTRING, Interface};

/// Destino de un `.lnk`, para filtrar por él sin perder sus argumentos.
pub(crate) fn resolve_lnk(link: &Path) -> Option<std::path::PathBuf> {
    // SAFETY: inicializa COM solo para este hilo; si ya estaba iniciado no cambia nada.
    let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    // SAFETY: crea el enlace en este hilo con COM ya iniciado.
    let shell: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_ALL) }.ok()?;
    let persist: IPersistFile = shell.cast().ok()?;
    let path = HSTRING::from(link.as_os_str());
    // SAFETY: `path` vive durante la llamada y solo se lee el archivo.
    unsafe { persist.Load(&path, STGM_READ) }.ok()?;
    let mut buffer = [0_u16; 260];
    let mut data = WIN32_FIND_DATAW::default();
    // SAFETY: `buffer` tiene 260 elementos y `data` es escribible.
    unsafe {
        shell
            .GetPath(&mut buffer, &mut data, SLGP_UNCPRIORITY.0 as u32)
            .ok()?;
    }
    let mut end = buffer.len();
    for (at, &unit) in buffer.iter().enumerate() {
        if unit == 0 {
            end = at;
            break;
        }
    }
    let target = String::from_utf16_lossy(&buffer[..end]);
    let target = target.trim();
    (!target.is_empty()).then(|| std::path::PathBuf::from(target))
}
