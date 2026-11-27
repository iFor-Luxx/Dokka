//! Lo que muestra el Centro de control, sin saber de dónde sale.

use dokka_model::{NetworkState, NowPlayingInfo, VolumeState};

/// Estado que pinta el panel: red, volumen y reproducción en curso.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlCenterProps {
    pub network: NetworkState,
    pub volume: VolumeState,
    pub now_playing: NowPlayingInfo,
}

/// Zona pulsable del panel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlCenterHit {
    Network,
    /// Punto del deslizador de volumen, de 0 (mínimo) a 1 (máximo).
    Slider(f32),
    Mute,
    Previous,
    Toggle,
    Next,
    Preferences,
    Desktop,
    Lock,
    TaskView,
    Project,
    Calculator,
    ScreenClip,
    Clock,
    TaskManager,
}
