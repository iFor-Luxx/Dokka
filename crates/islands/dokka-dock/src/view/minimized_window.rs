use dokka_model::WindowId;
use dokka_scene::{Rect, Thumbnail};
use dokka_theme::dock as theme;

/// Zona de la miniatura y del icono pequeño de una ventana minimizada.
/// La insignia es proporcional al hueco, así que sigue cuadrando con la escala.
/// Zona de la miniatura y del icono pequeño de una ventana minimizada.
pub(crate) fn minimized_window(window: WindowId, slot: Rect) -> (Thumbnail, Rect) {
    let side = slot.width * theme::BADGE_SIZE;
    // El icono asoma un poco fuera del hueco, como una insignia.
    let inside = side * theme::BADGE_INSIDE;
    let badge = Rect {
        x: slot.x + slot.width - inside,
        y: slot.y + slot.height - inside,
        width: side,
        height: side,
    };
    let thumbnail = Thumbnail {
        opacity: 1.0,
        window: window.value(),
        bounds: slot,
        reserved_corner: inside,
    };
    (thumbnail, badge)
}
