//! Traduce lo que hay bajo el cursor en el dock a lo que muestran los paneles.
//!
//! Con las vistas previas apagadas en Preferencias no se abre ninguna.

use dokka_dock::HoverKind;
use dokka_model::DockPosition;
use dokka_previews::{PreviewContent, PreviewRequest, PreviewWindow};
use dokka_theme::popup;

use crate::dock_session::DockSession;

pub(crate) fn preview_request(session: &DockSession, x: f32, y: f32) -> Option<PreviewRequest> {
    if !session.preferences.dock.previews {
        return None;
    }
    let apps = session.state.apps();
    let target = session.dock.hover_target(x, y, apps)?;
    let (content, app) = match target.kind {
        HoverKind::Calendar => (PreviewContent::Calendar, None),
        HoverKind::Windows { app, windows } => {
            let windows = windows
                .into_iter()
                .map(|(id, title)| PreviewWindow {
                    window: id.value(),
                    title,
                })
                .collect();
            (
                PreviewContent::Windows(windows),
                Some(app.as_str().to_owned()),
            )
        }
    };
    let size = session.dock.surface_size(apps);
    let (edge, bar_side, limit_along) = match target.position {
        DockPosition::Left => (target.icons_side + popup::GAP, target.bar_side, size.height),
        DockPosition::Right => (target.icons_side - popup::GAP, target.bar_side, size.height),
        _ => (0.0, 0.0, 0.0),
    };
    Some(PreviewRequest {
        content,
        app,
        position: target.position,
        center_x: target.center_x,
        bottom: target.icons_top - popup::GAP,
        floor: target.bar_top,
        limit: size.width,
        center_y: target.center_y,
        edge,
        bar_side,
        limit_along,
    })
}
