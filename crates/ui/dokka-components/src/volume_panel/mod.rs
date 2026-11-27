//! Panel de volumen estilo Tahoe: dos piezas de cristal (tarjeta
//! "Sonido" con deslizador fino y tarjeta con la fila de ajustes).
//!
//! No sabe de dónde sale el volumen: recibe el nivel de 0 a 1,
//! si está silenciado y la zona bajo el cursor. El fondo es invisible y
//! cada pieza va en `tiles`; el deslizador no lleva tirador y el altavoz
//! de la izquierda silencia. La cantidad de elementos es constante entre
//! estados (el resaltado oculto es de tamaño cero) para no reconstruir
//! capas.

mod icons;
mod props;
mod view;

pub use icons::VolumePanelIcons;
pub use props::{VolumePanelHit, VolumePanelProps};
pub use view::{volume_panel, volume_panel_hit, volume_panel_size};
