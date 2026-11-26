use dokka_scene::Size;

/// Lo que necesita la casilla, sin significado.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchFieldProps {
    pub screen: Size,
    pub query: String,
}
