//! Resumen del equipo para "Acerca de este PC", "Fecha y hora",
//! "Idioma y región", "Almacenamiento" y "Actualización de Dokka".
//!
//! Todo es opcional: lo que no se puede leer es `None` y se muestra
//! como "No disponible" (o "Calculando…" mientras se calcula el desglose).
//! No incluye nada sensible en el log: quien lo
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
    /// Fecha, hora y zona horaria leídas de Windows.
    pub datetime: DateTimeSummary,
    /// Idiomas y formatos leídos de Windows.
    pub locale: LocaleSummary,
    /// Desglose por categorías del disco del sistema, si ya se calculó.
    pub storage: Option<StorageBreakdown>,
}

/// Fecha y hora de Windows: todo opcional, lo que no se lee es `None`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DateTimeSummary {
    /// `true` con "Fecha y hora automáticas" de Windows.
    pub auto_time: Option<bool>,
    /// Servidor NTP de Windows, como "time.windows.com".
    pub ntp_server: Option<String>,
    /// `true` con "Zona horaria automática" de Windows.
    pub auto_timezone: Option<bool>,
    /// Nombre de la zona horaria actual.
    pub timezone_name: Option<String>,
}

impl DateTimeSummary {
    /// `true` cuando no se leyó nada útil.
    pub fn is_empty(&self) -> bool {
        self.auto_time.is_none()
            && self.ntp_server.is_none()
            && self.auto_timezone.is_none()
            && self.timezone_name.is_none()
    }
}

/// Idioma y región de Windows: todo opcional, lo que no se lee es `None`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocaleSummary {
    /// Idiomas preferidos del usuario, el primero es el principal.
    pub languages: Option<Vec<String>>,
    /// Región, como "Perú".
    pub region_name: Option<String>,
    /// Calendario, como "Gregoriano".
    pub calendar_name: Option<String>,
    /// Temperatura, "Celsius" o "Fahrenheit".
    pub temperature_unit: Option<String>,
    /// Sistema de medida, "Métrico" o "EE. UU.".
    pub measurement_system: Option<String>,
    /// Patrón de fecha corta de Windows, como "d/M/yyyy".
    pub date_pattern: Option<String>,
    /// Número con el formato de Windows, como "1.234,56".
    pub number_format: Option<String>,
    /// Fecha larga de hoy con el formato de Windows.
    pub long_date_example: Option<String>,
    /// Fecha corta de hoy con el formato de Windows.
    pub short_date_example: Option<String>,
    /// Número con el formato de Windows.
    pub number_example: Option<String>,
}

impl LocaleSummary {
    /// `true` cuando no se leyó nada útil.
    pub fn is_empty(&self) -> bool {
        self.languages.is_none()
            && self.region_name.is_none()
            && self.calendar_name.is_none()
            && self.temperature_unit.is_none()
            && self.measurement_system.is_none()
            && self.date_pattern.is_none()
            && self.number_format.is_none()
            && self.long_date_example.is_none()
            && self.short_date_example.is_none()
            && self.number_example.is_none()
    }
}

/// Uso del disco del sistema por categorías, en bytes.
///
/// Cada campo es `None` mientras esa categoría se calcula; `Some(0)` cuando
/// se leyó y está vacía. "Otros" no se guarda: es lo usado menos la suma,
/// nunca negativo, y lo calcula quien lo muestra.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StorageBreakdown {
    /// Suma de `EstimatedSize` de los programas instalados.
    pub apps_bytes: Option<u64>,
    /// Carpeta Documentos del usuario.
    pub documents_bytes: Option<u64>,
    /// Carpeta Imágenes del usuario.
    pub images_bytes: Option<u64>,
    /// Carpeta Música del usuario.
    pub music_bytes: Option<u64>,
    /// Carpeta Videos del usuario.
    pub videos_bytes: Option<u64>,
    /// Carpeta Descargas del usuario.
    pub downloads_bytes: Option<u64>,
    /// Papelera (`SHQueryRecycleBinW`).
    pub recycle_bytes: Option<u64>,
    /// Carpeta de Windows.
    pub windows_bytes: Option<u64>,
    /// Libres y total del disco del sistema al calcular.
    pub free_bytes: Option<u64>,
    pub total_bytes: Option<u64>,
    /// Letra del disco del sistema, como "C:".
    pub drive: Option<String>,
    /// Otros discos fijos, si hay.
    pub other_disks: Vec<OtherDisk>,
}

impl StorageBreakdown {
    /// Suma de las categorías ya calculadas, sin contar "Otros".
    pub fn known_bytes(&self) -> u64 {
        [
            self.apps_bytes,
            self.documents_bytes,
            self.images_bytes,
            self.music_bytes,
            self.videos_bytes,
            self.downloads_bytes,
            self.recycle_bytes,
            self.windows_bytes,
        ]
        .into_iter()
        .flatten()
        .sum()
    }

    /// Bytes usados del disco del sistema, si se conocen libres y total.
    pub fn used_bytes(&self) -> Option<u64> {
        match (self.free_bytes, self.total_bytes) {
            (Some(free), Some(total)) => Some(total.saturating_sub(free)),
            _ => None,
        }
    }

    /// "Datos del sistema y otros": lo usado menos la suma, nunca negativo.
    /// `None` si aún no se conocen los bytes usados.
    pub fn other_bytes(&self) -> Option<u64> {
        self.used_bytes()
            .map(|used| used.saturating_sub(self.known_bytes()))
    }

    /// `true` cuando todas las categorías ya tienen valor.
    pub fn is_complete(&self) -> bool {
        self.apps_bytes.is_some()
            && self.documents_bytes.is_some()
            && self.images_bytes.is_some()
            && self.music_bytes.is_some()
            && self.videos_bytes.is_some()
            && self.downloads_bytes.is_some()
            && self.recycle_bytes.is_some()
            && self.windows_bytes.is_some()
            && self.free_bytes.is_some()
            && self.total_bytes.is_some()
    }
}

/// Otro disco fijo con sus bytes libres y totales.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OtherDisk {
    /// Letra del disco, como "D:".
    pub drive: String,
    /// Bytes libres.
    pub free_bytes: u64,
    /// Bytes totales.
    pub total_bytes: u64,
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
            && self.datetime.is_empty()
            && self.locale.is_empty()
            && self.storage.is_none()
    }
}

/// Memoria o disco en GB con un decimal, como "15,6 GB".
/// Usa coma decimal, como el resto de Dokka.
pub fn bytes_in_gb(bytes: u64) -> String {
    let gb = bytes as f64 / 1024.0 / 1024.0 / 1024.0;
    let rounded = (gb * 10.0).round() / 10.0;
    format!("{rounded:.1}").replace('.', ",") + " GB"
}
