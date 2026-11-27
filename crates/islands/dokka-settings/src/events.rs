use dokka_model::Preferences;

/// Lo que Preferencias pide hacer al shell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsAction {
    /// Cerrar la ventana con el semáforo rojo (Dokka sigue en marcha).
    Close,
    /// Minimizar la ventana con el semáforo amarillo.
    Minimize,
    /// Seguir igual: zoom verde, cabecera, casilla o fondo.
    Stay,
    /// Guardar y aplicar los valores nuevos.
    Changed(Preferences),
    /// Ocultar sola la barra de tareas de Windows, o dejar de hacerlo.
    TaskbarAuto(bool),
}
