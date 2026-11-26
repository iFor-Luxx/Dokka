use dokka_model::{AppId, AppInfo};

/// Orden que alguien pide ejecutar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    PinApp(AppInfo),
    UnpinApp(AppId),
    ActivateApp(AppId),
    MoveApp { app_id: AppId, to_index: usize },
}
