//! Estilos de Dokka: colores por tema, fuente y medidas.

pub mod branding;
mod measures;
mod palette;
mod theme;
mod themes;
pub mod typography;

pub use branding::{
    ABOUT_MODEL_NAME, ABOUT_OS_NAME, ABOUT_OS_SECTION_TITLE, ABOUT_OS_VERSION_LABEL,
};

pub use measures::{
    background, control_center, control_gallery, dock, launchpad, menu, menubar, menubar_tray,
    music_panel, nav_pill, network_language, popup, search, settings, tooltip, tray_glass, volume,
};
pub use theme::{
    DockColors, IconColors, LaunchpadColors, MediaColors, MenuColors, MenubarColors, NavPillColors,
    PanelColors, SettingsColors, Theme, TooltipColors, TrayColors,
};
pub use themes::light;
