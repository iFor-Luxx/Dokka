//! Una fila del menú: texto o línea separadora.

/// Fila del menú, sin significado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuRow {
    Item(String),
    Separator,
}
