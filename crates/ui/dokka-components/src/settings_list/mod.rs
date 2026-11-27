//! Filas de la barra lateral de Preferencias: cuadrado de color con icono
//! blanco y nombre, con la elegida en azul y hover suave.
mod filtered;
mod view;

pub use filtered::{
    filtered_badges, filtered_empty, filtered_hover, filtered_icons, filtered_labels,
    filtered_row_at, filtered_selection,
};
pub use view::{row_at, row_icons, search_bounds, sidebar_panel_for};
