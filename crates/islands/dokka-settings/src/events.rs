use dokka_model::Preferences;

use super::general::{AboutButton, GeneralButton, GeneralPage};

/// Botón de "Acerca de este PC" que el shell debe abrir.
pub type AboutButtonAction = AboutButton;

/// Lo que Preferencias pide hacer al shell.
#[derive(Debug, Clone, PartialEq)]
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
    /// Se entró en un apartado de General: el shell pide sus datos.
    EnterGeneral(GeneralPage),
    /// Botón de "Acerca de este PC": el shell abre la página del sistema.
    OpenAbout(AboutButton),
    /// Botón de "Fecha y hora" o de "Idioma y región": el shell abre su página.
    OpenGeneral(GeneralButton),
    /// Botón "Vaciar papelera": el shell la vacía con su confirmación.
    EmptyTrash,
    /// Botón "Desinstalar Dokka…": el shell abre el desinstalador.
    Uninstall,
    /// Automáticas de Windows (`NTP` o `NoSync`).
    DateTimeAutoTime(bool),
    /// Servidor NTP validado.
    DateTimeServer(String),
    /// Fecha y hora manual con las automáticas apagadas.
    DateTimeManual(crate::datetime_edit::DateTimeEditManual),
    /// Zona automática (`tzautoupdate` 3 o 4).
    DateTimeAutoZone(bool),
    /// Zona por su clave en inglés.
    DateTimeZone(String),
}
