mod content;
mod content_hit;
mod datetime;
mod dock_position;
mod general;
mod hit;
mod language;
mod list_counts;
mod menubar_order;
mod search_filter;
mod sections;
mod slider_map;
mod storage;
mod update;

pub(crate) use content::{
    card_heights, card_tops, content_row, content_segmented, content_slider_area,
    content_slider_track, has_content, section_cards,
};
pub(crate) use content_hit::{
    DockSwitch, SliderTarget, about_button_at, dock_segment_at, dock_slider_at, dock_switch_at,
    general_row_at, menubar_switch_at, storage_button_at, taskbar_switch_at,
};
pub(crate) use datetime::{
    datetime_button_at, datetime_button_top, datetime_card_top, datetime_labels, datetime_row,
    datetime_switch_at,
};
pub(crate) use dock_position::{position_from_index, position_options, position_to_index};
pub(crate) use general::{
    GENERAL_HEADER_HELP_FIRST, GENERAL_HEADER_HELP_SECOND, GENERAL_ROWS, about_main_rows,
    about_top, general_badge_color, general_card_top, general_row_drawing, general_rows,
    general_scroll_max,
};
pub use hit::drag_zone;
pub(crate) use language::{
    language_button_at, language_button_top, language_card_top, language_example_top,
    language_format_rows, language_row, language_segment_at,
};
pub use list_counts::ListCounts;
pub(crate) use menubar_order::{
    float_top, menubar_bottom, menubar_label, menubar_on, menubar_row_at, menubar_top,
    reorder_target, reordered, toggle_menubar,
};
pub(crate) use search_filter::filter_sections;
pub use sections::SECTION_NAMES;
pub(crate) use sections::{SECTION_DOCK, SECTION_GENERAL, SECTION_MENUBAR, SECTION_TASKBAR};
pub(crate) use sections::{badge_drawing, section_drawing, section_names};
pub(crate) use slider_map::{
    icon_scale_from, icon_scale_neutral, icon_scale_to, magnification_from, magnification_neutral,
    magnification_to,
};
pub(crate) use storage::{
    StorageLayout, storage_category_color, storage_category_drawing, storage_header_rows,
    storage_layout, storage_list_rows,
};
pub(crate) use update::update_top;
