//! Vista grande de la ventana bajo el cursor y hueco de sus botones.

use dokka_model::WindowId;
use dokka_scene::Controls;
use dokka_win_control::window_rect;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Muestra la ventana bajo el cursor en su sitio real, sin activarla.
/// Ventanas minimizadas o sin imagen válida no muestran vista grande.
pub(crate) fn update_large_preview(session: &mut DockSession, surface: &mut Surface) {
    let Some(hovered) = session.previews.hovered_window() else {
        hide_large(session, surface);
        return;
    };
    let Some(area) = window_rect(WindowId::new(hovered)) else {
        hide_large(session, surface);
        return;
    };
    if session.renderer.show_large(hovered, area) {
        if surface.preview_show().is_err() {
            session.renderer.hide_large();
            surface.preview_hide();
        }
    } else {
        hide_large(session, surface);
    }
}

/// Esquina y tamaño que abarca el panel de botones y todos sus iconos.
pub(crate) fn above_window_area(controls: &Controls) -> [f32; 4] {
    let mut left = controls.panel.bounds.x;
    let mut top = controls.panel.bounds.y;
    let mut right = controls.panel.bounds.x + controls.panel.bounds.width;
    let mut bottom = controls.panel.bounds.y + controls.panel.bounds.height;
    for icon in &controls.icons {
        left = left.min(icon.x);
        top = top.min(icon.y);
        right = right.max(icon.x + icon.side);
        bottom = bottom.max(icon.y + icon.side);
    }
    [left, top, right - left, bottom - top]
}

fn hide_large(session: &mut DockSession, surface: &mut Surface) {
    session.renderer.hide_large();
    surface.preview_hide();
}
