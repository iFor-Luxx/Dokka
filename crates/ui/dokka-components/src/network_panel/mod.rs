//! Panel de red que cuelga de la barra, estilo Tahoe: un solo cristal
//! oscuro con título, estado con círculo e internet, línea y ajustes.
//! El círculo activo es blanco con icono azul; el apagado, translúcido
//! con icono blanco. El icono blanco de la barra sale de `for_bar`.

mod icons;
mod props;
mod view;

pub use icons::NetworkIcons;
pub use props::{NetworkHit, NetworkType};
pub use view::{network_hit, network_panel, network_size};
