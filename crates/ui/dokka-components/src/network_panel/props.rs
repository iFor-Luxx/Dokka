//! Datos del panel de red, sin saber de dónde vienen.

/// Tipo de conexión (como `NetworkKind` del modelo).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    Offline,
    Ethernet,
    Wifi,
    Other,
}

/// Fila elegible del panel de red.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkHit {
    Settings,
}
