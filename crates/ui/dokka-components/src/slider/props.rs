/// Lo que necesita el deslizador, sin saber qué ajusta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderProps {
    /// De 0 (mínimo) a 1 (máximo). Se recorta al pintar y al pulsar.
    pub value: f32,
    /// Si responde ahora (relleno azul) o está atenuado (relleno gris).
    pub active: bool,
}
