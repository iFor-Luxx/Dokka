//! Iconos de Dokka: los de color son propios; los de interfaz salen de Lucide.

mod app_tile;
mod calendar;
mod clock;
mod control_center;
mod explorer;
mod launchpad;
mod lucide;
mod menubar_tray;
mod network_language;
mod search;
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
pub use control_center::{
    control_center_bar_icon, control_center_calculator_icon, control_center_clock_icon,
    control_center_desktop_icon, control_center_ethernet_icon, control_center_globe_icon,
    control_center_lock_icon, control_center_muted_icon, control_center_next_icon,
    control_center_pause_icon, control_center_play_icon, control_center_preferences_icon,
    control_center_previous_icon, control_center_project_icon, control_center_screen_clip_icon,
    control_center_sun_dim_icon, control_center_sun_icon, control_center_task_manager_icon,
    control_center_task_view_icon, control_center_volume_icon, control_center_wifi_icon,
    control_center_wifi_off_icon,
};
pub use explorer::explorer_icon;
pub use launchpad::launchpad_icon;
pub use lucide::{
    checkmark_icon, general_about_icon, general_datetime_icon, general_language_icon,
    general_storage_icon, general_update_icon, media_next_icon, media_pause_icon, media_play_icon,
    media_previous_icon, menubar_icon, nav_next_icon, nav_previous_icon, preview_close_icon,
    preview_pin_icon, preview_pinned_icon, search_icon, settings_about_icon,
    settings_appearance_icon, settings_dock_icon, settings_general_icon, settings_grip_icon,
    settings_launchpad_icon, settings_menubar_icon, settings_shortcuts_icon, settings_taskbar_icon,
    storage_apps_icon, storage_documents_icon, storage_downloads_icon, storage_images_icon,
    storage_music_icon, storage_other_icon, storage_trash_icon, storage_videos_icon,
    storage_windows_icon, submenu_chevron_icon, traffic_circle, traffic_close_button, traffic_minimize_button,
    traffic_zoom_button,
};
pub use menubar_tray::TrayIcons;
pub use network_language::{
    language_check_icon, network_ethernet_icon, network_offline_icon, network_other_icon,
    network_wifi_icon, tray_chevron_icon, tray_settings_icon,
};
pub use search::{search_bar_icon, search_field_icon};
pub use trash::trash_icon;
pub use volume::{
    volume_high_icon, volume_low_icon, volume_muted_icon, volume_off_icon, volume_settings_icon,
};

pub use lucide_icons::LUCIDE_FONT_BYTES;
