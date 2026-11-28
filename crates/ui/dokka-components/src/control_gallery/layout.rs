//! Dónde cae cada pieza de la galería, única fuente para dibujo y golpe.
//!
//! La ventana va centrada sin tapar el centro (`gallery_rect` en el tema).
//! La columna trae la casilla y las filas; la zona derecha reparte los
//! controles en flujo por secciones con su título; el pie queda fijo.
//! Las dos zonas se desplazan con la rueda sin salirse de su caja.

use dokka_model::ControlItem;
use dokka_scene::Rect;
use dokka_theme::control_gallery as measures;

use super::super::control_center_panel::preview_size;
use super::categories::{CATEGORY_COUNT, visible_items};

/// Casilla "Buscar controles" arriba de la columna.
pub fn search_box(gallery: Rect) -> Rect {
    Rect {
        x: gallery.x + measures::PADDING,
        y: gallery.y + measures::PADDING,
        width: measures::LEFT_WIDTH - measures::PADDING * 2.0,
        height: measures::SEARCH_HEIGHT,
    }
}

/// Caja visible de las filas de categorías (lo que sobra se recorta).
pub fn list_view(gallery: Rect) -> Rect {
    let top = gallery.y + measures::PADDING + measures::SEARCH_HEIGHT + measures::SECTION_GAP;
    Rect {
        x: gallery.x,
        y: top,
        width: measures::LEFT_WIDTH,
        height: (gallery.y + gallery.height - measures::FOOTER_HEIGHT - top).max(0.0),
    }
}

/// Fila de esta categoría con el desplazamiento aplicado.
pub fn category_row(gallery: Rect, index: usize, list_scroll: f32) -> Rect {
    let view = list_view(gallery);
    Rect {
        x: view.x + measures::ROW_MARGIN,
        y: view.y + index as f32 * measures::ROW_HEIGHT - list_scroll,
        width: view.width - measures::ROW_MARGIN * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Alto de todas las filas, para recortar el desplazamiento.
pub fn list_height() -> f32 {
    CATEGORY_COUNT as f32 * measures::ROW_HEIGHT
}

/// Desplazamiento recortado de la columna.
pub fn clamp_list_scroll(gallery: Rect, scroll: f32) -> f32 {
    scroll
        .max(0.0)
        .min((list_height() - list_view(gallery).height).max(0.0))
}

/// Caja visible de las secciones (sobre el pie fijo).
pub fn grid_view(gallery: Rect) -> Rect {
    measures::grid_box(gallery)
}

/// Línea del pie fijo.
pub fn footer_line(gallery: Rect) -> Rect {
    Rect {
        x: gallery.x + measures::PADDING,
        y: gallery.y + gallery.height - measures::FOOTER_HEIGHT,
        width: gallery.width - measures::PADDING * 2.0,
        height: measures::DIVIDER_HEIGHT,
    }
}

/// Botón azul "Aceptar" a la derecha del pie.
pub fn accept_button(gallery: Rect) -> Rect {
    Rect {
        x: gallery.x + gallery.width - measures::PADDING - measures::ACCEPT_WIDTH,
        y: gallery.y + gallery.height - measures::FOOTER_HEIGHT
            + (measures::FOOTER_HEIGHT - measures::ACCEPT_HEIGHT) / 2.0,
        width: measures::ACCEPT_WIDTH,
        height: measures::ACCEPT_HEIGHT,
    }
}

/// Control colocado con su etiqueta debajo.
#[derive(Debug, Clone)]
pub struct PlacedItem {
    pub item: ControlItem,
    pub tile: Rect,
    /// Centro vertical del nombre bajo la pieza.
    pub label_y: f32,
    pub category: usize,
}

/// Sección con su título y sus controles ya colocados.
#[derive(Debug, Clone)]
pub struct PlacedSection {
    pub category: usize,
    pub title: Rect,
    pub items: Vec<PlacedItem>,
}

/// Secciones con sus controles en flujo, con el desplazamiento y recortadas
/// a la caja visible para que nada se salga del panel.
pub fn place_sections(
    gallery: Rect,
    selected: usize,
    query: &str,
    with_brightness: bool,
    grid_scroll: f32,
) -> Vec<PlacedSection> {
    let (mut sections, view) = lay_sections(gallery, selected, query, with_brightness, grid_scroll);
    // Recorta lo que cae fuera de la caja visible.
    for section in &mut sections {
        section.items.retain(|placed| {
            placed.tile.y + placed.tile.height + measures::LABEL_GAP + measures::LABEL_HEIGHT
                >= view.y
                && placed.tile.y <= view.y + view.height
        });
    }
    sections.retain(|section| {
        !section.items.is_empty()
            || (section.title.y + section.title.height >= view.y
                && section.title.y <= view.y + view.height)
    });
    sections
}

/// Alto de todo el contenido derecho, para recortar el desplazamiento.
/// Se mide sin recortar: el alto no depende del sitio ni del desplazamiento.
pub fn grid_height(selected: usize, query: &str, with_brightness: bool) -> f32 {
    let gallery = Rect {
        x: 0.0,
        y: 0.0,
        width: measures::WIDTH,
        height: measures::HEIGHT,
    };
    let (placed, view) = lay_sections(gallery, selected, query, with_brightness, 0.0);
    let mut bottom = view.y + measures::PADDING;
    for section in &placed {
        for item in &section.items {
            bottom = bottom.max(
                item.tile.y + item.tile.height + measures::LABEL_GAP + measures::LABEL_HEIGHT,
            );
        }
        bottom = bottom.max(section.title.y + section.title.height + measures::LABEL_GAP);
    }
    bottom - (view.y + measures::PADDING) + measures::PADDING * 2.0
}

/// Reparte las secciones en flujo y devuelve también la caja visible.
fn lay_sections(
    gallery: Rect,
    selected: usize,
    query: &str,
    with_brightness: bool,
    grid_scroll: f32,
) -> (Vec<PlacedSection>, Rect) {
    let view = grid_view(gallery);
    let left = view.x + measures::PADDING;
    let right = view.x + view.width - measures::PADDING;
    let mut top = view.y + measures::PADDING - grid_scroll;
    let mut sections = Vec::new();
    let mut order: Vec<usize> = Vec::new();
    for (category, _) in visible_items(selected, query, with_brightness) {
        if !order.contains(&category) {
            order.push(category);
        }
    }
    for category in order {
        let title = Rect {
            x: left,
            y: top,
            width: (right - left).max(0.0),
            height: measures::SECTION_TITLE_HEIGHT,
        };
        top += measures::SECTION_TITLE_HEIGHT + measures::LABEL_GAP;
        let mut items = Vec::new();
        let mut x = left;
        let mut row_h = 0.0;
        for (_, item) in visible_items(selected, query, with_brightness)
            .into_iter()
            .filter(|(slot, _)| *slot == category)
        {
            let size = preview_size(item);
            let cell_h = size.height + measures::LABEL_GAP + measures::LABEL_HEIGHT;
            if x + size.width > right && x > left {
                top += row_h + measures::SECTION_GAP;
                x = left;
                row_h = 0.0;
            }
            let tile = Rect {
                x,
                y: top,
                width: size.width,
                height: size.height,
            };
            items.push(PlacedItem {
                item,
                tile,
                label_y: top + size.height + measures::LABEL_GAP + measures::LABEL_HEIGHT / 2.0,
                category,
            });
            x += size.width + measures::SECTION_GAP;
            row_h = row_h.max(cell_h);
        }
        top += row_h + measures::SECTION_GAP;
        sections.push(PlacedSection {
            category,
            title,
            items,
        });
    }
    (sections, view)
}

/// Desplazamiento recortado de la zona derecha.
pub fn clamp_grid_scroll(
    gallery: Rect,
    selected: usize,
    query: &str,
    with_brightness: bool,
    scroll: f32,
) -> f32 {
    let free = grid_height(selected, query, with_brightness) - grid_view(gallery).height;
    scroll.max(0.0).min(free.max(0.0))
}
