//! Geometría del apartado "Actualización de Dokka": tarjeta y aviso.
//!
//! Una sola tarjeta con "Dokka" y su versión, y el aviso de que aún no hay
//! de dónde descargar versiones nuevas. Cabe sin bajar con la rueda.

use dokka_theme::settings as measures;

/// Altura total del contenido, sin recortar.
pub(crate) fn update_content_height() -> f32 {
    measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + measures::ABOUT_SECTION_TITLE_HEIGHT
        + measures::GROUP_GAP
}

/// Borde superior del apartado con el desplazamiento aplicado.
pub(crate) fn update_top(scroll: f32) -> f32 {
    measures::CONTENT_TOP - scroll
}
