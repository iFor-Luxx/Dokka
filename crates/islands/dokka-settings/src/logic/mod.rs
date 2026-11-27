mod content;
mod hit;
mod search_filter;
mod sections;

pub(crate) use content::{
    SliderTarget, card_tops, content_row, content_slider_area, content_slider_track,
    dock_slider_at, dock_switch_at, has_content, icon_scale_from, icon_scale_to,
    magnification_from, magnification_to, menubar_switch_at, section_cards, taskbar_switch_at,
};
pub use hit::drag_zone;
pub(crate) use search_filter::filter_sections;
pub use sections::SECTION_NAMES;
pub(crate) use sections::{SECTION_DOCK, SECTION_MENUBAR, SECTION_TASKBAR};
pub(crate) use sections::{badge_drawing, section_drawing, section_names};
