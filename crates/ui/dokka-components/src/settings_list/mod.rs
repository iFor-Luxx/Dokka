//! Filas de la barra lateral de Preferencias: cuadrado de color con icono
//! blanco y nombre, con la elegida en azul y hover suave.
mod props;
mod view;

pub use props::SettingsListProps;
pub use view::{
    row_at, row_badges, row_hover, row_icons, row_labels, row_selection, search_bounds,
    sidebar_panel,
};
