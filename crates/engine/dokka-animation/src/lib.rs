//! Movimientos calculados a partir del tiempo transcurrido.

mod bounce;
mod ease;
mod spring;

pub use bounce::Bounce;
pub use ease::ease_out_cubic;
pub use spring::Spring;
