/// Lo que la barra pide hacer al shell. Sin duplicar el dock:
/// el shell lo traduce a lo que ya hace `context_menu.rs` o a la plataforma.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenubarAction {
    OpenSettings,
    ShowDesktop,
    TaskManager,
    LockScreen,
    Power,
    Restart,
    Quit,
}
