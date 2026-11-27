//! Lista de salidas activas con la API oficial de Windows.

use dokka_contracts::{AudioOutputs, ContractError};
use dokka_model::AudioOutput;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::{
    DEVICE_STATE_ACTIVE, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator, eMultimedia, eRender,
};
use windows::Win32::System::Com::StructuredStorage::{PropVariantClear, PropVariantToStringAlloc};
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree,
    CoUninitialize, STGM_READ,
};
use windows::core::PWSTR;

/// Salidas de audio de Windows (`AudioOutputs`).
#[derive(Debug, Default)]
pub struct WinAudioOutputs;

impl AudioOutputs for WinAudioOutputs {
    fn outputs(&self) -> Result<Vec<AudioOutput>, ContractError> {
        let _com = init_com();
        let enumerator = enumerator()?;
        let default_id = default_id(&enumerator);
        let mut items = Vec::new();
        for device in active_devices(&enumerator)? {
            let id = device_id(&device)?;
            let name = friendly_name(&device).unwrap_or_else(|_| id.clone());
            items.push(AudioOutput {
                is_default: Some(&id) == default_id.as_ref(),
                id,
                name,
            });
        }
        Ok(items)
    }

    fn set_default(&self, id: &str) -> Result<(), ContractError> {
        let _com = init_com();
        let known = self.outputs()?.iter().any(|item| item.id == id);
        if !known {
            return Err(ContractError::NotFound);
        }
        super::policy::set_default_endpoint(id)
    }
}

/// Enumerador de audio documentado, sin agregación.
fn enumerator() -> Result<IMMDeviceEnumerator, ContractError> {
    // SAFETY: crea el enumerador de audio documentado, sin agregación.
    unsafe {
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|error| {
            ContractError::Failed(format!("no se pudo enumerar el audio: {error}"))
        })
    }
}

/// Dispositivos de render activos.
fn active_devices(enumerator: &IMMDeviceEnumerator) -> Result<Vec<IMMDevice>, ContractError> {
    // SAFETY: enumera los puntos de render activos; si no hay ninguno,
    // Windows devuelve colección vacía, sin pánico.
    let collection = unsafe {
        enumerator
            .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
            .map_err(|error| {
                ContractError::Failed(format!("no se pudo listar las salidas: {error}"))
            })?
    };
    // SAFETY: el recuento y cada elemento vienen de la colección viva.
    unsafe {
        let count = collection.GetCount().map_err(|error| {
            ContractError::Failed(format!("no se pudo listar las salidas: {error}"))
        })?;
        let mut devices = Vec::new();
        for index in 0..count {
            let device = collection.Item(index).map_err(|error| {
                ContractError::Failed(format!("no se pudo leer una salida: {error}"))
            })?;
            devices.push(device);
        }
        Ok(devices)
    }
}

/// Identificador del punto (`IMMDevice::GetId`).
fn device_id(device: &IMMDevice) -> Result<String, ContractError> {
    // SAFETY: `GetId` devuelve texto que libera quien llama.
    unsafe {
        let text: PWSTR = device.GetId().map_err(|error| {
            ContractError::Failed(format!("no se pudo leer una salida: {error}"))
        })?;
        let id = wide_to_string(text);
        CoTaskMemFree(Some(text.as_ptr() as *const core::ffi::c_void));
        Ok(id)
    }
}

/// Nombre descriptivo (`PKEY_Device_FriendlyName`).
fn friendly_name(device: &IMMDevice) -> Result<String, ContractError> {
    // SAFETY: la tienda y el valor viven en este bloque; se liberan antes de salir.
    unsafe {
        let store = device.OpenPropertyStore(STGM_READ).map_err(|error| {
            ContractError::Failed(format!("no se pudo leer el nombre: {error}"))
        })?;
        let mut variant = store.GetValue(&PKEY_Device_FriendlyName).map_err(|error| {
            ContractError::Failed(format!("no se pudo leer el nombre: {error}"))
        })?;
        let name = PropVariantToStringAlloc(&variant)
            .map(|text| {
                let value = wide_to_string(text);
                CoTaskMemFree(Some(text.as_ptr() as *const core::ffi::c_void));
                value
            })
            .map_err(|error| {
                ContractError::Failed(format!("no se pudo leer el nombre: {error}"))
            })?;
        let _ = PropVariantClear(&mut variant);
        if name.is_empty() {
            return Err(ContractError::Failed("salida sin nombre".to_owned()));
        }
        Ok(name)
    }
}

/// Id de la predeterminada multimedia; si no hay, nadie se marca.
fn default_id(enumerator: &IMMDeviceEnumerator) -> Option<String> {
    // SAFETY: pide el punto predeterminado; si no hay, se informa sin pánico.
    unsafe {
        let device = enumerator
            .GetDefaultAudioEndpoint(eRender, eMultimedia)
            .ok()?;
        device_id(&device).ok()
    }
}

/// Copia un texto ancho terminado en cero.
fn wide_to_string(text: PWSTR) -> String {
    // SAFETY: el puntero es texto ancho válido hasta su cero final.
    unsafe {
        let raw = text.as_ptr();
        if raw.is_null() {
            return String::new();
        }
        let mut length = 0;
        while *raw.add(length) != 0 {
            length += 1;
        }
        let slice = std::slice::from_raw_parts(raw, length);
        String::from_utf16_lossy(slice)
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
fn init_com() -> ComGuard {
    // SAFETY: inicializa COM en este hilo sin opciones; el nulo pide los
    // valores por defecto.
    let code = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    ComGuard { owns: code.0 == 0 }
}
