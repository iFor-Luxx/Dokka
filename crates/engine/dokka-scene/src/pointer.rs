/// Punta que une el panel con el sitio pulsado.
/// Nace en el borde inferior del panel y usa su tinte y su borde.
/// La base mide el doble que la altura.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pointer {
    pub center_x: f32,
    pub height: f32,
}
