//! Panel buscador tipo Spotlight: casilla grande con lupa y texto,
//! hasta siete resultados con icono y resaltado, y "Sin resultados".

mod content;
mod filter;
mod icons;
mod props;
mod rows;
mod view;

pub use filter::filter_search_order;
pub use icons::SearchPanelIcons;
pub use props::{SearchHit, SearchPanelItem, SearchPanelProps};
pub use view::{search_panel_hit, search_panel_panel, search_panel_size};
