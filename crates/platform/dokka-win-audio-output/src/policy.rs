//! Cambio de predeterminada con `IPolicyConfig` (sin documentar).
//!
//! Orden de la vtabla tras `IUnknown` (verificado dos veces contra el orden
//! del encargo, índices 0-2 `IUnknown`):
//! 3 GetMixFormat, 4 GetDeviceFormat, 5 ResetDeviceFormat, 6 SetDeviceFormat,
//! 7 GetProcessingPeriod, 8 SetProcessingPeriod, 9 GetShareMode,
//! 10 SetShareMode, 11 GetPropertyValue, 12 SetPropertyValue,
//! 13 SetDefaultEndpoint(PCWSTR id, ERole), 14 SetEndpointVisibility.
//! Solo se llama a la ranura 13; el resto solo ocupa su sitio.

use std::ffi::c_void;

use dokka_contracts::ContractError;
use windows::Win32::Media::Audio::{ERole, eConsole, eMultimedia};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};
use windows::core::{GUID, HRESULT, IUnknown, Interface as _, PCWSTR};

/// `CPolicyConfigClient`: crea el objeto que expone `IPolicyConfig`.
const POLICY_CONFIG_CLSID: GUID = GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

/// `IPolicyConfig`: hay que pedirla; la vtabla de `IUnknown` no tiene la ranura.
const POLICY_CONFIG_IID: GUID = GUID::from_u128(0xf8679f50_850a_41cf_9c72_430f290290c8);

type SetDefaultFn = unsafe extern "system" fn(*mut c_void, PCWSTR, ERole) -> HRESULT;

/// Fija `id` como predeterminada de consola y multimedia.
pub(crate) fn set_default_endpoint(id: &str) -> Result<(), ContractError> {
    // SAFETY: crea el objeto documentado `CPolicyConfigClient` sin agregar.
    let unknown: IUnknown = unsafe {
        CoCreateInstance(&POLICY_CONFIG_CLSID, None, CLSCTX_ALL).map_err(|error| {
            ContractError::Failed(format!("no se pudo abrir la política de audio: {error}"))
        })?
    };
    let mut raw: *mut c_void = std::ptr::null_mut();
    // SAFETY: `raw` es escribible; `QueryInterface` lo rellena solo si existe.
    let queried = unsafe { unknown.query(&POLICY_CONFIG_IID, &mut raw) };
    if queried.is_err() || raw.is_null() {
        // Windows cambió la interfaz: se informa sin llamar a nada.
        return Err(ContractError::Failed(
            "este Windows no permite cambiar la salida".to_owned(),
        ));
    }
    // SAFETY: `raw` es una referencia propia devuelta por `QueryInterface`;
    // al soltar `policy` se libera una sola vez.
    let policy = unsafe { IUnknown::from_raw(raw) };
    let wide: Vec<u16> = id.encode_utf16().chain(std::iter::once(0)).collect();
    for role in [eConsole, eMultimedia] {
        // SAFETY: el objeto sigue vivo; la ranura 13 es
        // `SetDefaultEndpoint(PCWSTR, ERole)` según el orden de arriba y el
        // primer parámetro es el propio objeto (`this`).
        let code = unsafe {
            let object = policy.as_raw() as *const *const SetDefaultFn;
            let slot = (*object).add(13);
            (*slot)(policy.as_raw(), PCWSTR::from_raw(wide.as_ptr()), role)
        };
        if let Err(error) = code.ok() {
            return Err(ContractError::Failed(format!(
                "no se pudo cambiar la salida: {error}"
            )));
        }
    }
    Ok(())
}
