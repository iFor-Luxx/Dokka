/// Lo que Preferencias pide hacer al shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsAction {
    /// Cerrar la ventana con el semáforo rojo (Dokka sigue en marcha).
    Close,
    /// Minimizar la ventana con el semáforo amarillo.
    Minimize,
    /// Seguir igual: zoom verde, cabecera, casilla o fondo.
    Stay,
}
