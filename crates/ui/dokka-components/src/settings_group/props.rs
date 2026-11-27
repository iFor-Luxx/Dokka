/// Una tarjeta de ajustes: dónde empieza y qué filas trae.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsGroupProps {
    /// Borde superior de la tarjeta en la ventana.
    pub top: f32,
    /// Etiquetas de sus filas, en orden de arriba abajo.
    pub rows: Vec<String>,
}
