use dokka_model::{AppId, AppInfo, OpenWindow, WindowId};

/// Hecho que ya ocurrió y que puede cambiar el estado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    AppPinned(AppInfo),
    AppUnpinned(AppId),
    AppMoved {
        app_id: AppId,
        to_index: usize,
    },
    /// También se usa cuando cambia una ventana ya conocida.
    WindowOpened(OpenWindow),
    WindowClosed(WindowId),
}
