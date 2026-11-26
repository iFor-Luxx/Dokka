use dokka_model::AppInfo;

/// Lo que el Launchpad pide hacer al shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchpadAction {
    /// Abrir esta app y cerrar el Launchpad.
    Open(AppInfo),
    /// Cerrar sin abrir nada.
    Close,
}
