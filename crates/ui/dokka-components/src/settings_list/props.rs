/// Lo que necesita la barra lateral, sin saber qué es cada sección.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsListProps {
    /// Fila elegida, con fondo azul y texto blanco.
    pub selected: usize,
    /// Fila bajo el cursor, con fondo suave.
    pub hovered: Option<usize>,
}
