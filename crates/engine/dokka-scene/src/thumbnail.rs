use crate::Rect;

/// Vista en vivo de una ventana de otra app.
///
/// La plataforma la dibuja por encima de todo lo demás de la escena, así que lo que
/// deba verse junto a ella tiene que ir en la esquina reservada.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Thumbnail {
    /// Identificador de la ventana mostrada.
    pub window: u64,
    /// Hueco disponible; la miniatura se ajusta sin deformarse.
    pub bounds: Rect,
    /// Lado del cuadrado libre en la esquina inferior derecha.
    pub reserved_corner: f32,
}
