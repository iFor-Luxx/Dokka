use std::sync::Arc;

use dokka_icons::{calendar_icon, clock_icon, explorer_icon, launchpad_icon, trash_icon};
use dokka_scene::Drawing;

use crate::Dock;
use crate::logic::long_date;

/// Fecha y hora que muestran el calendario y el reloj. `weekday` 0 es domingo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DockTime {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub weekday: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}

/// Iconos propios ya construidos; solo cambian cuando cambia lo que muestran.
#[derive(Debug)]
pub(crate) struct SystemIcons {
    pub(crate) explorer: Arc<Drawing>,
    pub(crate) launchpad: Arc<Drawing>,
    pub(crate) calendar: Arc<Drawing>,
    pub(crate) clock: Arc<Drawing>,
    pub(crate) trash: Arc<Drawing>,
    pub(crate) long_date: String,
    time: DockTime,
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
            long_date: String::new(),
            time: DockTime::default(),
            trash_is_full: false,
        }
    }
}

impl Dock {
    pub fn set_time(&mut self, time: DockTime) {
        let icons = &mut self.icons;
        let old = icons.time;
        if (old.year, old.month, old.day) != (time.year, time.month, time.day) {
            icons.calendar = Arc::new(calendar_icon(time.month, time.day));
            icons.long_date = long_date(time.year, time.month, time.day, time.weekday);
        }
        if old != time {
            icons.clock = Arc::new(clock_icon(time.hour, time.minute, time.second));
            icons.time = time;
        }
    }

    pub fn set_trash_full(&mut self, is_full: bool) {
        if self.icons.trash_is_full != is_full {
            self.icons.trash_is_full = is_full;
            self.icons.trash = Arc::new(trash_icon(is_full));
        }
    }
}
