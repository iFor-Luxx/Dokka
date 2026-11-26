use crate::{Divider, Dot, Icon, Panel, Text, Thumbnail};

/// Botones que deben verse por encima de una miniatura.
#[derive(Debug, Clone, PartialEq)]
pub struct Controls {
    pub panel: Panel,
    pub icons: Vec<Icon>,
}

/// Panel flotante sobre la escena principal.
/// Orden de pintado: panel, resaltado, líneas, puntos, iconos (punta), textos.
#[derive(Debug, Clone, PartialEq)]
pub struct Overlay {
    pub panel: Panel,
    pub highlight: Option<Panel>,
    pub dividers: Vec<Divider>,
    /// Se dibujan debajo de los textos.
    pub dots: Vec<Dot>,
    pub icons: Vec<Icon>,
    pub texts: Vec<Text>,
    pub thumbnails: Vec<Thumbnail>,
    /// La plataforma los pone en una capa propia, sobre las miniaturas.
    pub controls: Option<Controls>,
}
