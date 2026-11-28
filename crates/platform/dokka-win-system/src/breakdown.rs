//! Cálculo del desglose de almacenamiento en su propio hilo.
//!
//! La isla solo lee el último valor con `storage` y pide calcularlo con
//! `request_storage` al abrir el apartado; `cancel_storage` lo detiene al
//! salir. Lo barato (papelera, programas, otros discos) va primero para que
//! la ficha se rellene pronto; las carpetas después. Al cancelar se conserva
//! lo ya calculado. Nunca se registra ninguna ruta.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::thread;

use dokka_model::StorageBreakdown;

use super::apps_size;
use super::disks;
use super::folder_size;
use super::known_folders;
use super::storage;

/// Orden de cálculo con su generación: otra generación lo cancela.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StorageJob(u64);

/// Cálculo del desglose en su propio hilo.
#[derive(Debug)]
pub struct StorageWorker {
    commands: Sender<StorageJob>,
    shared: Arc<Mutex<Option<StorageBreakdown>>>,
    generation: Arc<AtomicU64>,
}

impl StorageWorker {
    pub fn start() -> Self {
        let (commands, queue) = channel();
        let shared = Arc::new(Mutex::new(None));
        let generation = Arc::new(AtomicU64::new(0));
        let worker_shared = Arc::clone(&shared);
        let worker_generation = Arc::clone(&generation);
        thread::spawn(move || watch(&queue, &worker_shared, &worker_generation));
        Self {
            commands,
            shared,
            generation,
        }
    }

    /// Último desglose calculado, aunque sea parcial.
    pub fn storage(&self) -> Option<StorageBreakdown> {
        if let Ok(known) = self.shared.lock() {
            known.clone()
        } else {
            None
        }
    }

    /// Calcula el desglose en el hilo; solo al abrir el apartado.
    pub fn request(&self) {
        let job = StorageJob(self.generation.fetch_add(1, Ordering::Relaxed) + 1);
        let _ = self.commands.send(job);
    }

    /// Detiene el cálculo en curso; conserva lo ya calculado.
    pub fn cancel(&self) {
        self.generation.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for StorageWorker {
    fn default() -> Self {
        Self::start()
    }
}

/// Bucle lector: una orden nueva cancela la anterior por generación.
fn watch(
    queue: &std::sync::mpsc::Receiver<StorageJob>,
    shared: &Arc<Mutex<Option<StorageBreakdown>>>,
    generation: &Arc<AtomicU64>,
) {
    loop {
        let first = match queue.recv() {
            Ok(job) => job,
            Err(_) => return,
        };
        // Varias peticiones seguidas son solo la última.
        let mut latest = first;
        for queued in queue.try_iter() {
            latest = queued;
        }
        if latest.0 != generation.load(Ordering::Relaxed) {
            continue;
        }
        compute(latest, shared, generation);
    }
}

/// Calcula cada categoría y publica el progreso; se detiene si la
/// generación cambia (otra petición o cancelación al salir).
fn compute(
    job: StorageJob,
    shared: &Arc<Mutex<Option<StorageBreakdown>>>,
    generation: &Arc<AtomicU64>,
) {
    let live = || generation.load(Ordering::Relaxed) == job.0;
    // Si no hay nada, se empieza de cero para mostrar "Calculando…"; si hay
    // un resultado anterior, se conserva y se reescribe por encima.
    let fresh = shared
        .lock()
        .map(|known| known.is_none())
        .unwrap_or(false);
    if fresh {
        publish(shared, StorageBreakdown::default());
    }
    let (drive, free, total) = storage::system_drive();
    if !live() {
        return;
    }
    update(shared, |breakdown| {
        breakdown.drive = drive.clone();
    });
    publish_free_total(shared, free, total);
    if !live() {
        return;
    }
    publish_recycle(shared, disks::recycle_size());
    if !live() {
        return;
    }
    publish_apps(shared, apps_size::installed_apps_size());
    if !live() {
        return;
    }
    publish_disks(shared, disks::other_disks(drive.as_deref()));
    if !live() {
        return;
    }
    let folders = known_folders::user_folders();
    let slots = [
        Category::Documents,
        Category::Images,
        Category::Music,
        Category::Videos,
        Category::Downloads,
    ];
    for (slot, folder) in slots.into_iter().zip(folders) {
        if !live() {
            return;
        }
        let size = folder.as_deref().map(|root| {
            folder_size::folder_size(root, &live).unwrap_or(0)
        });
        publish_category(shared, slot, size);
    }
    if !live() {
        return;
    }
    let windows = known_folders::windows_folder()
        .as_deref()
        .map(|root| folder_size::folder_size(root, &live).unwrap_or(0));
    publish_category(shared, Category::Windows, windows);
    if !live() {
        return;
    }
    // Relee la papelera al final: su confirmación pudo cerrarse mientras se
    // recorrían las carpetas, y medirla solo al principio quedaría obsoleto.
    publish_recycle(shared, disks::recycle_size());
}

/// Categoría que se publica.
#[derive(Debug, Clone, Copy)]
enum Category {
    Documents,
    Images,
    Music,
    Videos,
    Downloads,
    Windows,
}

fn publish(shared: &Arc<Mutex<Option<StorageBreakdown>>>, breakdown: StorageBreakdown) {
    if let Ok(mut known) = shared.lock() {
        *known = Some(breakdown);
    }
}

fn update(shared: &Arc<Mutex<Option<StorageBreakdown>>>, set: impl FnOnce(&mut StorageBreakdown)) {
    if let Ok(mut known) = shared.lock() {
        if let Some(breakdown) = known.as_mut() {
            set(breakdown);
        }
    }
}

fn publish_free_total(
    shared: &Arc<Mutex<Option<StorageBreakdown>>>,
    free: Option<u64>,
    total: Option<u64>,
) {
    update(shared, |breakdown| {
        breakdown.free_bytes = free;
        breakdown.total_bytes = total;
    });
}

fn publish_recycle(shared: &Arc<Mutex<Option<StorageBreakdown>>>, size: Option<u64>) {
    update(shared, |breakdown| breakdown.recycle_bytes = Some(size.unwrap_or(0)));
}

fn publish_apps(shared: &Arc<Mutex<Option<StorageBreakdown>>>, size: Option<u64>) {
    update(shared, |breakdown| breakdown.apps_bytes = Some(size.unwrap_or(0)));
}

fn publish_disks(
    shared: &Arc<Mutex<Option<StorageBreakdown>>>,
    disks: Vec<dokka_model::OtherDisk>,
) {
    update(shared, |breakdown| breakdown.other_disks = disks);
}

fn publish_category(
    shared: &Arc<Mutex<Option<StorageBreakdown>>>,
    slot: Category,
    size: Option<u64>,
) {
    let size = Some(size.unwrap_or(0));
    update(shared, |breakdown| match slot {
        Category::Documents => breakdown.documents_bytes = size,
        Category::Images => breakdown.images_bytes = size,
        Category::Music => breakdown.music_bytes = size,
        Category::Videos => breakdown.videos_bytes = size,
        Category::Downloads => breakdown.downloads_bytes = size,
        Category::Windows => breakdown.windows_bytes = size,
    });
}
