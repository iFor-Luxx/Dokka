//! Centro de control estilo Tahoe: rejilla de cuatro columnas que reparte
//! los controles visibles en el orden de fábrica (red, música, círculos,
//! Preferencias, Pantalla a todo lo ancho si hay brillo DDC, Sonido igual y
//! cuatro círculos al reacomodarse). Una carpeta por zona (`capsules`,
//! `music`, `circles`, `slider`, `brightness`) con ayudas comunes en `paint`,
//! el reparto en `layout` y la burbuja de editar con sus botones "−" en
//! `edit`. No conoce ninguna isla ni llama a Windows:
//! describe qué dibujar y qué zona se pulsa.

mod brightness;
mod capsules;
mod circles;
mod edit;
mod hit;
mod icons;
mod layout;
mod music;
mod paint;
mod props;
mod slider;
mod view;

pub use hit::{control_center_brightness_value, control_center_hit, control_center_slider_value};
pub use icons::ControlCenterIcons;
pub use props::{ControlCenterHit, ControlCenterProps};
pub use view::{control_center_panel, control_center_size};
