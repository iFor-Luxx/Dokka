//! Lo que muestra el Centro de control, sin saber de dónde sale.

use std::sync::Arc;

use dokka_model::{HiddenControls, NetworkState, NowPlayingInfo, VolumeState};
use dokka_scene::Bitmap;

/// Estado que pinta el panel: red, volumen, reproducción en curso, carátula,
/// brillo y qué controles están ocultos.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlCenterProps {
    pub network: NetworkState,
    pub volume: VolumeState,
    pub now_playing: NowPlayingInfo,
    /// Carátula real; sin ella la tarjeta muestra su marcador.
    pub artwork: Option<Arc<Bitmap>>,
    /// Brillo del monitor de 0 a 1; `None` sin DDC: sin tarjeta.
    pub brightness: Option<f32>,
    /// Controles ocultos; no se dibujan ni responden.
    pub hidden: HiddenControls,
}

/// Zona pulsable del panel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlCenterHit {
    Network,
    /// Punto del deslizador de volumen, de 0 (mínimo) a 1 (máximo).
    Slider(f32),
    /// Punto del deslizador de brillo, de 0 (mínimo) a 1 (máximo).
    Brightness(f32),
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
    /// Burbuja "Editar controles" bajo la rejilla (solo sin editar).
    Edit,
    /// Botón "−" de este control (solo al editar): se oculta al momento.
    Remove(dokka_model::ControlItem),
}
