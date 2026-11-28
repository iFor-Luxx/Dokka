//! Hilo propio que lee el equipo sin bloquear a la isla.
//!
//! La isla solo lee el último valor con `summary` y pide releer con
//! `request_refresh`, como `SystemBrightness`. Nunca se registra en el
//! log el número de serie ni el nombre del equipo.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, channel};
use std::thread;

use dokka_contracts::{ContractError, SystemInfo, SystemPage};
use dokka_model::{StorageBreakdown, SystemSummary};

use super::breakdown::StorageWorker;
use super::open;
use super::summary::read_all;

/// Órdenes al hilo lector.
#[derive(Debug, Clone, Copy)]
enum SystemCommand {
    Refresh,
}

/// Lector del equipo en su propio hilo.
#[derive(Debug)]
pub struct SystemSystemInfo {
    commands: Sender<SystemCommand>,
    shared: Arc<Mutex<Option<SystemSummary>>>,
    storage: StorageWorker,
}

impl SystemSystemInfo {
    pub fn start() -> Self {
        let (commands, queue) = channel();
        let shared = Arc::new(Mutex::new(None));
        let worker_shared = Arc::clone(&shared);
        thread::spawn(move || watch(&queue, &worker_shared));
        Self {
            commands,
            shared,
            storage: StorageWorker::start(),
        }
    }

    /// Pide releer el equipo en el hilo (al abrir "Acerca de este PC").
    pub fn request_refresh(&self) {
        let _ = self.commands.send(SystemCommand::Refresh);
    }

    /// Pide calcular el desglose (al abrir "Almacenamiento").
    pub fn request_storage(&self) {
        self.storage.request();
    }

    /// Detiene el cálculo (al salir de "Almacenamiento").
    pub fn cancel_storage(&self) {
        self.storage.cancel();
    }
}

impl Default for SystemSystemInfo {
    fn default() -> Self {
        Self::start()
    }
}

impl SystemInfo for SystemSystemInfo {
    fn summary(&self) -> Option<SystemSummary> {
        if let Ok(known) = self.shared.lock() {
            known.clone()
        } else {
            None
        }
    }

    fn request_refresh(&self) {
        Self::request_refresh(self);
    }

    fn storage(&self) -> Option<StorageBreakdown> {
        self.storage.storage()
    }

    fn request_storage(&self) {
        Self::request_storage(self);
    }

    fn cancel_storage(&self) {
        Self::cancel_storage(self);
    }

    fn open_page(&self, page: SystemPage) -> Result<(), ContractError> {
        open::open_address(page)
    }
}

/// Bucle lector: una lectura por orden, sin acumular trabajo.
fn watch(
    queue: &std::sync::mpsc::Receiver<SystemCommand>,
    shared: &Arc<Mutex<Option<SystemSummary>>>,
) {
    loop {
        if queue.recv().is_err() {
            return;
        }
        // Vacía la cola: varias peticiones seguidas son una sola lectura.
        for _ in queue.try_iter() {}
        let summary = read_all();
        if let Ok(mut known) = shared.lock() {
            *known = Some(summary);
        }
    }
}
