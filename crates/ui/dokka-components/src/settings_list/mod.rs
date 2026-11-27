//! Filas de la barra lateral de Preferencias: cuadrado de color con icono
//! blanco y nombre, con la elegida en azul y hover suave.
mod filtered;
mod props;
mod view;

pub use filtered::{
    filtered_badges, filtered_empty, filtered_hover, filtered_icons, filtered_labels,
    filtered_row_at, filtered_selection,
};
pub use props::SettingsListProps;
pub use view::{
    row_at, row_badges, row_hover, row_icons, row_labels, row_selection, search_bounds,
    sidebar_panel, sidebar_panel_for,
};
