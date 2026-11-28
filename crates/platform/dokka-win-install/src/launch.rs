//! Apertura de `Dokka.exe` sin elevación.
//!
//! El instalador va elevado; abrir Dokka directamente lo elevaría también.
//! Lanzar `explorer.exe` con la ruta como argumento lo abre como un clic del
//! usuario, sin elevación.

use std::path::Path;
use std::process::Command;

use dokka_contracts::InstallFailure;

use crate::install_error::from_io;

/// Abre `Dokka.exe` con `explorer.exe`, sin heredar la elevación.
pub(crate) fn launch_exe(exe: &Path) -> Result<(), InstallFailure> {
    let explorer = crate::locations::explorer_exe()?;
    Command::new(explorer)
        .arg(exe)
        .spawn()
        .map(drop)
        .map_err(|error| from_io("no se pudo abrir Dokka", &error))
}
