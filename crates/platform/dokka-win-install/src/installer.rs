//! `WindowsInstaller`: instala y desinstala Dokka cumpliendo `Installer`.
//!
//! Cada paso avisa con `progress` (paso y avance total de 0.0 a 1.0). Si
//! algo falla a mitad de `install`, se deshace lo ya hecho (archivos,
//! accesos directos, arranque y entrada del registro) para que no quede nada
//! a medias: la pantalla puede decir "No se cambió nada".

use std::path::PathBuf;

use dokka_contracts::{InstallFailure, InstallOptions, InstallStep, Installer};

use crate::close_dokka::close_running_dokka;
use crate::launch::launch_exe;
use crate::locations::{
    current_exe_bytes, desktop_link, dokka_exe, install_dir, start_menu_link, uninstaller_exe,
};
use crate::payload_files::{StagedFile, commit_staged, rollback_staged, stage_bytes};
use crate::registry_run::{
    RunBackup, read_run_value, remove_run_value, restore_run_value, set_run_value,
};
use crate::registry_uninstall::{
    UninstallBackup, remove_uninstall_entry, restore_uninstall_entry, write_uninstall_entry,
};
use crate::shortcuts::{ShortcutChange, create_shortcut, remove_shortcut, undo_shortcut};
use crate::uninstall_cleanup::{remove_install_files, remove_settings_dir};

/// El contenido de `dokka-shell.exe` ya compilado, para escribir `Dokka.exe`.
pub struct WindowsInstaller {
    payload: &'static [u8],
}

impl WindowsInstaller {
    pub fn new(payload: &'static [u8]) -> Self {
        Self { payload }
    }
}

/// Lo ya hecho en `install`, para deshacerlo si algo falla.
#[derive(Default)]
struct Journal {
    staged: Vec<StagedFile>,
    shortcuts: Vec<ShortcutChange>,
    run: Option<RunBackup>,
    uninstall: Option<UninstallBackup>,
    created_dir: Option<PathBuf>,
}

impl Journal {
    /// Diario vacío para una instalación.
    fn new() -> Self {
        Self::default()
    }

    /// Deshace en orden inverso; cada paso hace lo que puede sin fallar.
    fn rollback(&self) {
        if let Some(backup) = &self.uninstall {
            restore_uninstall_entry(backup);
        }
        if let Some(backup) = &self.run {
            restore_run_value(backup);
        }
        for change in self.shortcuts.iter().rev() {
            undo_shortcut(change);
        }
        for staged in self.staged.iter().rev() {
            rollback_staged(staged);
        }
        if let Some(dir) = &self.created_dir {
            let _ = std::fs::remove_dir(dir);
        }
    }
}

impl Installer for WindowsInstaller {
    fn install(
        &self,
        options: InstallOptions,
        progress: &mut dyn FnMut(InstallStep, f32),
    ) -> Result<(), InstallFailure> {
        if self.payload.is_empty() {
            return Err(InstallFailure::PayloadMissing);
        }
        let mut journal = Journal::new();
        let done = install_inner(self.payload, options, progress, &mut journal);
        match done {
            Ok(()) => {
                for staged in &journal.staged {
                    commit_staged(staged);
                }
                Ok(())
            }
            Err(error) => {
                journal.rollback();
                Err(error)
            }
        }
    }

    fn uninstall(
        &self,
        remove_settings: bool,
        progress: &mut dyn FnMut(InstallStep, f32),
    ) -> Result<(), InstallFailure> {
        progress(InstallStep::ClosingDokka, 0.1);
        close_running_dokka()?;
        progress(InstallStep::ClosingDokka, 0.25);
        remove_shortcut(&start_menu_link()?)?;
        remove_shortcut(&desktop_link()?)?;
        progress(InstallStep::RemovingFiles, 0.4);
        remove_run_value()?;
        remove_uninstall_entry()?;
        progress(InstallStep::Registering, 0.6);
        remove_install_files(&install_dir()?)?;
        progress(InstallStep::RemovingFiles, 0.85);
        if remove_settings {
            remove_settings_dir()?;
        }
        progress(InstallStep::RemovingFiles, 1.0);
        Ok(())
    }

    fn launch_dokka(&self) -> Result<(), InstallFailure> {
        let exe = dokka_exe()?;
        if !exe.is_file() {
            return Err(InstallFailure::Other("Dokka no está instalado".to_owned()));
        }
        launch_exe(&exe)
    }
}

/// Pasos de `install` registrando cada cambio en el diario.
fn install_inner(
    payload: &[u8],
    options: InstallOptions,
    progress: &mut dyn FnMut(InstallStep, f32),
    journal: &mut Journal,
) -> Result<(), InstallFailure> {
    progress(InstallStep::ClosingDokka, 0.05);
    close_running_dokka()?;
    progress(InstallStep::ClosingDokka, 0.2);
    let dir = install_dir()?;
    if !dir.is_dir() {
        journal.created_dir = Some(dir.clone());
    }
    let exe = dokka_exe()?;
    let uninstaller = uninstaller_exe()?;
    journal.staged.push(stage_bytes(&exe, payload)?);
    progress(InstallStep::CopyingFiles, 0.35);
    let current = current_exe_bytes()?;
    journal.staged.push(stage_bytes(&uninstaller, &current)?);
    progress(InstallStep::CopyingFiles, 0.5);
    journal
        .shortcuts
        .push(create_shortcut(&start_menu_link()?, &exe)?);
    progress(InstallStep::CreatingShortcuts, 0.6);
    if options.desktop_shortcut {
        journal
            .shortcuts
            .push(create_shortcut(&desktop_link()?, &exe)?);
    }
    progress(InstallStep::CreatingShortcuts, 0.7);
    if options.start_with_windows {
        journal.run = Some(set_run_value(&exe)?);
    } else {
        journal.run = Some(RunBackup {
            previous: read_run_value(),
        });
        remove_run_value()?;
    }
    progress(InstallStep::Registering, 0.85);
    journal.uninstall = Some(write_uninstall_entry(&dir, &exe, &uninstaller)?);
    progress(InstallStep::Registering, 1.0);
    Ok(())
}
