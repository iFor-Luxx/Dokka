//! Lista de idiomas instalados con marca Lucide en el activo,
//! resaltado de fila y ajustes. En la barra el idioma es su código en texto.

mod icons;
mod props;
mod view;

pub use icons::LanguageIcons;
pub use props::{LanguageEntry, LanguageHit};
pub use view::{language_hit, language_panel, language_size};
