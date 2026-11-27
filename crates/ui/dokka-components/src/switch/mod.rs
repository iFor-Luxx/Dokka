//! Interruptor estilo Tahoe: pista redondeada con tirador alargado.
//!
//! No sabe qué ajusta: recibe si está encendido y dibuja su pista y su
//! tirador en cápsula con los colores del tema. Sin texto ni iconos.

mod props;
mod view;

pub use props::SwitchProps;
pub use view::{switch_knob_bounds, switch_panels, switch_touch};
