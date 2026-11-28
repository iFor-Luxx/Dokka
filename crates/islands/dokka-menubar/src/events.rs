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
    /// Fijar el brillo DDC de 0 a 1 (clic o arrastre en su deslizador).
    BrightnessSetLevel(f32),
    OpenSoundSettings,
    /// Elegir la salida con este índice de la lista guardada.
    ChooseOutput(usize),
    OpenNetworkSettings,
    /// Activar este idioma en la app que estaba al frente.
    ChooseLanguage(u64),
    OpenLanguageSettings,
    MusicPrevious,
    MusicToggle,
    MusicNext,
    TaskView,
    Project,
    Calculator,
    ScreenClip,
    Clock,
    /// Ocultar controles del centro: los guarda el shell por el mismo camino
    /// que Preferencias y actualiza su copia para no devolverlos.
    HiddenControlsChanged(dokka_model::HiddenControls),
    /// Abrir el resultado visible con este índice.
    SearchOpen(usize),
    /// Clic izquierdo en una app en segundo plano: activarla.
    TrayActivate(dokka_model::TrayItemId),
    /// Clic derecho en una app en segundo plano: su menú propio.
    TrayMenu(dokka_model::TrayItemId),
}
