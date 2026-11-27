//! Barra fina del deslizador: fondo y relleno blanco de extremos redondos.
//! El relleno llega hasta el centro del tirador para que ambos coincidan;
//! con silencio va atenuado. Existe siempre, también al mínimo,
//! para no reconstruir capas al arrastrar.

use dokka_scene::{Panel, Rect};
use dokka_theme::Theme;

use super::VolumePanelProps;
use super::layout::knob_center;

/// Fondo y relleno, en ese orden.
pub(crate) fn track_panels(theme: &Theme, bar: Rect, props: &VolumePanelProps) -> [Panel; 2] {
    [track_panel(theme, bar), fill_panel(theme, bar, props)]
}

/// Barra de fondo con el color del cristal, extremos redondos.
fn track_panel(theme: &Theme, bar: Rect) -> Panel {
    Panel {
        bounds: bar,
        corner_radius: capped(bar.height / 2.0, bar.width, bar.height),
        border_width: 0.0,
        tint: theme.tray.track,
        border: theme.tray.track,
    }
}

/// Relleno hasta el centro del tirador: principal o atenuado con silencio.
fn fill_panel(theme: &Theme, bar: Rect, props: &VolumePanelProps) -> Panel {
    let width = (knob_center(bar, props.level) - bar.x).clamp(0.0, bar.width);
    let tint = if props.is_muted {
        theme.tray.secondary
    } else {
        theme.tray.fill
    };
    let fill = Rect {
        x: bar.x,
        y: bar.y,
        width,
        height: bar.height,
    };
    Panel {
        bounds: fill,
        corner_radius: capped(bar.height / 2.0, width, bar.height),
        border_width: 0.0,
        tint,
        border: tint,
    }
}

/// Radio recortado para no superar la mitad del lado menor.
fn capped(maximum: f32, width: f32, height: f32) -> f32 {
    if width <= 0.0 || height <= 0.0 {
        return 0.0;
    }
    maximum.min(width / 2.0).min(height / 2.0).max(0.0)
}
