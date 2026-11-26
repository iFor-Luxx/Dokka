use dokka_components::preview_card::PreviewWindow;

/// Qué se puede mostrar en un panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewContent {
    Calendar,
    Windows(Vec<PreviewWindow>),
}

/// Lo que hay bajo el cursor y dónde debe aparecer su panel.
#[derive(Debug, Clone, PartialEq)]
pub struct PreviewRequest {
    pub content: PreviewContent,
    /// App dueña de las ventanas, si las hay.
    pub app: Option<String>,
    pub center_x: f32,
    /// Borde inferior del panel.
    pub bottom: f32,
    /// Hasta dónde baja la zona que mantiene el panel abierto.
    pub floor: f32,
    /// Ancho disponible.
    pub limit: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCommand {
    Previous,
    Toggle,
    Next,
}

/// Resultado de un clic dentro de un panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewClick {
    Handled,
    Activate(u64),
    Media(MediaCommand),
}
