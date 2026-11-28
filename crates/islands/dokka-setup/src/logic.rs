//! Paso del trabajo mostrado bajo la barra de avance.

/// Paso del trabajo, en el mismo orden del contrato de la app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkStep {
    Closing,
    Copying,
    Shortcuts,
    Registering,
    Removing,
}

/// Control bajo un punto, según la pantalla.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    Round,
    Primary,
    Link,
    SwitchStart,
    SwitchShortcut,
    CheckRemove,
}

/// Frase del paso en curso, con puntos suspensivos.
pub fn step_text(step: WorkStep) -> &'static str {
    match step {
        WorkStep::Closing => "Cerrando Dokka…",
        WorkStep::Copying => "Copiando archivos…",
        WorkStep::Shortcuts => "Creando accesos directos…",
        WorkStep::Registering => "Registrando…",
        WorkStep::Removing => "Quitando archivos…",
    }
}
