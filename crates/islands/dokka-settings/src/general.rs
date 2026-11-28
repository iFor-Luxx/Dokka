//! Apartados de la sección General y botones de "Acerca de este PC".
//!
//! La barra lateral no cambia: General sigue siendo la sección 0.
//! Dentro de ella se navega entre la lista y sus apartados; la píldora
//! va y viene entre ellos y el título cambia.

/// Apartado dentro de General.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GeneralPage {
    /// Lista de apartados.
    #[default]
    Root,
    /// Ficha del equipo con datos reales.
    About,
    /// Actualización de Dokka (próximamente).
    Update,
    /// Almacenamiento (próximamente).
    Storage,
    /// Fecha y hora (próximamente).
    DateTime,
    /// Idioma y región (próximamente).
    Language,
}

impl GeneralPage {
    /// Título que se ve en la cabecera.
    pub fn title(self) -> &'static str {
        match self {
            Self::Root => "General",
            Self::About => "Acerca de este PC",
            Self::Update => "Actualización de Dokka",
            Self::Storage => "Almacenamiento",
            Self::DateTime => "Fecha y hora",
            Self::Language => "Idioma y región",
        }
    }

    /// Fila de la lista de General que lleva a cada apartado.
    pub fn from_row(row: usize) -> Option<Self> {
        match row {
            0 => Some(Self::About),
            1 => Some(Self::Update),
            2 => Some(Self::Storage),
            3 => Some(Self::DateTime),
            4 => Some(Self::Language),
            _ => None,
        }
    }
}

/// Botón de la ficha "Acerca de este PC".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AboutButton {
    /// Abre `ms-settings:display`.
    DisplaySettings,
    /// Abre `ms-settings:storagesense`.
    StorageSettings,
    /// Abre `msinfo32`.
    SystemReport,
}
