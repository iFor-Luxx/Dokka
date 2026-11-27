mod content;
mod content_hit;
mod dock_position;
mod hit;
mod menubar_order;
mod search_filter;
mod sections;
mod slider_map;

pub(crate) use content::{
    card_heights, card_tops, content_row, content_segmented, content_slider_area,
    content_slider_track, has_content, section_cards,
};
pub(crate) use content_hit::{
    DockSwitch, SliderTarget, dock_segment_at, dock_slider_at, dock_switch_at, menubar_switch_at,
    taskbar_switch_at,
};
pub(crate) use dock_position::{position_from_index, position_options, position_to_index};
pub use hit::drag_zone;
pub(crate) use menubar_order::{
    float_top, menubar_bottom, menubar_label, menubar_on, menubar_row_at, menubar_top,
    reorder_target, reordered, toggle_menubar,
};
pub(crate) use search_filter::filter_sections;
pub use sections::SECTION_NAMES;
pub(crate) use sections::{SECTION_DOCK, SECTION_MENUBAR, SECTION_TASKBAR};
pub(crate) use sections::{badge_drawing, section_drawing, section_names};
pub(crate) use slider_map::{
    icon_scale_from, icon_scale_neutral, icon_scale_to, magnification_from, magnification_neutral,
    magnification_to,
};
