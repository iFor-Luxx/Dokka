//! Una fila del menú: texto con marcas o línea separadora.

/// Texto de una fila, sin significado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuItem {
    pub label: String,
    pub checked: bool,
    pub has_submenu: bool,
}

/// Fila del menú, sin significado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuRow {
    Item(MenuItem),
    Separator,
}

impl MenuRow {
    /// Fila de texto sin marcas.
    pub fn item(label: impl Into<String>) -> Self {
        Self::Item(MenuItem {
            label: label.into(),
            checked: false,
            has_submenu: false,
        })
    }

    /// Fila madre que abre un submenú al pasar el mouse.
    pub fn submenu(label: impl Into<String>) -> Self {
        Self::Item(MenuItem {
            label: label.into(),
            checked: false,
            has_submenu: true,
        })
    }

    /// Fila elegible con icono de marca a la izquierda cuando está elegida.
    pub fn checked(label: impl Into<String>, checked: bool) -> Self {
        Self::Item(MenuItem {
            label: label.into(),
            checked,
            has_submenu: false,
        })
    }
}
