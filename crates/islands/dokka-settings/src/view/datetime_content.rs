//! Página "Fecha y hora": delega en sus tarjetas editables.
//!
//! El dibujo real vive en `datetime_edit` para no superar el límite por
//! archivo. La isla solo emite acciones; el shell conecta y pide releer.

use dokka_theme::Theme;

use super::content::ContentScene;
use crate::state::Settings;

/// Las tres tarjetas editables con sus interruptores y desplegable.
pub(crate) fn datetime_content(theme: &Theme, settings: &mut Settings) -> ContentScene {
    let minus = settings.stepper_minus_drawing(theme);
    let plus = settings.stepper_plus_drawing(theme);
    super::datetime_edit::datetime_edit_content(theme, settings, &minus, &plus)
}

/// Fecha en vivo para la tarjeta, con segundos para que se vea pasar.
pub(crate) fn format_live_public(
    now: Option<crate::general::PageNow>,
    hour_24: bool,
) -> String {
    super::datetime_edit::format_live_shared(now, hour_24)
}
