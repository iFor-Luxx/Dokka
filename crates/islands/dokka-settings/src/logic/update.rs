//! Geometría del apartado "Actualización de Dokka": tarjeta, aviso y desinstalar.
//!
//! Una tarjeta con "Dokka" y su versión, el aviso de que aún no hay de dónde
//! descargar versiones nuevas y, separada abajo, la fila de desinstalar con
//! su nota cuando no está instalado. Cabe sin bajar con la rueda.

use dokka_theme::settings as measures;

/// Altura total del contenido, sin recortar.
pub(crate) fn update_content_height() -> f32 {
    measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + measures::ABOUT_SECTION_TITLE_HEIGHT
        + measures::GROUP_GAP
        + measures::UNINSTALL_ROW_HEIGHT
        + measures::UNINSTALL_NOTE_GAP
        + measures::UNINSTALL_NOTE_HEIGHT
        + measures::GROUP_GAP
}

/// Borde superior del apartado con el desplazamiento aplicado.
pub(crate) fn update_top(scroll: f32) -> f32 {
    measures::CONTENT_TOP - scroll
}
