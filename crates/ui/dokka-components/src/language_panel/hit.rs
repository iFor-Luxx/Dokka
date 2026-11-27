use dokka_scene::Rect;

use super::layout::{row_rect, settings_row};
use super::{LanguageEntry, LanguageHit};

/// Fila que hay en el punto, si es elegible.
pub fn language_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    languages: &[LanguageEntry],
) -> Option<LanguageHit> {
    if x < bounds.x || x >= bounds.x + bounds.width {
        return None;
    }
    for (index, language) in languages.iter().enumerate() {
        let row = row_rect(bounds, index);
        if y >= row.y && y < row.y + row.height {
            return Some(LanguageHit::Language(language.id));
        }
    }
    let settings = settings_row(bounds, languages.len());
    if y >= settings.y && y < settings.y + settings.height {
        return Some(LanguageHit::Settings);
    }
    None
}
