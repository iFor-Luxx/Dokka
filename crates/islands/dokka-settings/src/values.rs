use dokka_model::Preferences;

use super::logic::SliderTarget;
use super::painted::PaintedControl;

/// Valores reales que muestra el contenido: lo vigente más el arrastre.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ContentValues {
    pub preferences: Preferences,
    pub taskbar_auto: bool,
    /// Deslizador a medias, si hay alguno agarrado.
    pub drag: Option<SliderTarget>,
    /// Valor de 0 a 1 del deslizador agarrado.
    pub drag_value: f32,
    /// Control bajo el cursor, para repintar su hover.
    pub control: Option<PaintedControl>,
}

impl Default for ContentValues {
    fn default() -> Self {
        Self {
            preferences: Preferences::default(),
            taskbar_auto: false,
            drag: None,
            drag_value: 0.0,
            control: None,
        }
    }
}

impl ContentValues {
    /// Recuerda lo vigente antes de mostrar: el shell lo llama al abrir.
    pub(crate) fn set_values(&mut self, preferences: Preferences, taskbar_auto: bool) {
        self.preferences = preferences;
        self.taskbar_auto = taskbar_auto;
        self.drag = None;
        self.control = None;
    }
}
