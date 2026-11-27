use dokka_scene::Color;
use dokka_theme::Theme;

/// Lo que necesita el deslizador, sin saber qué ajusta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderProps {
    /// De 0 (mínimo) a 1 (máximo). Se recorta al pintar y al pulsar.
    pub value: f32,
    /// Valor de fábrica de 0 a 1: con él (`Some`) el relleno va desde ahí
    /// hasta el valor y se ve su marca; sin él (`None`) el relleno va desde
    /// la izquierda y no hay marca. Solo Preferencias lo trae.
    pub neutral: Option<f32>,
    /// Si responde ahora (relleno principal) o está atenuado.
    pub active: bool,
}

/// Colores del deslizador según la superficie: tarjeta clara o cristal.
/// El único deslizador de Dokka; los paneles no dibujan el suyo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderColors {
    /// Barra de fondo.
    pub track: Color,
    /// Relleno hasta el valor cuando está activo.
    pub fill: Color,
    /// Relleno hasta el valor cuando está atenuado.
    pub muted: Color,
    /// Marca del valor de fábrica sobre la pista.
    pub tick: Color,
    /// Tirador blanco en cápsula.
    pub knob: Color,
    /// Borde fino del tirador sobre la tarjeta.
    pub knob_border: Color,
}

impl SliderColors {
    /// Sobre cristal (volumen y centro de control): pista del tema, relleno
    /// blanco, atenuado secundario y tirador blanco con borde sutil.
    pub(crate) fn glass(theme: &Theme) -> Self {
        Self {
            track: theme.tray.track,
            fill: theme.tray.fill,
            muted: theme.tray.secondary,
            tick: theme.tray.track,
            knob: theme.tray.fill,
            knob_border: theme.tray.track,
        }
    }
}
