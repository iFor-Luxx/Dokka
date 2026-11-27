//! Iconos de Dokka: los de color son propios; los de interfaz salen de Lucide.

mod app_tile;
mod calendar;
mod clock;
mod explorer;
mod launchpad;
mod lucide;
mod shapes;
mod tile_color;
mod tile_frame;
mod tile_paint;
mod tile_pixels;
mod tile_square;
mod trash;
mod volume;

pub use app_tile::app_tile;

pub use calendar::calendar_icon;
pub use clock::clock_icon;
pub use explorer::explorer_icon;
pub use launchpad::launchpad_icon;
pub use lucide::{
    checkmark_icon, media_next_icon, media_pause_icon, media_play_icon, media_previous_icon,
    menubar_icon, nav_next_icon, nav_previous_icon, preview_close_icon, preview_pin_icon,
    preview_pinned_icon, search_icon, settings_about_icon, settings_appearance_icon,
    settings_dock_icon, settings_general_icon, settings_launchpad_icon, settings_shortcuts_icon,
    settings_taskbar_icon, submenu_chevron_icon, traffic_circle, traffic_close_button,
    traffic_close_icon, traffic_minimize_button, traffic_minimize_icon, traffic_zoom_button,
    traffic_zoom_icon,
};
pub use trash::trash_icon;
pub use volume::{
    volume_high_icon, volume_low_icon, volume_mute_icon, volume_muted_icon, volume_off_icon,
    volume_settings_icon,
};

pub use lucide_icons::LUCIDE_FONT_BYTES;
