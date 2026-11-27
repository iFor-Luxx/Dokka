//! Deslizador reutilizable: barra gris con relleno y tirador blanco.
//!
//! Recibe su zona, el valor de 0 a 1 y si está activo (relleno azul)
//! o atenuado (relleno gris); da los planos de la barra y el relleno,
//! el punto del tirador y el valor bajo un punto. Sin saber qué ajusta:
//! el volumen lo usa y Preferencias lo reutilizará.

mod icons;
mod props;
mod view;

pub use icons::SliderIcons;
pub use props::SliderProps;
pub use view::{
    slider_fill_panel, slider_hit, slider_knob, slider_panels, slider_size, slider_track_bounds,
    slider_track_panel, slider_value_at,
};
