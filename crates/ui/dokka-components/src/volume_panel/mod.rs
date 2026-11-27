//! Panel de volumen estilo Tahoe: título con porcentaje, barra fina con
//! tirador en cápsula entre dos altavoces, sección de salidas si hay,
//! silenciar y ajustes separados por líneas.
//!
//! No sabe de dónde sale el volumen: recibe el nivel de 0 a 1,
//! si está silenciado, las salidas activas y la zona bajo el cursor.
//! `tiles` vacío; el resaltado oculto es de tamaño cero para no
//! reconstruir capas. El deslizador es el componente `slider`
//! (cuatro planos, sin puntos): la geometría vive en `layout` y el dibujo,
//! el golpe y el arrastre salen de él.

mod hit;
mod icons;
mod layout;
mod props;
mod rows;
mod view;

pub use hit::{volume_panel_hit, volume_slider_value};
pub use icons::VolumePanelIcons;
pub use props::{VolumePanelHit, VolumePanelProps};
pub use view::{volume_panel, volume_panel_size};
