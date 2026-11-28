//! Datos y ayudas de la entrada inicial del instalador.
//!
//! Los tiempos y distancias viven en las medidas del instalador de
//! `dokka-theme`; la app los pasa aquí como datos y la plataforma solo los
//! aplica. Las animaciones las hace el compositor, sin repintar por fotograma.

use windows::Foundation::TimeSpan;
use windows::UI::Composition::{Compositor, CubicBezierEasingFunction};
use windows::core::Result;
use windows_numerics::Vector2;

/// Imágenes y tiempos de la entrada, como datos desde la app.
pub struct InstallerEntryData<'a> {
    pub fondo: &'a [u8],
    pub izquierda: &'a [u8],
    pub derecha: &'a [u8],
    pub travel: f32,
    pub blur_max: f32,
    pub mountains_seconds: f32,
    pub photo_fade_seconds: f32,
    pub content_delay_seconds: f32,
    pub content_seconds: f32,
    pub rise: f32,
    pub ease_x1: f32,
    pub ease_y1: f32,
    pub ease_x2: f32,
    pub ease_y2: f32,
}

/// `true` si Windows tiene desactivadas las animaciones del área de cliente:
/// se muestra el estado final sin entrada.
pub(crate) fn animations_disabled() -> bool {
    use windows::Win32::UI::WindowsAndMessaging::{
        SPI_GETCLIENTAREAANIMATION, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW,
    };
    let mut enabled = windows::core::BOOL(0);
    // SAFETY: `enabled` es un BOOL válido donde Windows escribe el estado.
    let done = unsafe {
        SystemParametersInfoW(
            SPI_GETCLIENTAREAANIMATION,
            0,
            Some((&raw mut enabled).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .is_ok()
    };
    if !done {
        return true;
    }
    enabled.0 == 0
}

/// Segundos a la unidad del compositor (intervalos de 100 ns).
pub(crate) fn span(seconds: f32) -> TimeSpan {
    TimeSpan {
        Duration: (seconds.max(0.0) * 10_000_000.0) as i64,
    }
}

/// Tiempos de la entrada ya copiados (la plataforma no guarda las imágenes).
#[derive(Clone, Copy)]
pub(crate) struct EntryTiming {
    pub travel: f32,
    pub blur_max: f32,
    pub mountains_seconds: f32,
    pub photo_fade_seconds: f32,
    pub content_delay_seconds: f32,
    pub content_seconds: f32,
    pub rise: f32,
    pub ease_x1: f32,
    pub ease_y1: f32,
    pub ease_x2: f32,
    pub ease_y2: f32,
}

impl<'a> From<&InstallerEntryData<'a>> for EntryTiming {
    fn from(data: &InstallerEntryData<'a>) -> Self {
        Self {
            travel: data.travel,
            blur_max: data.blur_max,
            mountains_seconds: data.mountains_seconds,
            photo_fade_seconds: data.photo_fade_seconds,
            content_delay_seconds: data.content_delay_seconds,
            content_seconds: data.content_seconds,
            rise: data.rise,
            ease_x1: data.ease_x1,
            ease_y1: data.ease_y1,
            ease_x2: data.ease_x2,
            ease_y2: data.ease_y2,
        }
    }
}

/// Frenada suave con los tiempos ya copiados (para arrancar sin la entrada).
pub(crate) fn cubic_ease_timing(
    compositor: &Compositor,
    timing: &EntryTiming,
) -> Result<CubicBezierEasingFunction> {
    compositor.CreateCubicBezierEasingFunction(
        Vector2 {
            X: timing.ease_x1,
            Y: timing.ease_y1,
        },
        Vector2 {
            X: timing.ease_x2,
            Y: timing.ease_y2,
        },
    )
}
