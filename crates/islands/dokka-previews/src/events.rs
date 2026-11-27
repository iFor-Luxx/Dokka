use dokka_components::preview_card::PreviewWindow;
use dokka_model::DockPosition;

/// Qué se puede mostrar en un panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewContent {
    Calendar,
    Windows(Vec<PreviewWindow>),
}

/// Lo que hay bajo el cursor y dónde debe aparecer su panel.
/// Abajo usa `center_x`/`bottom`/`floor`/`limit` como hoy;
/// en laterales usa `center_y`/`edge`/`bar_side`/`limit_along`.
#[derive(Debug, Clone, PartialEq)]
pub struct PreviewRequest {
    pub content: PreviewContent,
    /// App dueña de las ventanas, si las hay.
    pub app: Option<String>,
    pub position: DockPosition,
    pub center_x: f32,
    /// Borde inferior del panel.
    pub bottom: f32,
    /// Hasta dónde baja la zona que mantiene el panel abierto.
    pub floor: f32,
    /// Ancho disponible.
    pub limit: f32,
    pub center_y: f32,
    /// Borde del panel hacia el dock (x a la izquierda, y abajo igual que `bottom`).
    pub edge: f32,
    /// Borde de la barra hacia dentro.
    pub bar_side: f32,
    /// Alto disponible para centrar en laterales.
    pub limit_along: f32,
}
