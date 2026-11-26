use crate::{Divider, Dot, Icon, Panel, Thumbnail};

/// Todo lo que muestra una superficie.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub panel: Panel,
    pub icons: Vec<Icon>,
    pub dots: Vec<Dot>,
    pub dividers: Vec<Divider>,
    pub thumbnails: Vec<Thumbnail>,
}
