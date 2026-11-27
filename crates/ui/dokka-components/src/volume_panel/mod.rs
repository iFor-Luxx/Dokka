//! Panel de volumen estilo Tahoe: un solo cristal oscuro con título,
//! cápsula ancha con el número, silenciar y ajustes separados por líneas.
//!
//! No sabe de dónde sale el volumen: recibe el nivel de 0 a 1,
//! si está silenciado y la zona bajo el cursor. `tiles` vacío y cantidad
//! constante (el resaltado oculto es de tamaño cero) para no reconstruir
//! capas.

mod hit;
mod icons;
mod layout;
mod props;
mod view;

pub use hit::{volume_panel_hit, volume_slider_value};
pub use icons::VolumePanelIcons;
pub use props::{VolumePanelHit, VolumePanelProps};
pub use view::{volume_panel, volume_panel_size};
