//! Grupo de filas de ajustes estilo Tahoe: tarjeta clara con filas
//! separadas por líneas finas, etiqueta a la izquierda y control a la
//! derecha. No sabe qué ajusta cada fila: da la tarjeta, las líneas, las
//! etiquetas y la geometría de cada fila para que la isla coloque sus
//! interruptores y deslizadores. Las filas pueden medir distinto (el
//! deslizador es más alto por sus textos). Sin iconos ni texto dibujado.

mod layout;
mod props;
mod view;

pub use layout::{divider_rect, group_height, group_panel_bounds, row_bounds};
pub use props::SettingsGroupProps;
pub use view::{group_dividers, group_labels, group_panel};
