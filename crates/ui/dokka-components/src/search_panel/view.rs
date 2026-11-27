use dokka_scene::{Overlay, Panel, Rect, Size};
use dokka_theme::{Theme, search as measures};

use super::content::{panel_icons, panel_texts};
use super::rows::{field_cursor, result_highlight, row_at};
use super::{SearchHit, SearchPanelIcons, SearchPanelProps};

/// Alto del panel para lo que se muestra (hasta siete filas).
pub fn search_panel_size(props: &SearchPanelProps) -> Size {
    let visible = measures::visible_results(props.results.len());
    Size {
        width: measures::WIDTH,
        height: measures::panel_height(visible),
    }
}

/// Dos piezas de cristal separadas: cápsula de búsqueda y tarjeta de
/// resultados. Capas siempre iguales: panel invisible, dos piezas, un
/// resaltado, una línea de cursor, nueve textos y ocho iconos; lo que no
/// se ve va de tamaño cero o vacío.
pub fn search_panel_panel(
    theme: &Theme,
    drawings: &SearchPanelIcons,
    bounds: Rect,
    props: &SearchPanelProps,
) -> Overlay {
    let visible = measures::visible_results(props.results.len());
    let search = measures::search_box(bounds);
    let results = measures::results_box(bounds);
    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::RESULTS_RADIUS,
            border_width: 0.0,
            tint: theme.tray.clear,
            border: theme.tray.clear,
        },
        tiles: vec![
            Panel {
                bounds: search,
                corner_radius: measures::SEARCH_RADIUS.min(search.height / 2.0),
                border_width: measures::RESULTS_BORDER_WIDTH,
                tint: theme.tray.tile,
                border: theme.tray.tile_border,
            },
            Panel {
                bounds: results,
                corner_radius: measures::RESULTS_RADIUS,
                border_width: measures::RESULTS_BORDER_WIDTH,
                tint: theme.tray.tile,
                border: theme.tray.tile_border,
            },
        ],
        pill: None,
        highlight: Some(result_highlight(theme, bounds, props, visible)),
        flat_panels: Vec::new(),
        dividers: vec![field_cursor(theme, &search, props)],
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
pub fn search_panel_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    props: &SearchPanelProps,
) -> Option<SearchHit> {
    let visible = measures::visible_results(props.results.len());
    row_at(bounds, x, y, visible).map(SearchHit::Result)
}
