//! Trabajo de instalar en un hilo aparte; el avance llega por mensajes.
//!
//! La ventana nunca se bloquea: cada aviso del contrato se reenvía por el
//! canal y despierta a la ventana, que lo recoge como `Refresh`.

use std::sync::mpsc::Sender;
use std::thread;

use dokka_setup::WorkStep;
use dokka_win_surface::InstallerWaker;

/// Lo que el hilo de trabajo cuenta a la ventana.
#[derive(Debug)]
pub(crate) enum WorkerMsg {
    Progress { step: WorkStep, fraction: f32 },
    Done { detail: Option<String> },
}

/// Empieza a instalar en un hilo aparte con las opciones elegidas.
pub(crate) fn spawn_install(
    start_with_windows: bool,
    desktop_shortcut: bool,
    sender: Sender<WorkerMsg>,
    waker: InstallerWaker,
) {
    thread::spawn(move || {
        run_install(start_with_windows, desktop_shortcut, &sender, waker);
    });
}

/// Empieza a desinstalar en un hilo aparte.
pub(crate) fn spawn_uninstall(
    remove_settings: bool,
    sender: Sender<WorkerMsg>,
    waker: InstallerWaker,
) {
    thread::spawn(move || {
        run_uninstall(remove_settings, &sender, waker);
    });
}

fn run_install(
    start_with_windows: bool,
    desktop_shortcut: bool,
    sender: &Sender<WorkerMsg>,
    waker: InstallerWaker,
) {
    use dokka_contracts::{InstallOptions, InstallStep, Installer};
    let options = InstallOptions {
        start_with_windows,
        desktop_shortcut,
    };
    let installer = dokka_win_install::WindowsInstaller::new(super::PAYLOAD);
    let result = installer.install(options, &mut |step: InstallStep, fraction: f32| {
        report(sender, waker, map_step(step), fraction);
    });
    let detail = match result {
        Ok(()) => None,
        Err(error) => Some(format!(
            "{} No se cambió nada en tu PC.",
            failure_text(&error)
        )),
    };
    let _ = sender.send(WorkerMsg::Done { detail });
    waker.wake();
}

fn run_uninstall(remove_settings: bool, sender: &Sender<WorkerMsg>, waker: InstallerWaker) {
    use dokka_contracts::Installer;
    let installer = dokka_win_install::WindowsInstaller::new(super::PAYLOAD);
    let result = installer.uninstall(remove_settings, &mut |step, fraction| {
        report(sender, waker, map_step(step), fraction);
    });
    let detail = match result {
        Ok(()) => None,
        Err(error) => Some(failure_text(&error)),
    };
    let _ = sender.send(WorkerMsg::Done { detail });
    waker.wake();
}

/// Traduce el paso del contrato al de la isla, en el mismo orden.
fn map_step(step: dokka_contracts::InstallStep) -> WorkStep {
    use dokka_contracts::InstallStep;
    match step {
        InstallStep::ClosingDokka => WorkStep::Closing,
        InstallStep::CopyingFiles => WorkStep::Copying,
        InstallStep::CreatingShortcuts => WorkStep::Shortcuts,
        InstallStep::Registering => WorkStep::Registering,
        InstallStep::RemovingFiles => WorkStep::Removing,
    }
}

/// Frase del fallo en español, sin tecnicismos.
fn failure_text(failure: &dokka_contracts::InstallFailure) -> String {
    use dokka_contracts::InstallFailure;
    match failure {
        InstallFailure::NoSpace => "No hay espacio suficiente en el disco.".to_owned(),
        InstallFailure::CannotCloseDokka => "No se pudo cerrar Dokka.".to_owned(),
        InstallFailure::PayloadMissing => "Falta el archivo de instalación.".to_owned(),
        InstallFailure::AccessDenied => "Windows denegó el permiso.".to_owned(),
        InstallFailure::Other(message) => message.clone(),
    }
}

/// Reenvía el avance y despierta a la ventana.
fn report(sender: &Sender<WorkerMsg>, waker: InstallerWaker, step: WorkStep, fraction: f32) {
    let _ = sender.send(WorkerMsg::Progress { step, fraction });
    waker.wake();
}
