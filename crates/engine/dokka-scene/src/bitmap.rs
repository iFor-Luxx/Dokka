/// Imagen en memoria: BGRA de 8 bits, alfa premultiplicado, filas de arriba abajo.
#[derive(Debug, PartialEq, Eq)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}
