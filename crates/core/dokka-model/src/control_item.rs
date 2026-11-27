//! Controles del Centro de control y qué conjunto está oculto.
//!
//! `ControlItem` es cada pieza en su orden de fábrica; `HiddenControls` es
//! el conjunto de ocultos como máscara de bits (`Copy`, pequeño).

/// Cada pieza del Centro de control, en su orden de fábrica.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlItem {
    Network,
    Music,
    Desktop,
    Lock,
    Settings,
    TaskView,
    Project,
    Brightness,
    Sound,
    Calculator,
    ScreenClip,
    Clock,
    TaskManager,
}

impl ControlItem {
    /// Orden de fábrica, de arriba a abajo en la rejilla llena.
    pub const FACTORY_ORDER: [Self; 13] = [
        Self::Network,
        Self::Music,
        Self::Desktop,
        Self::Lock,
        Self::Settings,
        Self::TaskView,
        Self::Project,
        Self::Brightness,
        Self::Sound,
        Self::Calculator,
        Self::ScreenClip,
        Self::Clock,
        Self::TaskManager,
    ];

    /// Nombre guardado en `menubar.hidden_controls`, en minúsculas.
    pub fn name(self) -> &'static str {
        match self {
            Self::Network => "network",
            Self::Music => "music",
            Self::Desktop => "desktop",
            Self::Lock => "lock",
            Self::Settings => "settings",
            Self::TaskView => "task_view",
            Self::Project => "project",
            Self::Brightness => "brightness",
            Self::Sound => "sound",
            Self::Calculator => "calculator",
            Self::ScreenClip => "screen_clip",
            Self::Clock => "clock",
            Self::TaskManager => "task_manager",
        }
    }

    /// Pieza con este nombre guardado, si está bien escrita.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "network" => Some(Self::Network),
            "music" => Some(Self::Music),
            "desktop" => Some(Self::Desktop),
            "lock" => Some(Self::Lock),
            "settings" => Some(Self::Settings),
            "task_view" => Some(Self::TaskView),
            "project" => Some(Self::Project),
            "brightness" => Some(Self::Brightness),
            "sound" => Some(Self::Sound),
            "calculator" => Some(Self::Calculator),
            "screen_clip" => Some(Self::ScreenClip),
            "clock" => Some(Self::Clock),
            "task_manager" => Some(Self::TaskManager),
            _ => None,
        }
    }
}

/// Conjunto de controles ocultos, como máscara de bits. Vacío por defecto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HiddenControls(u16);

impl HiddenControls {
    /// Sin nada oculto.
    pub fn empty() -> Self {
        Self(0)
    }

    /// `true` si este control está oculto.
    pub fn contains(self, item: ControlItem) -> bool {
        self.0 & (1 << (item as u16)) != 0
    }

    /// Oculta este control.
    pub fn insert(&mut self, item: ControlItem) {
        self.0 |= 1 << (item as u16);
    }

    /// Vuelve a mostrar este control.
    pub fn remove(&mut self, item: ControlItem) {
        self.0 &= !(1 << (item as u16));
    }
}
