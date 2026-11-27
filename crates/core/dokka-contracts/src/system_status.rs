use dokka_model::{AudioOutput, KeyboardLanguage, NetworkState, VolumeState};

use crate::ContractError;

/// Volumen del dispositivo de salida predeterminado.
pub trait VolumeControl {
    fn volume(&self) -> Result<VolumeState, ContractError>;
    /// `level` de 0 a 1; fuera de rango se recorta.
    fn set_level(&self, level: f32) -> Result<(), ContractError>;
    fn set_muted(&self, muted: bool) -> Result<(), ContractError>;
}

/// Consulta de la conexión de red.
pub trait NetworkStatus {
    fn network(&self) -> Result<NetworkState, ContractError>;
}

/// Idiomas de teclado instalados.
pub trait KeyboardLanguages {
    fn languages(&self) -> Result<Vec<KeyboardLanguage>, ContractError>;
    /// Cambia el idioma de la app que está al frente.
    fn activate_language(&self, id: u64) -> Result<(), ContractError>;
}

/// Salidas de audio activas (render).
pub trait AudioOutputs {
    /// Activas con su marca de predeterminada multimedia.
    fn outputs(&self) -> Result<Vec<AudioOutput>, ContractError>;
    /// Cambia la predeterminada de consola y multimedia.
    fn set_default(&self, id: &str) -> Result<(), ContractError>;
}

/// Brillo del monitor principal por DDC/CI.
///
/// `brightness` devuelve el último valor conocido de 0 a 1.
/// `None` significa que no se puede leer (sin DDC): la tarjeta no existe.
/// `set_brightness` envía la orden sin esperar; fuera de rango se recorta.
pub trait BrightnessControl {
    fn brightness(&self) -> Option<f32>;
    fn set_brightness(&self, level: f32);
}
