//! Lista de idiomas estilo Tahoe: título, una cápsula por idioma con
//! círculo y subtítulo "Activa", y ajustes en su zona propia. Sin marca
//! de verificación: el círculo activo ya indica el idioma en uso.
//! En la barra el idioma es su código en texto.

mod icons;
mod props;
mod view;

pub use icons::LanguageIcons;
pub use props::{LanguageEntry, LanguageHit};
pub use view::{language_hit, language_panel, language_size};
