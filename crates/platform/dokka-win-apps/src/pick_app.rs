//! Selector de aplicaciones: abre el diálogo de Windows en un hilo aparte.
//!
//! El hilo de la interfaz no puede esperar al diálogo modal, así que el diálogo
//! vive en su propio hilo y el resultado se recoge sin bloquear con `chosen`.

use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError, channel};
use std::thread;

use dokka_model::{AppId, AppInfo};
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoCreateInstance, CoInitializeEx,
    CoTaskMemFree, CoUninitialize,
};
use windows::Win32::UI::Shell::Common::COMDLG_FILTERSPEC;
use windows::Win32::UI::Shell::{
    FOS_FILEMUSTEXIST, FOS_FORCEFILESYSTEM, FOS_PATHMUSTEXIST, FileOpenDialog, IFileOpenDialog,
    SIGDN_FILESYSPATH,
};
use windows::core::w;

/// Pide una aplicación con el diálogo de Windows y entrega la elegida.
///
/// Solo puede haber un diálogo abierto: si ya hay uno, `open` no hace nada.
/// Cancelar no es un error: `chosen` devuelve `None` como si aún no hubiera nada.
#[derive(Default)]
pub struct AppPicker {
    waiting: Option<Receiver<Option<AppInfo>>>,
}

impl AppPicker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abre el diálogo en un hilo aparte. No bloquea.
    pub fn open(&mut self) {
        if self.waiting.is_some() {
            return;
        }
        let (sender, waiting) = channel();
        self.waiting = Some(waiting);
        thread::spawn(move || {
            // Si el shell ya no escucha, la elección simplemente se pierde.
            let _ = sender.send(pick_once());
        });
    }

    /// La aplicación elegida, si el diálogo ya terminó. No espera.
    pub fn chosen(&mut self) -> Option<AppInfo> {
        let waiting = self.waiting.as_ref()?;
        match waiting.try_recv() {
            Ok(choice) => {
                self.waiting = None;
                choice
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                self.waiting = None;
                None
            }
        }
    }
}

/// Libera COM al salir del hilo del diálogo.
struct ComGuard;

impl Drop for ComGuard {
    fn drop(&mut self) {
        // SAFETY: este hilo inicializó COM y sigue vivo.
        unsafe { CoUninitialize() };
    }
}

/// Muestra el diálogo y convierte la elección en `AppInfo`.
/// `None` cuando se cancela o Windows no responde.
fn pick_once() -> Option<AppInfo> {
    // SAFETY: inicializa COM solo para este hilo, que termina con el diálogo.
    if unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) }.is_err() {
        return None;
    }
    let _com = ComGuard;
    // SAFETY: crea el diálogo del sistema en este hilo con COM ya iniciado.
    let dialog: IFileOpenDialog =
        unsafe { CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL) }.ok()?;
    let filters = [
        COMDLG_FILTERSPEC {
            pszName: w!("Aplicaciones"),
            pszSpec: w!("*.exe;*.lnk"),
        },
        COMDLG_FILTERSPEC {
            pszName: w!("Todos"),
            pszSpec: w!("*.*"),
        },
    ];
    // SAFETY: el diálogo vive en este hilo con COM ya iniciado.
    unsafe { dialog.SetFileTypes(&filters) }.ok()?;
    // Sin FOS_NODEREFERENCELINKS: los accesos directos se resuelven a su destino.
    // SAFETY: el diálogo vive en este hilo con COM ya iniciado.
    unsafe { dialog.SetOptions(FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST) }
        .ok()?;
    // SAFETY: el diálogo vive en este hilo y muestra su propio bucle modal.
    if unsafe { dialog.Show(None) }.is_err() {
        // Cancelar también llega como error; en ambos casos no hay elección.
        return None;
    }
    // SAFETY: el diálogo sigue abierto en este hilo y ya tiene resultado.
    let item = unsafe { dialog.GetResult() }.ok()?;
    // SAFETY: `item` es el resultado recién obtenido del diálogo.
    let display = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH) }.ok()?;
    // SAFETY: `display` es la cadena que reservó el diálogo; se libera siempre,
    // haya funcionado o no la copia.
    let text = unsafe {
        let copy = display.to_string();
        CoTaskMemFree(Some(display.0 as *const c_void));
        copy
    }
    .ok()?;
    let executable = PathBuf::from(text);
    let name = executable
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .filter(|name| !name.is_empty())?;
    Some(AppInfo {
        id: AppId::from_executable(&executable),
        name,
        executable,
    })
}
