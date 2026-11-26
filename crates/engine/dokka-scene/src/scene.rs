use crate::{Icon, Panel};

/// Todo lo que muestra una superficie.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub panel: Panel,
    pub icons: Vec<Icon>,
}
