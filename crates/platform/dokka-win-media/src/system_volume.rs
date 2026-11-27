//! Volumen del dispositivo de salida predeterminado.

use dokka_contracts::{ContractError, VolumeControl};
use dokka_model::VolumeState;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator, eConsole, eRender};
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
};

/// Volumen general de Windows sobre el dispositivo de salida predeterminado.
///
/// Cada llamada resuelve el dispositivo de nuevo: son tres llamadas COM,
/// barato para consultar cada segundo, y así sigue al dispositivo actual
/// sin vigilar cambios ni usar APIs indocumentadas (`IPolicyConfig`).
#[derive(Debug, Default)]
pub struct SystemVolume;

impl VolumeControl for SystemVolume {
    fn volume(&self) -> Result<VolumeState, ContractError> {
        let _com = init_com();
        let endpoint = default_endpoint()?;
        // SAFETY: el punto de volumen viene de `Activate` y sigue vivo aquí.
        unsafe {
            let level = endpoint.GetMasterVolumeLevelScalar().map_err(|error| {
                ContractError::Failed(format!("no se pudo leer el volumen: {error}"))
            })?;
            let muted = endpoint.GetMute().map_err(|error| {
                ContractError::Failed(format!("no se pudo leer el silencio: {error}"))
            })?;
            Ok(VolumeState {
                level,
                is_muted: muted.as_bool(),
            })
        }
    }

    fn set_level(&self, level: f32) -> Result<(), ContractError> {
        let _com = init_com();
        let endpoint = default_endpoint()?;
        // SAFETY: el punto de volumen viene de `Activate` y el contexto nulo
        // significa sin evento de guía, como documenta la API.
        unsafe {
            endpoint
                .SetMasterVolumeLevelScalar(level.clamp(0.0, 1.0), std::ptr::null())
                .map_err(|error| {
                    ContractError::Failed(format!("no se pudo cambiar el volumen: {error}"))
                })
        }
    }

    fn set_muted(&self, muted: bool) -> Result<(), ContractError> {
        let _com = init_com();
        let endpoint = default_endpoint()?;
        // SAFETY: igual que en `set_level`; el booleano se convierte solo.
        unsafe {
            endpoint.SetMute(muted, std::ptr::null()).map_err(|error| {
                ContractError::Failed(format!("no se pudo cambiar el silencio: {error}"))
            })
        }
    }
}

/// Libera COM solo si este hilo lo inició.
struct ComGuard {
    owns: bool,
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        if self.owns {
            // SAFETY: este hilo inició COM con `CoInitializeEx` y sigue vivo.
            unsafe { CoUninitialize() };
        }
    }
}

/// Inicializa COM sin romper el apartamento existente.
///
/// Si el hilo ya estaba iniciado (`S_FALSE`) o usa otro modelo
/// (`RPC_E_CHANGED_MODE`), la llamada no tiene efecto y se sigue usando COM.
fn init_com() -> ComGuard {
    // SAFETY: inicializa COM en este hilo sin opciones; el nulo pide los
    // valores por defecto y cada estructura declara su tamaño.
    let code = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    ComGuard { owns: code.0 == 0 }
}

/// Punto de volumen del dispositivo de salida predeterminado.
///
/// Enumerador documentado (`IMMDeviceEnumerator`) y render de consola;
/// nunca cambia el dispositivo ni usa `IPolicyConfig`.
fn default_endpoint() -> Result<IAudioEndpointVolume, ContractError> {
    // SAFETY: crea el enumerador de audio documentado, sin agregación.
    let enumerator: IMMDeviceEnumerator = unsafe {
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|error| {
            ContractError::Failed(format!("no se pudo enumerar el audio: {error}"))
        })?
    };
    // SAFETY: pide el dispositivo de salida predeterminado; si no hay
    // ninguno, Windows devuelve error y se informa sin pánico.
    let device = unsafe {
        enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .map_err(|error| {
                ContractError::Failed(format!("no hay dispositivo de salida: {error}"))
            })?
    };
    // SAFETY: activa la interfaz documentada de volumen sobre ese dispositivo.
    unsafe {
        let endpoint: IAudioEndpointVolume =
            device.Activate(CLSCTX_ALL, None).map_err(|error| {
                ContractError::Failed(format!("no se pudo abrir el volumen: {error}"))
            })?;
        Ok(endpoint)
    }
}
