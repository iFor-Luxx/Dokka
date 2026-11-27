//! Panel de red que cuelga de la barra: tarjeta con nombre, tipo
//! e internet, y fila de ajustes. El icono blanco de la barra según
//! el tipo sale de `NetworkIcons::for_bar`.

mod icons;
mod props;
mod view;

pub use icons::NetworkIcons;
pub use props::{NetworkHit, NetworkType};
pub use view::{network_hit, network_panel, network_size};
