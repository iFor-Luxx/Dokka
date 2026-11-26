use crate::{Divider, Dot, Icon, Label, Overlay, Panel, Text, Thumbnail};

/// Todo lo que muestra una superficie.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub panel: Panel,
    pub icons: Vec<Icon>,
    pub dots: Vec<Dot>,
    pub dividers: Vec<Divider>,
    pub thumbnails: Vec<Thumbnail>,
    pub labels: Vec<Label>,
    /// Textos sueltos, como los nombres bajo los iconos del Launchpad.
    pub texts: Vec<Text>,
    /// Resaltado suave detrás del elemento bajo el cursor.
    pub highlight: Option<Panel>,
    pub overlay: Option<Overlay>,
}
