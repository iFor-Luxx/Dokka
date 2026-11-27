//! Deslizador estilo Tahoe: pista fina con relleno y tirador en cápsula.
//!
//! Recibe su zona, el valor de 0 a 1, si está activo y sus colores (barra,
//! relleno, atenuado, tirador y su borde); da los tres planos —barra,
//! relleno y tirador— y el valor bajo un punto. El área deja medio tirador
//! de aire a cada lado para que el tirador no se salga en los extremos.
//! Sin saber qué ajusta: solo lo usa Preferencias.

mod icons;
mod props;
mod view;

pub use icons::SliderIcons;
pub use props::{SliderColors, SliderProps};
pub use view::{
    slider_fill_panel, slider_hit, slider_knob_bounds, slider_knob_panel, slider_panels,
    slider_track_bounds, slider_track_panel, slider_value_at,
};
