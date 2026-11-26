use crate::{Color, Rect};

/// Línea que separa grupos dentro de un panel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Divider {
    pub bounds: Rect,
    pub color: Color,
}
