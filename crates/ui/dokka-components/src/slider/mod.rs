//! Deslizador reutilizable estilo Tahoe: cápsula ancha con relleno.
//!
//! Recibe su zona, el valor de 0 a 1, si está activo y sus colores (barra,
//! relleno y relleno atenuado); da los planos de la barra y el relleno y el
//! valor bajo un punto. Sin saber qué ajusta: el volumen lo usa con el
//! cristal ahumado y Preferencias le pasará los suyos.

mod icons;
mod props;
mod view;

pub use icons::SliderIcons;
pub use props::{SliderColors, SliderProps};
pub use view::{
    slider_fill_panel, slider_hit, slider_panels, slider_size, slider_track_bounds,
    slider_track_panel, slider_value_at,
};
