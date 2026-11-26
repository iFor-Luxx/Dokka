//! Cuadrícula de apps sin saber qué es el Launchpad.

mod props;
mod view;

pub use props::{AppGridItem, AppGridProps};
pub use view::{app_at, app_highlight, app_icons, app_texts, grid_cells, page_capacity};
