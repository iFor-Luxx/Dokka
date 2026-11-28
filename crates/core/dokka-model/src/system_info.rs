//! Resumen del equipo para "Acerca de este PC".
//!
//! Todo es opcional: lo que no se puede leer es `None` y se muestra
//! como "No disponible". No incluye nada sensible en el log: quien lo
//! use no debe registrar el número de serie ni el nombre del equipo.

/// Resumen del equipo, con lo que se pudo leer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SystemSummary {
    /// Nombre del equipo en la red.
    pub computer_name: Option<String>,
    /// Modelo visible, ya con fabricante si hace falta.
    pub model: Option<String>,
    /// Procesador, tal como lo anuncia Windows.
    pub processor: Option<String>,
    /// Gráfica principal (DXGI).
    pub graphics: Option<String>,
    /// Memoria instalada en bytes.
    pub memory_bytes: Option<u64>,
    /// Número de serie de la placa o del equipo.
    pub serial: Option<String>,
    /// Edición de Windows, como "Windows 11 Pro".
    pub windows_edition: Option<String>,
    /// Versión de Windows, como "24H2 (compilación 26100.4351)".
    pub windows_version: Option<String>,
    /// Nombre de la pantalla principal.
    pub display_name: Option<String>,
    /// Resolución de la pantalla principal, como "1920 × 1080".
    pub display_size: Option<String>,
    /// Letra del disco del sistema, como "C:".
    pub storage_drive: Option<String>,
    /// Bytes libres en el disco del sistema.
    pub storage_free_bytes: Option<u64>,
    /// Bytes totales del disco del sistema.
    pub storage_total_bytes: Option<u64>,
}

impl SystemSummary {
    /// Texto para mostrar, o "No disponible" si no se pudo leer.
    pub fn display(value: &Option<String>) -> &str {
        value.as_deref().unwrap_or("No disponible")
    }

    /// `true` cuando no se leyó nada útil.
    pub fn is_empty(&self) -> bool {
        self.computer_name.is_none()
            && self.model.is_none()
            && self.processor.is_none()
            && self.graphics.is_none()
            && self.memory_bytes.is_none()
            && self.serial.is_none()
            && self.windows_edition.is_none()
            && self.windows_version.is_none()
            && self.display_name.is_none()
            && self.display_size.is_none()
            && self.storage_drive.is_none()
            && self.storage_free_bytes.is_none()
            && self.storage_total_bytes.is_none()
    }
}

/// Memoria o disco en GB con un decimal, como "15,6 GB".
/// Usa coma decimal, como el resto de Dokka.
pub fn bytes_in_gb(bytes: u64) -> String {
    let gb = bytes as f64 / 1024.0 / 1024.0 / 1024.0;
    let rounded = (gb * 10.0).round() / 10.0;
    format!("{rounded:.1}").replace('.', ",") + " GB"
}
