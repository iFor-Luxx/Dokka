use std::sync::Arc;

use dokka_icons::{calendar_icon, clock_icon, explorer_icon, launchpad_icon, trash_icon};
use dokka_scene::Drawing;
use dokka_theme::IconColors;

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
    pub(crate) fn new(colors: &IconColors) -> Self {
        Self {
            explorer: Arc::new(explorer_icon(colors)),
            launchpad: Arc::new(launchpad_icon(colors)),
            calendar: Arc::new(calendar_icon(colors, 1, 1)),
            clock: Arc::new(clock_icon(colors, 0, 0, 0)),
            trash: Arc::new(trash_icon(colors, false)),
            long_date: String::new(),
            time: DockTime::default(),
            trash_is_full: false,
        }
    }
}

impl Dock {
    pub fn set_time(&mut self, time: DockTime) {
        let colors = &self.theme.icons;
        let icons = &mut self.icons;
        let old = icons.time;
        if (old.year, old.month, old.day) != (time.year, time.month, time.day) {
            icons.calendar = Arc::new(calendar_icon(colors, time.month, time.day));
            icons.long_date = long_date(time.year, time.month, time.day, time.weekday);
        }
        if old != time {
            let clock = clock_icon(colors, time.hour, time.minute, time.second);
            icons.clock = Arc::new(clock);
            icons.time = time;
        }
    }

    pub fn set_trash_full(&mut self, is_full: bool) {
        if self.icons.trash_is_full != is_full {
            self.icons.trash_is_full = is_full;
            self.icons.trash = Arc::new(trash_icon(&self.theme.icons, is_full));
        }
    }
}
