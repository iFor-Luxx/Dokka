use std::fmt;

/// Opciones elegidas en la ventana del instalador.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstallOptions {
    pub start_with_windows: bool,
    pub desktop_shortcut: bool,
}

/// Paso del trabajo que se está haciendo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStep {
    ClosingDokka,
    CopyingFiles,
    CreatingShortcuts,
    Registering,
    RemovingFiles,
}

/// Por qué no se pudo instalar o desinstalar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallFailure {
    NoSpace,
    CannotCloseDokka,
    PayloadMissing,
    AccessDenied,
    Other(String),
}

impl fmt::Display for InstallFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSpace => f.write_str("no hay espacio en el disco"),
            Self::CannotCloseDokka => f.write_str("Dokka sigue abierto y no se pudo cerrar"),
            Self::PayloadMissing => f.write_str("falta el contenido de Dokka.exe"),
            Self::AccessDenied => f.write_str("acceso denegado"),
            Self::Other(reason) => write!(f, "la instalación falló: {reason}"),
        }
    }
}

impl std::error::Error for InstallFailure {}

/// Instala, desinstala y abre Dokka en Windows, sin ninguna interfaz.
pub trait Installer: Send {
    fn install(
        &self,
        options: InstallOptions,
        progress: &mut dyn FnMut(InstallStep, f32),
    ) -> Result<(), InstallFailure>;
    fn uninstall(
        &self,
        remove_settings: bool,
        progress: &mut dyn FnMut(InstallStep, f32),
    ) -> Result<(), InstallFailure>;
    fn launch_dokka(&self) -> Result<(), InstallFailure>;
}
