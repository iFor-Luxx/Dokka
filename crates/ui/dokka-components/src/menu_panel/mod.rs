//! Menú propio del clic derecho: filas de texto con resaltado y punta.

mod chevron;
mod props;
mod view;

pub use chevron::MenuChevrons;
pub use props::{MenuItem, MenuRow};
pub use view::{menu_overlay, menu_row_at, menu_row_top, menu_size, submenu_overlay};
