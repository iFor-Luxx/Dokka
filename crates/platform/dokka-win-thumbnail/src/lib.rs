//! Imagen viva de una ventana como visual de composición.
//!
//! Aísla las funciones sin documentar de Windows que lo permiten.

#![allow(unsafe_code)]

mod compositor;
mod window_image;

pub use compositor::{LiveCompositor, LiveTarget};
pub use window_image::{LiveImage, source_size};
