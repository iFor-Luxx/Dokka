use dokka_components::calendar_panel::calendar_size;
use dokka_scene::Rect;
use dokka_theme::menubar;

/// Calendario abierto: desfase de meses respecto al actual.
#[derive(Debug, Clone, Copy)]
pub(crate) struct OpenCalendar {
    pub(crate) month_offset: i32,
}

/// Panel del calendario: alineado a la derecha bajo el reloj, con el mismo
/// hueco que el menú y margen al borde. Solo cubre lo desplegado.
pub(crate) fn calendar_panel_for(bar_width: f32) -> Rect {
    let size = calendar_size();
    let limit = bar_width.max(size.width);
    let x =
        (bar_width - menubar::SIDE_PADDING - size.width).clamp(0.0, (limit - size.width).max(0.0));
    Rect {
        x,
        y: menubar::HEIGHT + menubar::MENU_GAP,
        width: size.width,
        height: size.height,
    }
}
