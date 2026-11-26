//! Casilla de búsqueda sin saber qué es el Launchpad.

mod props;
mod view;

pub use props::SearchFieldProps;
pub use view::{
    search_contains, search_contains_at, search_cursor, search_label, search_label_at,
    search_magnifier, search_magnifier_at, search_panel, search_panel_at,
};
