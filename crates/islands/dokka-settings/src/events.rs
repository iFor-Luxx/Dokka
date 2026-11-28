use dokka_model::Preferences;

use super::general::{AboutButton, GeneralPage};

/// Botón de "Acerca de este PC" que el shell debe abrir.
pub type AboutButtonAction = AboutButton;

/// Lo que Preferencias pide hacer al shell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsAction {
    /// Cerrar la ventana con el semáforo rojo (Dokka sigue en marcha).
    Close,
    /// Minimizar la ventana con el semáforo amarillo.
    Minimize,
    /// Alternar el tamaño con el semáforo verde.
    Zoom,
    /// Seguir igual: cabecera, casilla o fondo.
    Stay,
    /// Guardar y aplicar los valores nuevos.
    Changed(Preferences),
    /// Ocultar sola la barra de tareas de Windows, o dejar de hacerlo.
    TaskbarAuto(bool),
    /// Se entró en un apartado de General: el shell pide los datos si es "Acerca".
    EnterGeneral(GeneralPage),
    /// Botón de "Acerca de este PC": el shell abre la página del sistema.
    OpenAbout(AboutButton),
}
