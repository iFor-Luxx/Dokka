//! Panel de volumen estilo Tahoe: título, deslizador con altavoz,
//! botón de silencio, separador y fila de ajustes.
//!
//! No sabe de dónde sale el volumen: recibe el nivel de 0 a 1,
//! si está silenciado y la zona bajo el cursor. La cantidad de elementos
//! es constante entre estados (el resaltado oculto es de tamaño cero)
//! para no reconstruir capas.

mod icons;
mod props;
mod view;

pub use icons::VolumePanelIcons;
pub use props::{VolumePanelHit, VolumePanelProps};
pub use view::{volume_panel, volume_panel_hit, volume_panel_size};
