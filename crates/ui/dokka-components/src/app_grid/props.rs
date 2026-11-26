use dokka_scene::{IconImage, Size};

/// Una app para dibujar: nombre e imagen ya resuelta.
#[derive(Debug, Clone, PartialEq)]
pub struct AppGridItem {
    pub name: String,
    pub image: Option<IconImage>,
}

/// Lo que necesita la cuadrícula, sin significado.
#[derive(Debug, Clone, PartialEq)]
pub struct AppGridProps {
    pub apps: Vec<AppGridItem>,
    pub highlighted: Option<usize>,
    pub screen: Size,
}
