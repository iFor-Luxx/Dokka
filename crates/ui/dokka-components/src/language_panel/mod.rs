//! Lista de idiomas estilo Tahoe: un solo cristal oscuro con título,
//! una fila compacta por idioma con marca y código, línea y ajustes.
//! En la barra el idioma es su código en texto.

mod hit;
mod icons;
mod layout;
mod props;
mod view;

pub use hit::language_hit;
pub use icons::LanguageIcons;
pub use props::{LanguageEntry, LanguageHit};
pub use view::{language_panel, language_size};
