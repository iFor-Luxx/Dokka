/// Punta que une el panel con el sitio pulsado.
/// Nace en el borde del panel hacia el dock y usa su tinte y su borde.
/// La base mide el doble que la altura.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pointer {
    pub center_x: f32,
    /// Centro vertical para puntas laterales; sin uso abajo.
    pub center_y: f32,
    pub height: f32,
    pub side: PointerSide,
}

/// Borde del panel donde nace la punta: hacia el dock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PointerSide {
    #[default]
    Bottom,
    Left,
    Right,
}
