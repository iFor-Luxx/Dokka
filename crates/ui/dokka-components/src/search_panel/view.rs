use dokka_scene::{Overlay, Rect, Size};
use dokka_theme::{Theme, search as measures};

use super::{SearchHit, SearchPanelIcons, SearchPanelProps};
use super::content::{panel_icons, panel_texts};
use super::rows::{field_cursor, result_highlight, row_at, row_separator};
use crate::popup_panel::popup_panel;
use crate::search_field::search_panel_at;

/// Alto del panel para lo que se muestra (hasta siete filas).
pub fn search_panel_size(props: &SearchPanelProps) -> Size {
    let visible = measures::visible_results(props.results.len());
    Size {
        width: measures::WIDTH,
        height: measures::panel_height(visible),
    }
}

/// Cristal con casilla grande, filas y resaltado. Capas siempre iguales:
/// un plano para la casilla, un resaltado, dos líneas, nueve textos y ocho
/// iconos; lo que no se ve va de tamaño cero o vacío.
pub fn search_panel_panel(
    theme: &Theme,
    drawings: &SearchPanelIcons,
    bounds: Rect,
    props: &SearchPanelProps,
) -> Overlay {
    let visible = measures::visible_results(props.results.len());
    let search = measures::search_box(bounds);
    Overlay {
        panel: popup_panel(theme, bounds),
        pill: None,
        highlight: Some(result_highlight(theme, bounds, props, visible)),
        flat_panels: vec![search_panel_at(
            theme.panel.highlight,
            theme.panel.border,
            search,
            measures::SEARCH_RADIUS,
            measures::SEARCH_BORDER_WIDTH,
        )],
        dividers: vec![field_cursor(theme, &search, props), row_separator(theme, bounds)],
        dots: Vec::new(),
        pointer: None,
        texts: panel_texts(theme, bounds, &search, props, visible),
        icons: panel_icons(theme, drawings, bounds, &search, props, visible),
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Fila bajo el punto, si es visible.
pub fn search_panel_hit(bounds: Rect, x: f32, y: f32, props: &SearchPanelProps) -> Option<SearchHit> {
    let visible = measures::visible_results(props.results.len());
    row_at(bounds, x, y, visible).map(SearchHit::Result)
}
