//! Interruptor estilo Tahoe: pista redondeada con tirador blanco.
//!
//! No sabe qué ajusta: recibe si está encendido y dibuja sus planos y su
//! punto con los colores del tema. Sin texto ni iconos.

mod props;
mod view;

pub use props::SwitchProps;
pub use view::{switch_dot, switch_hit, switch_panels, switch_touch};
