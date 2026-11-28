/// Lo que la isla pide hacer a la app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupAction {
    /// Cerrar la ventana (X, enlace Salir o botón Cerrar).
    Close,
    /// Pasar a la pantalla de opciones.
    ShowOptions,
    /// Volver a la bienvenida.
    BackWelcome,
    /// Conmutar "Abrir al iniciar Windows".
    ToggleStart,
    /// Conmutar "Crear acceso directo".
    ToggleShortcut,
    /// Aceptar las opciones elegidas.
    OptionsDone,
    /// Empezar a instalar con las opciones vigentes.
    StartInstall,
    /// Empezar a desinstalar.
    StartUninstall,
    /// Conmutar "Borrar también mis ajustes".
    ToggleRemove,
    /// Reintentar el trabajo que falló.
    Retry,
    /// Abrir Dokka tras instalar y cerrar.
    Launch,
}
