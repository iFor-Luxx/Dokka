/// Lo que la barra pide hacer al shell. Sin duplicar el dock:
/// el shell lo traduce a lo que ya hace `context_menu.rs` o a la plataforma.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenubarAction {
    OpenSettings,
    ShowDesktop,
    TaskManager,
    LockScreen,
    Power,
    Restart,
    Quit,
    /// Fijar el volumen de 0 a 1 (clic o arrastre en el deslizador).
    VolumeSetLevel(f32),
    VolumeToggleMute,
    OpenSoundSettings,
    OpenNetworkSettings,
    /// Activar este idioma en la app que estaba al frente.
    ChooseLanguage(u64),
    OpenLanguageSettings,
    MusicPrevious,
    MusicToggle,
    MusicNext,
    /// Abrir el resultado visible con este índice.
    SearchOpen(usize),
}
