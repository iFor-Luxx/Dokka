//! Panel de volumen estilo Tahoe: título con porcentaje, barra fina con
//! tirador entre dos altavoces, sección de salidas si hay, silenciar y
//! ajustes separados por líneas.
//!
//! No sabe de dónde sale el volumen: recibe el nivel de 0 a 1,
//! si está silenciado, las salidas activas y la zona bajo el cursor.
//! `tiles` vacío; el resaltado oculto es de tamaño cero para no
//! reconstruir capas. La geometría de la barra vive en `layout` y la
//! comparten el dibujo (`track`, `knob`), el golpe y el arrastre (`hit`).

mod hit;
mod icons;
mod knob;
mod layout;
mod props;
mod rows;
mod track;
mod view;

pub use hit::{volume_panel_hit, volume_slider_value};
pub use icons::VolumePanelIcons;
pub use props::{VolumePanelHit, VolumePanelProps};
pub use view::{volume_panel, volume_panel_size};
