//! Menú propio del clic derecho: filas de texto con resaltado y punta.

mod props;
mod view;

pub use props::{MenuItem, MenuRow};
pub use view::{menu_overlay, menu_row_at, menu_row_top, menu_size, submenu_overlay};
