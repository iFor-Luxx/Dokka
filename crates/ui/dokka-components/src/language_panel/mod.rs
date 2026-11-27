//! Lista de idiomas estilo Tahoe: un solo cristal oscuro con título,
//! una fila compacta por idioma con marca y código, línea y ajustes.
//! En la barra el idioma es su código en texto.

mod icons;
mod props;
mod view;

pub use icons::LanguageIcons;
pub use props::{LanguageEntry, LanguageHit};
pub use view::{language_hit, language_panel, language_size};
