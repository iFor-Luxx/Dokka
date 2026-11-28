//! Control de desinstalación desde Preferencias.
//!
//! La isla solo muestra el estado y emite una acción; el shell conecta con
//! quien lo cumple. Si Dokka no está instalado (versión de desarrollo), el
//! botón se atenúa y no responde.

use super::contract_error::ContractError;

/// Abre el desinstalador propio sin bloquear a la isla.
///
/// `can_uninstall` dice si Dokka está instalado; sin instalar, lo protegido
/// se atenúa. `open_uninstaller` abre `Desinstalar.exe --uninstall` en su
/// hilo y no cierra Dokka: lo hace el desinstalador.
pub trait UninstallControl {
    /// `true` si Dokka está instalado y se puede desinstalar.
    fn can_uninstall(&self) -> bool;
    /// Abre el desinstalador con su confirmación, sin bloquear.
    fn open_uninstaller(&self) -> Result<(), ContractError>;
}
