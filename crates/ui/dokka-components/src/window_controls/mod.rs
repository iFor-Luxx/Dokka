//! Semáforos de la ventana de Preferencias: cerrar, minimizar y zoom.
mod props;
mod traffic_glyphs;
mod view;

pub use props::TrafficButton;
pub use traffic_glyphs::{TrafficButtons, traffic_icons};
pub use view::traffic_at;
