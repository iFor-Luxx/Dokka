/// Lo que se tocó del panel de volumen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VolumePanelHit {
    /// La barra, con el valor elegido de 0 a 1.
    Slider(f32),
    /// Una salida de la lista, por su índice visible.
    Output(usize),
    /// El botón de silencio.
    Mute,
    /// La fila que abre los ajustes.
    Settings,
}

/// Lo que necesita el panel, sin saber de dónde sale el volumen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumePanelProps {
    /// De 0 (silencio) a 1 (máximo).
    pub level: f32,
    pub is_muted: bool,
    /// Zona bajo el cursor, si hay alguna.
    pub hovered: Option<VolumePanelHit>,
}
