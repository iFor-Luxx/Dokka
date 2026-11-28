//! Instala y desinstala Dokka en Windows, sin interfaz.
#![allow(unsafe_code)]

mod close_dokka;
mod install_error;
mod installer;
mod launch;
mod locations;
mod payload_files;
mod registry;
mod registry_run;
mod registry_uninstall;
mod shortcuts;
mod uninstall_cleanup;

pub use installer::WindowsInstaller;
