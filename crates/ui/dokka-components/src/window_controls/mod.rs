//! Semáforos de la ventana de Preferencias: cerrar, minimizar y zoom.
mod props;
mod traffic_glyphs;
mod view;

pub use props::TrafficButton;
pub use traffic_glyphs::{TrafficGlyphs, traffic_glyph_icons};
pub use view::{traffic_at, traffic_dots};
