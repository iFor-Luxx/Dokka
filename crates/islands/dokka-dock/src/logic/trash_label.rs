/// Etiqueta de la papelera: solo el nombre si está vacía, con peso si tiene contenido.
pub(crate) fn trash_label(size: Option<u64>) -> String {
    match size {
        None => "Papelera".to_owned(),
        Some(bytes) => format!("Papelera · {}", format_size(bytes)),
    }
}

/// Peso en bytes con unidades de 1024, como el Explorador de Windows.
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{bytes} bytes");
    }
    // Empieza en KB y sube mientras el valor alcance 1024.
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    let mut value = bytes as f64 / 1024.0;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    // Al redondear hasta 1024 se pasa a la unidad siguiente.
    if unit < UNITS.len() - 1 {
        let rounded = if value < 10.0 {
            (value * 10.0).round() / 10.0
        } else {
            value.round()
        };
        if rounded >= 1024.0 {
            value /= 1024.0;
            unit += 1;
        }
    }
    if value < 10.0 {
        // Un decimal con coma, sin ",0".
        let tenths = (value * 10.0).round() as u64;
        let whole = tenths / 10;
        let decimal = tenths % 10;
        if decimal == 0 {
            format!("{whole} {}", UNITS[unit])
        } else {
            format!("{whole},{decimal} {}", UNITS[unit])
        }
    } else {
        format!("{} {}", value.round() as u64, UNITS[unit])
    }
}
