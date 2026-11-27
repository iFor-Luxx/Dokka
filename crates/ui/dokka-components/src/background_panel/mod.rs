//! Apps en segundo plano estilo Tahoe: un solo cristal con título y una
//! fila por app (círculo suave con su icono real y nombre); sin apps,
//! el mensaje de vacío. Como el resto de paneles de la barra.

mod hit;
mod layout;
mod props;
mod view;

pub use hit::background_hit;
pub use props::{BackgroundApp, BackgroundHit};
pub use view::{background_panel, background_size};
