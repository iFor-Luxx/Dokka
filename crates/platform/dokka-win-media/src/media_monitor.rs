use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::thread;
use std::time::Duration;

use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager as Manager;
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize};
use windows::core::Result;

use crate::MediaSession;
use crate::read_sessions::{perform, read_sessions};

const REFRESH: Duration = Duration::from_millis(700);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaAction {
    Previous,
    Toggle,
    Next,
}

/// Vigila la reproducción desde un hilo aparte.
#[derive(Debug)]
pub struct MediaMonitor {
    commands: Sender<(String, MediaAction)>,
    updates: Receiver<Vec<MediaSession>>,
    latest: Vec<MediaSession>,
}

impl MediaMonitor {
    pub fn start() -> Self {
        let (commands, command_queue) = channel();
        let (update_sender, updates) = channel();
        // El hilo termina solo cuando se suelta este monitor.
        thread::spawn(move || {
            if let Err(error) = watch(&command_queue, &update_sender) {
                eprintln!("Controles de música no disponibles: {error}");
            }
        });
        Self {
            commands,
            updates,
            latest: Vec::new(),
        }
    }

    /// Estado más reciente, sin esperar.
    pub fn sessions(&mut self) -> &[MediaSession] {
        if let Some(newest) = self.updates.try_iter().last() {
            self.latest = newest;
        }
        &self.latest
    }

    pub fn send(&mut self, source: &str, action: MediaAction) {
        if action == MediaAction::Toggle {
            // Parte del estado más fresco y adelanta el icono sin esperar al hilo.
            let _ = self.sessions();
            for media in &mut self.latest {
                if media.source == source {
                    media.is_playing = !media.is_playing;
                }
            }
        }
        // Si el hilo ya terminó, la orden simplemente se pierde.
        let _ = self.commands.send((source.to_owned(), action));
    }
}

fn watch(
    commands: &Receiver<(String, MediaAction)>,
    updates: &Sender<Vec<MediaSession>>,
) -> Result<()> {
    // SAFETY: inicializa Windows Runtime solo para este hilo, que vive con el proceso.
    unsafe { RoInitialize(RO_INIT_MULTITHREADED)? };
    let manager = Manager::RequestAsync()?.join()?;
    let mut previous = Vec::new();
    // Tras una orden, la siguiente lectura se envía siempre para confirmar
    // el valor adelantado (o corregirlo si la app no obedeció).
    let mut confirm_pending = false;
    loop {
        match commands.recv_timeout(REFRESH) {
            Ok((source, action)) => {
                if let Err(error) = perform(&manager, &source, action) {
                    eprintln!("No se pudo controlar la reproducción: {error}");
                }
                // Sin leer de inmediato: la app aún no ha cambiado y esa
                // lectura antigua revertiría el icono adelantado. Si llega
                // otra orden antes de la siguiente lectura, sigue valiendo
                // esta misma obligación.
                confirm_pending = true;
                continue;
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Ok(()),
        }
        let current = read_sessions(&manager);
        if confirm_pending || current != previous {
            previous.clone_from(&current);
            confirm_pending = false;
            if updates.send(current).is_err() {
                return Ok(());
            }
        }
    }
}
