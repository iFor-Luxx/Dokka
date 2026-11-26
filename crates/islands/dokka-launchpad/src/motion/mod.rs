//! Transición de páginas del Launchpad: deslizamiento y tope elástico.

mod page_navigation;
mod page_slide;
mod zoom;

pub(crate) use page_slide::PageMotion;
pub(crate) use zoom::ZoomMotion;
