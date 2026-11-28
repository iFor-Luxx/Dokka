//! Casilla de búsqueda sin saber qué es el Launchpad.

mod cursor;
mod props;
mod settings;
mod view;

pub use cursor::text_cursor;
pub use props::SearchFieldProps;
pub use settings::{settings_search_cursor, settings_search_text};
pub use view::{
    search_contains, search_contains_at, search_cursor, search_label, search_label_at,
    search_magnifier, search_magnifier_at, search_panel, search_panel_at,
};
