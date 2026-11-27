//! Una fila del menú: texto con icono, marcas o línea separadora.

use std::sync::Arc;

use dokka_scene::Drawing;

/// Texto de una fila, sin significado.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub checked: bool,
    pub has_submenu: bool,
    /// Icono Lucide de la fila, en color del texto. La marca elegida
    /// usa la misma columna cuando no hay icono.
    pub icon: Option<Arc<Drawing>>,
}

/// Fila del menú, sin significado.
#[derive(Debug, Clone, PartialEq)]
pub enum MenuRow {
    Item(MenuItem),
    Separator,
}

impl MenuRow {
    /// Fila de texto sin marcas ni icono.
    pub fn item(label: impl Into<String>) -> Self {
        Self::Item(MenuItem {
            label: label.into(),
            checked: false,
            has_submenu: false,
            icon: None,
        })
    }

    /// Fila de texto con su icono Lucide a la izquierda.
    pub fn item_with_icon(label: impl Into<String>, icon: Arc<Drawing>) -> Self {
        Self::Item(MenuItem {
            label: label.into(),
            checked: false,
            has_submenu: false,
            icon: Some(icon),
        })
    }

    /// Fila madre que abre un submenú al pasar el mouse.
    pub fn submenu(label: impl Into<String>) -> Self {
        Self::Item(MenuItem {
            label: label.into(),
            checked: false,
            has_submenu: true,
            icon: None,
        })
    }

    /// Fila elegible con icono de marca a la izquierda cuando está elegida.
    pub fn checked(label: impl Into<String>, checked: bool) -> Self {
        Self::Item(MenuItem {
            label: label.into(),
            checked,
            has_submenu: false,
            icon: None,
        })
    }

    /// Añade el icono Lucide a una fila ya creada.
    pub fn with_icon(self, icon: Arc<Drawing>) -> Self {
        match self {
            Self::Item(mut item) => {
                item.icon = Some(icon);
                Self::Item(item)
            }
            Self::Separator => Self::Separator,
        }
    }
}
