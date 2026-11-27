/// Una tarjeta de ajustes: dónde empieza, qué filas trae y cuánto mide cada
/// una. Las filas del deslizador son más altas por sus textos bajo la barra.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsGroupProps {
    /// Borde superior de la tarjeta en la ventana.
    pub top: f32,
    /// Etiquetas de sus filas, en orden de arriba abajo.
    pub rows: Vec<String>,
    /// Alto de cada fila, en el mismo orden.
    pub heights: Vec<f32>,
}
