//! Estado de Almacenamiento y Actualización dentro de General.
//!
//! Vive aparte para que `state.rs` y `general_state.rs` no crezcan: el
//! desglose calculado lo pasa el shell (`set_storage`) tras pedirlo al
//! entrar, y la versión de Dokka la fija el shell al abrir (`set_dokka_version`).

use super::state::Settings;

/// Categorías del desglose en el orden de la lista y de la barra.
pub(crate) const STORAGE_CATEGORIES: usize = 9;

/// Nombres de las filas de categorías, en orden de arriba abajo.
pub(crate) fn storage_category_names() -> [&'static str; STORAGE_CATEGORIES] {
    [
        "Aplicaciones",
        "Documentos",
        "Imágenes",
        "Música",
        "Videos",
        "Descargas",
        "Papelera",
        "Windows",
        "Datos del sistema y otros",
    ]
}

impl Settings {
    /// Último desglose calculado del disco del sistema, si hay.
    pub fn storage_breakdown(&self) -> Option<dokka_model::StorageBreakdown> {
        self.storage.clone()
    }

    /// Guarda el desglose calculado por el hilo propio.
    pub fn set_storage(&mut self, breakdown: Option<dokka_model::StorageBreakdown>) {
        self.storage = breakdown;
    }

    /// Versión instalada de Dokka que muestra "Actualización de Dokka".
    pub fn dokka_version(&self) -> Option<&str> {
        self.dokka_version.as_deref()
    }

    /// Fija la versión instalada (el paquete del shell) al abrir.
    pub fn set_dokka_version(&mut self, version: String) {
        self.dokka_version = Some(version);
    }
}
