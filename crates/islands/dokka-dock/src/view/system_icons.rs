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
    pub(crate) trash_size: Option<u64>,
    time: DockTime,
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
            trash_size: None,
            time: DockTime::default(),
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

    pub fn set_trash(&mut self, size: Option<u64>) {
        // El icono solo distingue entre vacía y con contenido.
        let was_full = self.icons.trash_size.is_some();
        let is_full = size.is_some();
        self.icons.trash_size = size;
        if was_full != is_full {
            self.icons.trash = Arc::new(trash_icon(&self.theme.icons, is_full));
        }
    }
}
