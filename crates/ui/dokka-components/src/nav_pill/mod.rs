//! Píldora segmentada de navegación estilo Tahoe: cápsula con dos
//! chevrones Lucide y una línea corta en el centro. No sabe qué navega:
//! recibe su rectángulo, qué lados están disponibles y el hover.

mod icons;
mod props;
mod view;

pub use icons::NavPillIcons;
pub use props::{NavHalf, NavPillProps};
pub use view::{
    nav_pill_at, nav_pill_contains, nav_pill_divider, nav_pill_highlight, nav_pill_icons,
    nav_pill_panel,
};
