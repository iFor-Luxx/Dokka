//! Traduce lo que hay bajo el cursor en el dock a lo que muestran los paneles.
//!
//! Con las vistas previas apagadas en Preferencias no se abre ninguna.

use dokka_dock::HoverKind;
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
    Some(PreviewRequest {
        content,
        app,
        center_x: target.center_x,
        bottom: target.icons_top - popup::GAP,
        floor: target.bar_top,
        limit: session.dock.surface_size(apps).width,
    })
}
