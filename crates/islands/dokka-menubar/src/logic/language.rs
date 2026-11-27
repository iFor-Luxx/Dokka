//! Panel de idioma: lista real, apertura, dibujo y elección.

use dokka_components::language_panel::{
    LanguageEntry, LanguageHit, language_hit, language_panel, language_size,
};
use dokka_scene::Overlay;

use super::tray::{tray_anchor, tray_place};
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Idiomas instalados y código corto para la barra.
    /// El shell la rellena antes de que la barra tome el foco.
    pub fn set_languages(&mut self, languages: Vec<LanguageEntry>, code: String) {
        self.languages = languages;
        self.language_code = code;
    }

    /// Mide según cuántos idiomas hay y lo coloca bajo su icono.
    pub fn open_language(&mut self, bar_width: f32) {
        let Some(anchor) = tray_anchor(super::TrayKind::Language, bar_width) else {
            return;
        };
        let size = language_size(&self.languages);
        self.show_tray(super::TrayKind::Language, tray_place(anchor, size, bar_width));
        self.language_hovered = None;
    }

    pub(crate) fn language_overlay(&self) -> Overlay {
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: dokka_theme::menubar::HEIGHT,
            width: language_size(&self.languages).width,
            height: language_size(&self.languages).height,
        });
        language_panel(
            &self.theme,
            &self.language_icons,
            bounds,
            &self.languages,
            self.language_hovered,
        )
    }

    pub(crate) fn language_click(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match language_hit(bounds, x, y, &self.languages) {
            Some(LanguageHit::Language(id)) => Some(MenubarAction::ChooseLanguage(id)),
            Some(LanguageHit::Settings) => Some(MenubarAction::OpenLanguageSettings),
            None => None,
        }
    }

    pub(crate) fn language_hover(&mut self, x: f32, y: f32) {
        self.language_hovered = self
            .tray_bounds()
            .and_then(|bounds| language_hit(bounds, x, y, &self.languages));
    }
}
