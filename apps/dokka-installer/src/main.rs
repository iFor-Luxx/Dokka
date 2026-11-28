//! Punto de entrada del instalador propio: solo arranca y conecta.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod run;
mod worker;

use std::error::Error;

/// Instalable embebido por `build.rs` desde `DOKKA_PAYLOAD` (vacío sin ella).
static PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/payload.bin"));

fn main() -> Result<(), Box<dyn Error>> {
    run::run()
}
