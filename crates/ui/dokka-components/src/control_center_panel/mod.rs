//! Centro de control estilo Tahoe: rejilla de cuatro columnas con la cápsula
//! de red y dos círculos blancos a la izquierda, música a la derecha, cápsula
//! de Preferencias con dos círculos debajo, Sonido a todo lo ancho y cuatro
//! círculos al final. Una carpeta por zona (`capsules`, `music`, `circles`,
//! `slider`) con ayudas comunes en `paint`. No conoce ninguna isla ni llama
//! a Windows: describe qué dibujar y qué zona se pulsa.

mod capsules;
mod circles;
mod hit;
mod icons;
mod layout;
mod music;
mod paint;
mod props;
mod slider;
mod view;

pub use hit::control_center_hit;
pub use icons::ControlCenterIcons;
pub use props::{ControlCenterHit, ControlCenterProps};
pub use view::{control_center_panel, control_center_size};
