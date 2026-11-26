//! Casilla de búsqueda sin saber qué es el Launchpad.

mod props;
mod view;

pub use props::SearchFieldProps;
pub use view::{search_contains, search_cursor, search_label, search_magnifier, search_panel};
