use crate::{Dot, Panel, Text, Thumbnail};

/// Panel flotante sobre la escena principal.
#[derive(Debug, Clone, PartialEq)]
pub struct Overlay {
    pub panel: Panel,
    /// Se dibujan debajo de los textos.
    pub dots: Vec<Dot>,
    pub texts: Vec<Text>,
    pub thumbnails: Vec<Thumbnail>,
}
