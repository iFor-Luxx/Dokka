use dokka_scene::Size;

/// Lo que necesita la casilla, sin significado.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchFieldProps {
    pub screen: Size,
    pub query: String,
    /// `true` cuando la casilla tiene el foco: muestra el cursor vertical.
    pub focused: bool,
    /// Fase del parpadeo del cursor; solo se pinta si hay foco y es `true`.
    pub cursor_visible: bool,
}
