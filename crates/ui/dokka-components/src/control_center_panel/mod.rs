//! Centro de control estilo Tahoe: tarjetas de red, volumen con su
//! deslizador mínimo y música, más la fila de botones redondos
//! (Preferencias, escritorio, bloqueo y tareas). No conoce ninguna isla
//! ni llama a Windows: describe qué dibujar y qué zona se pulsa.

mod hit;
mod icons;
mod layout;
mod props;
mod view;

pub use hit::control_center_hit;
pub use icons::ControlCenterIcons;
pub use props::{ControlCenterHit, ControlCenterProps};
pub use view::{control_center_panel, control_center_size};
