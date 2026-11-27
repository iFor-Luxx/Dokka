use super::logic::SliderTarget;
use super::values::ContentValues;

/// Lo que cambia lo que se ve del contenido: valores, hover y arrastre.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaintedControl {
    /// Interruptor del Dock (tarjeta 1 o 3).
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
}

/// `f32` comparable para la huella de repintado: el arrastre nunca es `NaN`.
#[derive(Debug, Clone, Copy)]
pub struct OrderedF32(pub f32);

impl PartialEq for OrderedF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl PaintedSettings {
    /// Foto de lo dibujado, con la API pública de la isla.
    pub(crate) fn of(
        selected: usize,
        hovered: Option<usize>,
        nav_hovered: Option<dokka_components::nav_pill::NavHalf>,
        traffic_hovered: bool,
        window_focused: bool,
        query: &str,
        search_focused: bool,
        can_previous: bool,
        can_next: bool,
        values: &ContentValues,
    ) -> Self {
        Self {
            selected,
            hovered,
            nav_hovered,
            traffic_hovered,
            window_focused,
            query: query.to_owned(),
            search_focused,
            can_previous,
            can_next,
            preferences: values.preferences,
            taskbar_auto: values.taskbar_auto,
            control: values.control,
            drag: values.drag.map(|target| {
                let kind = match target {
                    SliderTarget::DockScale => 0,
                    SliderTarget::DockStrength => 1,
                };
                (kind, OrderedF32(values.drag_value))
            }),
        }
    }
}
