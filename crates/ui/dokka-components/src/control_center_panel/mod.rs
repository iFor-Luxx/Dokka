//! Centro de control estilo Tahoe: rejilla de dos columnas con cápsulas de
//! red y Preferencias a la izquierda y música cuadrada a la derecha, fila de
//! círculos (escritorio, bloqueo y tareas) y tarjeta de Sonido con deslizador
//! a todo lo ancho. No conoce ninguna isla ni llama a Windows: describe qué
//! dibujar y qué zona se pulsa.

mod hit;
mod icons;
mod layout;
mod props;
mod view;

pub use hit::control_center_hit;
pub use icons::ControlCenterIcons;
pub use props::{ControlCenterHit, ControlCenterProps};
pub use view::{control_center_panel, control_center_size};
