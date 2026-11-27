use dokka_components::calendar_panel::calendar_size;
use dokka_scene::Rect;
use dokka_theme::menubar;

/// Calendario abierto: desfase de meses y mayor número de semanas visto
/// (igual que en el dock, el panel no se encoge al navegar).
#[derive(Debug, Clone, Copy)]
pub(crate) struct OpenCalendar {
    pub(crate) month_offset: i32,
    pub(crate) calendar_weeks: u16,
}

/// Panel del calendario: alineado a la derecha bajo el reloj, con el mismo
/// hueco que el menú y margen al borde. Solo cubre lo desplegado.
pub(crate) fn calendar_panel_for(bar_width: f32, weeks: u16) -> Rect {
    let size = calendar_size(weeks);
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
