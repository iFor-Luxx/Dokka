//! Deslizador estilo Tahoe: pista fina con relleno y tirador en cápsula.
//!
//! El único deslizador de Dokka: lo usan Preferencias, el panel de volumen
//! y el Sonido y la Pantalla del centro de control; ningún panel dibuja el
//! suyo. Recibe su zona, el valor de 0 a 1, si está activo, el valor de
//! fábrica opcional y sus colores: con fábrica (`Some`) el relleno va desde
//! ella hasta el valor con su marca (Preferencias); sin ella (`None`) el
//! relleno va desde la izquierda sin marca. Da cuatro planos —barra, relleno,
//! marca y tirador— y el valor bajo un punto. El relleno y la marca existen
//! siempre (de tamaño cero si no se ven) y su estilo no cambia al arrastrar:
//! así no se reconstruyen capas. El área deja medio tirador de aire a cada
//! lado para que el tirador no se salga en los extremos. Sin saber qué ajusta.

mod icons;
mod props;
mod view;

pub use icons::SliderIcons;
pub use props::{SliderColors, SliderProps};
pub use view::{
    slider_fill_panel, slider_hit, slider_knob_bounds, slider_knob_panel, slider_panels,
    slider_tick_bounds, slider_tick_panel, slider_track_bounds, slider_track_panel,
    slider_value_at,
};
