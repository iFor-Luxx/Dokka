/// Segmentado estilo Tahoe: cápsula gris con la opción elegida en blanco.
#[derive(Debug, Clone, PartialEq)]
pub struct SegmentedProps {
    /// Textos de cada segmento, en orden de izquierda a derecha.
    pub options: Vec<String>,
    /// Índice elegido (0 al primero). La isla traduce a su ajuste.
    pub selected: usize,
}
