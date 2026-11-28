//! Qué zona de la galería cae bajo el punto.
//!
//! Primero el botón "Aceptar" y la casilla (que no cierra), luego las filas
//! de categorías y al final los controles visibles con el desplazamiento.
//! Los atenuados también se devuelven: la isla los deja sin acción.

use dokka_scene::Rect;

use super::categories::CATEGORY_COUNT;
use super::layout::{accept_button, category_row, place_sections, search_box};
use super::props::GalleryHit;

/// Zona bajo el punto, si es pulsable.
pub fn gallery_hit(
    gallery: Rect,
    selected: usize,
    query: &str,
    with_brightness: bool,
    list_scroll: f32,
    grid_scroll: f32,
    x: f32,
    y: f32,
) -> Option<GalleryHit> {
    if contains(accept_button(gallery), x, y) {
        return Some(GalleryHit::Accept);
    }
    if contains(search_box(gallery), x, y) {
        return Some(GalleryHit::Search);
    }
    for index in 0..CATEGORY_COUNT {
        if contains(category_row(gallery, index, list_scroll), x, y) {
            return Some(GalleryHit::Category(index));
        }
    }
    for section in place_sections(gallery, selected, query, with_brightness, grid_scroll) {
        for placed in &section.items {
            if contains(placed.tile, x, y) {
                return Some(GalleryHit::Control(placed.item));
            }
        }
    }
    None
}

/// `true` si el punto cae en la ventana (con el aire de la sombra).
pub fn gallery_contains(gallery: Rect, x: f32, y: f32) -> bool {
    contains(gallery, x, y)
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
