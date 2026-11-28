//! Geometría del botón "Desinstalar Dokka…" en "Actualización de Dokka".
//!
//! Una fila propia separada bajo el aviso, con su nota cuando Dokka no está
//! instalado. El golpe y el dibujo usan los mismos bordes.

use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::update::update_top;

/// Borde superior de la tarjeta de desinstalar.
pub(crate) fn uninstall_top(scroll: f32) -> f32 {
    update_top(scroll)
        + measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + measures::ABOUT_SECTION_TITLE_HEIGHT
        + measures::GROUP_GAP
}

/// Borde superior del aviso bajo el botón.
pub(crate) fn uninstall_note_top(scroll: f32) -> f32 {
    uninstall_top(scroll) + measures::UNINSTALL_ROW_HEIGHT + measures::UNINSTALL_NOTE_GAP
}

/// Fila del botón bajo el punto, si cae en ella.
pub(crate) fn uninstall_button_at(width: f32, scroll: f32, x: f32, y: f32) -> Option<()> {
    let top = uninstall_top(scroll);
    let bounds = Rect {
        x: measures::CONTENT_LEFT,
        y: top,
        width,
        height: measures::UNINSTALL_ROW_HEIGHT,
    };
    in_touch(&bounds, x, y).then_some(())
}

/// `true` si el punto cae en la fila (para no arrastrar la ventana desde ahí).
pub(crate) fn uninstall_control_at(width: f32, scroll: f32, x: f32, y: f32) -> bool {
    uninstall_button_at(width, scroll, x, y).is_some()
}

fn in_touch(touch: &Rect, x: f32, y: f32) -> bool {
    (touch.x..touch.x + touch.width).contains(&x) && (touch.y..touch.y + touch.height).contains(&y)
}
