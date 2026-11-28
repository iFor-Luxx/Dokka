//! Hilo propio que escribe fecha, hora y zona sin bloquear a la isla.
//!
//! La isla solo emite acciones; el shell las conecta aquí. Cada orden se
//! ejecuta en este hilo y luego se relee el resumen para que la pantalla
//! muestre el valor real. Si algo falla se registra y el resto sigue.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, channel};
use std::thread;

use dokka_contracts::{DateTimeControls, ManualDateTime, TimezoneOption};
use dokka_model::SystemSummary;

/// Órdenes de escritura, una por clic del usuario.
#[derive(Debug)]
pub(crate) enum WriteCommand {
    AutoTime(bool),
    NtpServer(String),
    Manual(ManualDateTime),
    AutoZone(bool),
    Zone(String),
    LoadZones,
}

/// Escritor en su propio hilo con la última lista de zonas.
#[derive(Debug)]
pub(crate) struct DateTimeWriteWorker {
    commands: Sender<WriteCommand>,
    zones: Arc<Mutex<Option<Vec<TimezoneOption>>>>,
}

impl DateTimeWriteWorker {
    pub(crate) fn start(shared: Arc<Mutex<Option<SystemSummary>>>) -> Self {
        let (commands, queue) = channel();
        let zones = Arc::new(Mutex::new(None));
        let worker_zones = Arc::clone(&zones);
        thread::spawn(move || watch(&queue, &shared, &worker_zones));
        Self { commands, zones }
    }

    pub(crate) fn send(&self, command: WriteCommand) {
        let _ = self.commands.send(command);
    }

    pub(crate) fn cached_zones(&self) -> Option<Vec<TimezoneOption>> {
        if let Ok(known) = self.zones.lock() {
            known.clone()
        } else {
            None
        }
    }
}

/// Bucle escritor: una orden por clic, luego relee el resumen.
fn watch(
    queue: &std::sync::mpsc::Receiver<WriteCommand>,
    shared: &Arc<Mutex<Option<SystemSummary>>>,
    zones: &Arc<Mutex<Option<Vec<TimezoneOption>>>>,
) {
    loop {
        let Ok(command) = queue.recv() else {
            return;
        };
        match command {
            WriteCommand::AutoTime(enabled) => {
                if !super::datetime_set::set_automatic_time(enabled) {
                    eprintln!("Dokka no pudo cambiar la hora automática");
                }
                refresh(shared);
            }
            WriteCommand::NtpServer(server) => {
                if !super::datetime_set::set_ntp_server(&server) {
                    eprintln!("Dokka no pudo cambiar el servidor NTP");
                }
                refresh(shared);
            }
            WriteCommand::Manual(date) => {
                let done = super::datetime_set::set_manual_time(
                    date.year,
                    date.month,
                    date.day,
                    date.hour,
                    date.minute,
                );
                if !done {
                    eprintln!("Dokka no pudo cambiar la fecha y hora");
                }
                refresh(shared);
            }
            WriteCommand::AutoZone(enabled) => {
                if !super::datetime_set::set_automatic_timezone(enabled) {
                    eprintln!("Dokka no pudo cambiar la zona automática");
                }
                refresh(shared);
            }
            WriteCommand::Zone(key) => {
                if !super::datetime_set::set_timezone(&key) {
                    eprintln!("Dokka no pudo cambiar la zona horaria");
                }
                refresh(shared);
            }
            WriteCommand::LoadZones => {
                let list = super::datetime_zones::list_timezones();
                if let Ok(mut known) = zones.lock() {
                    *known = Some(list);
                }
            }
        }
    }
}

/// Relee el resumen para que la pantalla muestre el valor real.
fn refresh(shared: &Arc<Mutex<Option<SystemSummary>>>) {
    let summary = super::summary::read_all();
    if let Ok(mut known) = shared.lock() {
        *known = Some(summary);
    }
}

impl DateTimeControls for super::monitor::SystemSystemInfo {
    fn can_change_protected(&self) -> bool {
        super::datetime_elevation::is_elevated()
    }

    fn request_timezones(&self) {
        self.write_send(WriteCommand::LoadZones);
    }

    fn timezones(&self) -> Option<Vec<TimezoneOption>> {
        self.write_zones()
    }

    fn set_automatic_time(&self, enabled: bool) {
        self.write_send(WriteCommand::AutoTime(enabled));
    }

    fn set_ntp_server(&self, server: String) {
        self.write_send(WriteCommand::NtpServer(server));
    }

    fn set_manual_time(&self, date: ManualDateTime) {
        self.write_send(WriteCommand::Manual(date));
    }

    fn set_automatic_timezone(&self, enabled: bool) {
        self.write_send(WriteCommand::AutoZone(enabled));
    }

    fn set_timezone(&self, key: String) {
        self.write_send(WriteCommand::Zone(key));
    }
}
