//! Galería de controles del modo editar, estilo Tahoe.
//!
//! Ventana flotante de cristal claro con la columna de categorías a la
//! izquierda y las secciones a la derecha, cada control dibujado igual que
//! en el centro (mismo tamaño y mismo dibujo) con su nombre debajo. Abajo
//! queda fijo el aviso con el botón "Aceptar". No sabe de la isla: recibe
//! qué dibujar y dice qué zona se pulsa.

mod categories;
mod chrome;
mod hit;
mod layout;
mod props;
mod view;

pub use categories::{
    CATEGORY_COUNT, category_items, category_name, item_category, item_name, visible_items,
};
pub use hit::{gallery_contains, gallery_hit};
pub use layout::{
    PlacedItem, PlacedSection, accept_button, category_row, clamp_grid_scroll, clamp_list_scroll,
    footer_line, grid_height, grid_view, list_height, list_view, place_sections, search_box,
};
pub use props::{ControlGalleryProps, GalleryHit};
pub use view::gallery_panel;
