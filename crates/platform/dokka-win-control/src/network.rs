use dokka_contracts::{ContractError, NetworkStatus};
use dokka_model::{NetworkKind, NetworkState};
use windows::Networking::Connectivity::{
    ConnectionProfile, NetworkConnectivityLevel, NetworkInformation,
};
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize};

/// Conexión de red en uso según Windows.
///
/// Consulta síncrona y barata (`GetInternetConnectionProfile`): la isla la
/// llama cada pocos segundos sin bloquear la interfaz. Sin perfil hay
/// `Offline`, nombre vacío y sin internet, sin que sea error.
#[derive(Debug, Default)]
pub struct SystemNetwork;

impl NetworkStatus for SystemNetwork {
    fn network(&self) -> Result<NetworkState, ContractError> {
        // SAFETY: prepara Windows Runtime para este hilo; si ya lo estaba,
        // el error se ignora como en el resto de `platform/`.
        let _ = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };
        Ok(read_state())
    }
}

/// Perfil de internet actual o apagado si no hay ninguno.
fn read_state() -> NetworkState {
    let offline = NetworkState {
        kind: NetworkKind::Offline,
        name: String::new(),
        has_internet: false,
    };
    let Ok(profile) = NetworkInformation::GetInternetConnectionProfile() else {
        return offline;
    };
    NetworkState {
        kind: classify(&profile),
        name: profile
            .ProfileName()
            .map(|name| name.to_string())
            .unwrap_or_default(),
        has_internet: profile
            .GetNetworkConnectivityLevel()
            .is_ok_and(|level| level == NetworkConnectivityLevel::InternetAccess),
    }
}

/// Wifi si Windows lo dice o el adaptador es 802.11; cable si es Ethernet
/// (IANA 6); lo demás (móvil, VPN...) es `Other`.
fn classify(profile: &ConnectionProfile) -> NetworkKind {
    const IANA_ETHERNET: u32 = 6;
    const IANA_WIFI: u32 = 71;
    if profile.IsWlanConnectionProfile().unwrap_or(false) {
        return NetworkKind::Wifi;
    }
    if let Ok(adapter) = profile.NetworkAdapter()
        && let Ok(iana) = adapter.IanaInterfaceType()
    {
        if iana == IANA_WIFI {
            return NetworkKind::Wifi;
        }
        if iana == IANA_ETHERNET {
            return NetworkKind::Ethernet;
        }
    }
    NetworkKind::Other
}
