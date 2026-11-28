//! Movimientos calculados a partir del tiempo transcurrido.

mod bounce;
mod ease;
mod reveal;
mod spring;

pub use bounce::Bounce;
pub use ease::{ease_out_back, ease_out_cubic};
pub use reveal::Reveal;
pub use spring::Spring;
