//! Panel de música: canción actual, apertura, dibujo y botones.

use dokka_components::music_panel::{
    MusicPanelHit, MusicTrack, music_panel_hit, music_panel_panel, music_panel_size,
};
use dokka_model::NowPlayingInfo;
use dokka_scene::Overlay;

use super::tray::{tray_anchor, tray_place};
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Lo que suena ahora; sin nada, el panel muestra su vacío.
    /// El shell la rellena cada `Tick` desde `MediaMonitor`.
    pub fn set_now_playing(&mut self, current: Option<NowPlayingInfo>) {
        self.track = current
            .filter(|info| !info.title.trim().is_empty())
            .map(|info| MusicTrack {
                title: info.title,
                artist: info.artist,
                is_playing: info.is_playing,
            });
        self.now_playing = self.track.as_ref().map_or(
            NowPlayingInfo {
                title: String::new(),
                artist: String::new(),
                is_playing: false,
            },
            |track| NowPlayingInfo {
                title: track.title.clone(),
                artist: track.artist.clone(),
                is_playing: track.is_playing,
            },
        );
    }

    /// Mide con su tamaño real y lo coloca bajo su icono.
    pub fn open_music(&mut self, bar_width: f32) {
        let Some(anchor) = tray_anchor(super::TrayKind::Music, bar_width) else {
            return;
        };
        let size = music_panel_size(self.track.as_ref());
        self.show_tray(super::TrayKind::Music, tray_place(anchor, size, bar_width));
    }

    pub(crate) fn music_overlay(&self) -> Overlay {
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: dokka_theme::menubar::HEIGHT,
            width: music_panel_size(self.track.as_ref()).width,
            height: music_panel_size(self.track.as_ref()).height,
        });
        music_panel_panel(&self.theme, &self.music_icons, bounds, self.track.as_ref())
    }

    pub(crate) fn music_click(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match music_panel_hit(bounds, x, y, self.track.as_ref()) {
            Some(MusicPanelHit::Previous) => Some(MenubarAction::MusicPrevious),
            Some(MusicPanelHit::Toggle) => Some(MenubarAction::MusicToggle),
            Some(MusicPanelHit::Next) => Some(MenubarAction::MusicNext),
            None => None,
        }
    }
}
