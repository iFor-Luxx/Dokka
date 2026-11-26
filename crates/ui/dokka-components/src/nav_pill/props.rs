use dokka_scene::Rect;

/// Mitad de la píldora: anterior a la izquierda, siguiente a la derecha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavHalf {
    Previous,
    Next,
}

/// Lo que necesita la píldora, sin saber qué navega.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavPillProps {
    /// Cápsula completa.
    pub bounds: Rect,
    /// Si se puede ir atrás (mitad izquierda pulsable).
    pub can_previous: bool,
    /// Si se puede ir adelante (mitad derecha pulsable).
    pub can_next: bool,
    /// Mitad bajo el cursor, si hay alguna.
    pub hovered: Option<NavHalf>,
}
