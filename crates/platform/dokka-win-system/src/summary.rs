//! Arma el resumen del equipo con lo que se pudo leer.
//!
//! Cada lector falla solo: su campo queda en `None` y el resto sigue.
//! El modelo final prefiere el producto del sistema; si es relleno,
//! usa el de la placa base con su fabricante.

use dokka_model::SystemSummary;

use super::display;
use super::graphics;
use super::machine;
use super::placeholders::clean;
use super::smbios;
use super::storage;

/// Lee todo una vez, sin detenerse en el primer fallo.
pub(crate) fn read_all() -> SystemSummary {
    let computer_name = machine::computer_name();
    let processor = machine::processor();
    let memory_bytes = machine::memory_bytes();
    let (edition, version) = machine::windows_edition_and_version();
    let graphics = graphics::graphics();
    let display_size = display::resolution();
    let display_name = display::display_name();
    let (drive, free, total) = storage::system_drive();
    let (model, serial) = model_and_serial();

    SystemSummary {
        computer_name,
        model,
        processor,
        graphics,
        memory_bytes,
        serial,
        windows_edition: edition,
        windows_version: version,
        display_name,
        display_size,
        storage_drive: drive,
        storage_free_bytes: free,
        storage_total_bytes: total,
        storage: None,
    }
}

/// Modelo visible y número de serie, con el plan B de la placa base.
fn model_and_serial() -> (Option<String>, Option<String>) {
    let (bios_manufacturer, bios_product) = machine::bios_model();
    let smbios = smbios::read_smbios();

    let mut system_product = bios_product;
    let mut system_manufacturer = bios_manufacturer;
    let mut serial = None;
    let mut board_product = None;
    let mut board_manufacturer = None;
    if let Some(table) = smbios {
        if system_product.is_none() {
            system_product = table.system_product;
        }
        // El registro manda; la tabla solo vale para el plan B.
        if system_manufacturer.is_none() {
            system_manufacturer = table.system_manufacturer;
        }
        serial = table.system_serial;
        board_product = table.board_product;
        board_manufacturer = table.board_manufacturer;
    }

    // Modelo: producto del sistema; si es relleno, el de la placa.
    let mut model = system_product;
    if model.is_none() {
        model = match (board_manufacturer.clone(), board_product.clone()) {
            (Some(maker), Some(product)) => clean(Some(format!("{maker} {product}"))),
            (None, Some(product)) => clean(Some(product)),
            (Some(maker), None) => clean(Some(maker)),
            (None, None) => None,
        };
    } else if let (Some(maker), Some(product)) = (system_manufacturer.clone(), model.clone()) {
        // Si el producto ya trae al fabricante, no lo duplica.
        if !product
            .to_ascii_lowercase()
            .contains(&maker.to_ascii_lowercase())
        {
            model = clean(Some(format!("{maker} {product}")));
        }
    }
    // El plan B de la placa para la serie no vale: es de otro componente.
    (model, serial)
}
