use dokka_scene::Size;

/// Lo que necesitan los puntos, sin significado.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageDotsProps {
    pub screen: Size,
    /// Cuántas páginas hay; siempre al menos una.
    pub total: usize,
    /// La que se muestra.
    pub current: usize,
}
