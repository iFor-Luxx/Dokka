//! Puerta pública: escucha la bandeja mientras vive.
//!
//! Al soltarlo se destruye la ventana y termina el hilo; Windows vuelve a
//! recibirlo todo directo. Si algo falla, `start` devuelve error o la
//! bandeja queda vacía: Dokka sigue funcionando.

use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use dokka_contracts::{ContractError, SystemTray};
use dokka_model::{TrayItem, TrayItemId};
use dokka_scene::Bitmap;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_CLOSE};

use crate::{click, state, window};

/// Escucha la bandeja mientras vive; al soltarlo Windows la atiende solo.
#[derive(Debug)]
pub struct TrayHost {
    shared: Arc<Mutex<state::Shared>>,
    worker: Option<Worker>,
}

#[derive(Debug)]
struct Worker {
    thread: Option<JoinHandle<()>>,
    window_raw: usize,
}

impl TrayHost {
    pub fn start() -> Result<Self, ContractError> {
        let shared = Arc::new(Mutex::new(state::Shared::new()));
        match window::spawn(shared.clone()) {
            Ok((thread, window)) => Ok(Self {
                shared,
                worker: Some(Worker {
                    thread: Some(thread),
                    window_raw: window.0 as usize,
                }),
            }),
            Err(reason) => Err(ContractError::Failed(reason)),
        }
    }

    /// Cambia cada vez que cambian los iconos, sus textos o sus imágenes.
    pub fn revision(&self) -> u64 {
        self.shared
            .lock()
            .ok()
            .map(|state| state.revision())
            .unwrap_or(0)
    }

    /// Imagen del icono; el mismo `Arc` mientras no cambie.
    pub fn icon(&self, id: TrayItemId) -> Option<Arc<Bitmap>> {
        self.shared
            .lock()
            .ok()
            .and_then(|state| state.image_for(id))
    }
}

impl SystemTray for TrayHost {
    fn items(&self) -> Vec<TrayItem> {
        self.shared
            .lock()
            .ok()
            .map(|state| state.items())
            .unwrap_or_default()
    }

    fn activate(&self, id: TrayItemId) -> Result<(), ContractError> {
        let target = self
            .shared
            .lock()
            .ok()
            .and_then(|state| state.click_for(id));
        let Some(target) = target else {
            return Err(ContractError::NotFound);
        };
        click::press(&target, false)
    }

    fn open_menu(&self, id: TrayItemId) -> Result<(), ContractError> {
        let target = self
            .shared
            .lock()
            .ok()
            .and_then(|state| state.click_for(id));
        let Some(target) = target else {
            return Err(ContractError::NotFound);
        };
        click::press(&target, true)
    }
}

impl Drop for TrayHost {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            close_worker(worker);
        }
    }
}

/// Avisa al hilo para que destruya su ventana y espera a que termine.
fn close_worker(worker: Worker) {
    let Worker { thread, window_raw } = worker;
    if window_raw != 0 {
        let window = HWND(window_raw as *mut c_void);
        if !window.is_invalid() {
            // SAFETY: avisa al hilo para que destruya su ventana; si ya
            // terminó, el aviso falla sin más.
            let _ = unsafe { PostMessageW(Some(window), WM_CLOSE, WPARAM(0), LPARAM(0)) };
        }
    }
    if let Some(thread) = thread {
        let _ = thread.join();
    }
}
