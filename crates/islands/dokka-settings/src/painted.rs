use super::logic::SliderTarget;
use super::values::ContentValues;

/// Lo que cambia lo que se ve del contenido: valores, hover y arrastre.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaintedControl {
    /// Interruptor del Dock (0 ampliación, 1 ocultado, 2 animar, 3 genio,
    /// 4 indicadores, 5 vistas previas).
    DockSwitch(usize),
    /// Interruptor de la barra superior (fila 0-6).
    MenubarSwitch(usize),
    /// Interruptor de la barra de tareas.
    TaskbarSwitch,
    /// Deslizador del Dock: 0 tamaño, 1 ampliación.
    DockSlider(u8),
}

/// Todo lo que cambia lo que se ve: si no cambió, el shell no repinta.
#[derive(Debug, Clone, PartialEq)]
pub struct PaintedSettings {
    pub selected: usize,
    pub hovered: Option<usize>,
    pub nav_hovered: Option<dokka_components::nav_pill::NavHalf>,
    pub traffic_hovered: bool,
    pub window_focused: bool,
    pub zoomed: bool,
    pub query: String,
    pub search_focused: bool,
    pub can_previous: bool,
    pub can_next: bool,
    /// Valores reales del contenido.
    pub preferences: dokka_model::Preferences,
    pub taskbar_auto: bool,
    /// Control bajo el cursor, si hay alguno.
    pub control: Option<PaintedControl>,
    /// Deslizador a medias: cuál (0 tamaño, 1 ampliación) y su valor de 0 a 1.
    pub drag: Option<(u8, OrderedF32)>,
    /// Fila de la barra a medias: de dónde a dónde y a qué altura flotante.
    pub reorder: Option<PaintedReorder>,
}

/// Fila arrastrada en la huella: hueco de origen, destino y altura.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaintedReorder {
    pub from: usize,
    pub target: usize,
    pub height: OrderedF32,
}

/// `f32` comparable para la huella de repintado: el arrastre nunca es `NaN`.
#[derive(Debug, Clone, Copy)]
pub struct OrderedF32(pub f32);

impl PartialEq for OrderedF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

/// Deslizador a medias en la forma que guarda la huella.
pub(crate) fn painted_drag(values: &ContentValues) -> Option<(u8, OrderedF32)> {
    values.drag.map(|target| {
        let kind = match target {
            SliderTarget::DockScale => 0,
            SliderTarget::DockStrength => 1,
        };
        (kind, OrderedF32(values.drag_value))
    })
}

/// Fila de la barra a medias: solo si ya pasó el umbral y se mueve.
pub(crate) fn painted_reorder(values: &ContentValues) -> Option<PaintedReorder> {
    let drag = values.reorder?;
    if !drag.is_active() {
        return None;
    }
    Some(PaintedReorder {
        from: drag.from(),
        target: drag.target(),
        height: OrderedF32(drag.current_y()),
    })
}
