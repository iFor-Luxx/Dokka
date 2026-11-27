//! Un icono que una app dejó en la bandeja de Windows.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrayItemId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayItem {
    pub id: TrayItemId,
    /// Texto que la app muestra al pasar el cursor; puede tener varias líneas o estar vacío.
    pub title: String,
}
