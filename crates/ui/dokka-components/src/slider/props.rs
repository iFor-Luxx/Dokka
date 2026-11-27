use dokka_scene::Color;

/// Lo que necesita el deslizador, sin saber qué ajusta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderProps {
    /// De 0 (mínimo) a 1 (máximo). Se recorta al pintar y al pulsar.
    pub value: f32,
    /// Si responde ahora (relleno principal) o está atenuado.
    pub active: bool,
}

/// Colores del deslizador, para reutilizarlo fuera del cristal Tahoe
/// (Preferencias le pasará los suyos).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderColors {
    /// Barra de fondo.
    pub track: Color,
    /// Relleno hasta el valor cuando está activo.
    pub fill: Color,
    /// Relleno hasta el valor cuando está atenuado.
    pub muted: Color,
}
