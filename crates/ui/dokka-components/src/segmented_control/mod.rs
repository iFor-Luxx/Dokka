//! Segmentado estilo Tahoe para elegir una opción entre varias.
//!
//! Cápsula gris con la opción elegida en pastilla blanca de radios
//! concéntricos y texto centrado en cada tercio. No sabe qué ajusta: recibe
//! los textos y el índice elegido; si la isla usa otro tipo, lo traduce.
//! Da dos planos (fondo y pastilla) y un texto por opción, y el segmento
//! bajo un punto con la misma geometría del dibujo. Los textos caben con el
//! ancho del tema.

mod layout;
mod props;
mod view;

pub use layout::{segment_bounds, segmented_touch};
pub use props::SegmentedProps;
pub use view::{segmented_at, segmented_hit, segmented_panels, segmented_texts};
