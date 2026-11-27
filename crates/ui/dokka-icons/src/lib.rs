//! Iconos de Dokka: los de color son propios; los de interfaz salen de Lucide.

mod app_tile;
mod calendar;
mod clock;
mod control_center;
mod explorer;
mod launchpad;
mod lucide;
mod menu;
mod menubar_tray;
mod music;
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
    control_center_task_manager_icon, control_center_task_view_icon, control_center_volume_icon,
    control_center_wifi_icon, control_center_wifi_off_icon,
};
pub use explorer::explorer_icon;
pub use launchpad::launchpad_icon;
pub use lucide::{
    checkmark_icon, media_next_icon, media_pause_icon, media_play_icon, media_previous_icon,
    menubar_icon, nav_next_icon, nav_previous_icon, preview_close_icon, preview_pin_icon,
    preview_pinned_icon, search_icon, settings_about_icon, settings_appearance_icon,
    settings_dock_icon, settings_general_icon, settings_launchpad_icon, settings_menubar_icon,
    settings_shortcuts_icon, settings_taskbar_icon, submenu_chevron_icon, traffic_circle,
    traffic_close_button, traffic_close_icon, traffic_minimize_button, traffic_minimize_icon,
    traffic_zoom_button, traffic_zoom_icon,
};
pub use menu::{
    menu_add_app_icon, menu_add_separator_icon, menu_add_system_icon, menu_admin_icon,
    menu_close_icon, menu_desktop_icon, menu_documents_icon, menu_downloads_icon,
    menu_empty_trash_icon, menu_hide_taskbar_icon, menu_launchpad_icon, menu_lock_icon,
    menu_new_window_icon, menu_open_icon, menu_pictures_icon, menu_pin_icon, menu_power_icon,
    menu_preferences_icon, menu_quit_icon, menu_remove_separator_icon, menu_restart_icon,
    menu_reveal_icon, menu_show_taskbar_icon, menu_task_manager_icon, menu_taskbar_settings_icon,
    menu_trash_open_icon, menu_unpin_icon,
};
pub use menubar_tray::{TRAY_GLYPH_SIDE, TrayIcons};
pub use music::music_bar_icon;
pub use network_language::{
    language_check_icon, language_row_icon, network_ethernet_icon, network_offline_icon,
    network_other_icon, network_wifi_icon, tray_chevron_icon, tray_settings_icon,
};
pub use search::{search_bar_icon, search_field_icon};
pub use trash::trash_icon;
pub use volume::{
    volume_high_icon, volume_low_icon, volume_mute_icon, volume_muted_icon, volume_off_icon,
    volume_settings_icon,
};

pub use lucide_icons::LUCIDE_FONT_BYTES;
