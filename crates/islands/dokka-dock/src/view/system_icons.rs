use std::sync::Arc;

use dokka_icons::{calendar_icon, clock_icon, explorer_icon, launchpad_icon, trash_icon};
use dokka_scene::Drawing;

use crate::Dock;

/// Iconos propios ya construidos; solo cambian cuando cambia lo que muestran.
#[derive(Debug)]
pub(crate) struct SystemIcons {
    pub(crate) explorer: Arc<Drawing>,
    pub(crate) launchpad: Arc<Drawing>,
    pub(crate) calendar: Arc<Drawing>,
    pub(crate) clock: Arc<Drawing>,
    pub(crate) trash: Arc<Drawing>,
    date: (u16, u16),
    time: (u16, u16, u16),
    trash_is_full: bool,
}

impl SystemIcons {
    pub(crate) fn new() -> Self {
        Self {
            explorer: Arc::new(explorer_icon()),
            launchpad: Arc::new(launchpad_icon()),
            calendar: Arc::new(calendar_icon(1, 1)),
            clock: Arc::new(clock_icon(0, 0, 0)),
            trash: Arc::new(trash_icon(false)),
            date: (1, 1),
            time: (0, 0, 0),
            trash_is_full: false,
        }
    }
}

impl Dock {
    pub fn set_time(&mut self, month: u16, day: u16, hour: u16, minute: u16, second: u16) {
        let icons = &mut self.icons;
        if icons.date != (month, day) {
            icons.date = (month, day);
            icons.calendar = Arc::new(calendar_icon(month, day));
        }
        if icons.time != (hour, minute, second) {
            icons.time = (hour, minute, second);
            icons.clock = Arc::new(clock_icon(hour, minute, second));
        }
    }

    pub fn set_trash_full(&mut self, is_full: bool) {
        if self.icons.trash_is_full != is_full {
            self.icons.trash_is_full = is_full;
            self.icons.trash = Arc::new(trash_icon(is_full));
        }
    }
}
