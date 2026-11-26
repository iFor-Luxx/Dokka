mod filter_apps;
mod launchpad_click;
mod merge_apps;

pub use filter_apps::filter_apps;
pub(crate) use launchpad_click::{app_index, click_launchpad};
pub use merge_apps::merge_apps;
