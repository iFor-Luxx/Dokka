//! Copia el instalable a `OUT_DIR/payload.bin` o deja uno vacío.
//!
//! Con `DOKKA_PAYLOAD` copia ese archivo (lo genera `tools/build_installer.ps1`);
//! sin ella crea uno vacío para que `cargo check` y `cargo clippy` del
//! workspace funcionen sin el binario. Embebe además el manifiesto de
//! administrador solo para el binario.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=DOKKA_PAYLOAD");
    println!("cargo:rerun-if-changed=build.rs");
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let target = out.join("payload.bin");
    match env::var("DOKKA_PAYLOAD") {
        Ok(path) if !path.trim().is_empty() => {
            println!("cargo:rerun-if-changed={path}");
            let bytes = fs::read(&path)
                .map_err(|error| format!("no se pudo leer DOKKA_PAYLOAD: {error}"))?;
            fs::write(&target, bytes)?;
        }
        _ => {
            fs::write(&target, [])?;
        }
    }
    println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-bins=/MANIFESTUAC:level='requireAdministrator' uiAccess='false'"
    );
    Ok(())
}
