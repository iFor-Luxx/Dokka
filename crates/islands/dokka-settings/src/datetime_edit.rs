//! Estado de edición de "Fecha y hora": borradores y desplegables.
//!
//! Vive aparte para que `state.rs` no crezca. La isla solo emite acciones;
//! el shell conecta y pide releer tras cada cambio.

/// Zona para el desplegable: clave y nombre visible traducido.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TimezoneEntry {
    pub key: String,
    pub display: String,
}

/// Fecha manual que viaja al shell con "Aplicar".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeEditManual {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
}

/// Borradores de servidor, fecha manual y zona.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DateTimeEdit {
    /// `true` si el proceso está elevado y lo protegido responde.
    pub can_change: bool,
    /// Todas las zonas ordenadas por visible, si ya se leyeron.
    pub zones: Vec<TimezoneEntry>,
    /// `true` con la casilla del servidor abierta.
    pub server_editing: bool,
    /// Lo escrito en la casilla del servidor.
    pub server_draft: String,
    /// `true` con el editor de fecha abierto.
    pub manual_editing: bool,
    /// Borrador de fecha y hora manual.
    pub day: u16,
    pub month: u16,
    pub year: u16,
    pub hour: u16,
    pub minute: u16,
    /// `true` con el desplegable de zonas abierto.
    pub zone_open: bool,
    /// Primera zona visible del desplegable.
    pub zone_scroll: usize,
}

impl DateTimeEdit {
    /// Fija si lo protegido responde y las zonas conocidas.
    pub(crate) fn set_info(&mut self, can_change: bool, zones: Vec<TimezoneEntry>) {
        self.can_change = can_change;
        self.zones = zones;
        if !can_change {
            self.server_editing = false;
            self.manual_editing = false;
        }
        let max = self.zone_max();
        if self.zone_scroll > max {
            self.zone_scroll = max;
        }
        if self.zone_open && zones_empty(&self.zones) {
            self.zone_open = false;
        }
    }

    /// Abre la casilla con el valor actual.
    pub(crate) fn start_server(&mut self, current: &str) {
        self.server_editing = true;
        self.server_draft = current.to_owned();
    }

    /// Cierra la casilla sin aplicar.
    pub(crate) fn cancel_server(&mut self) {
        self.server_editing = false;
        self.server_draft.clear();
    }

    /// Letra a la casilla, hasta 253.
    pub(crate) fn push_server(&mut self, character: char) {
        if !self.server_editing || self.server_draft.len() >= 253 {
            return;
        }
        if character.is_ascii_alphanumeric() || character == '.' || character == '-' {
            self.server_draft.push(character);
        }
    }

    /// Borra la última letra de la casilla.
    pub(crate) fn pop_server(&mut self) {
        if self.server_editing {
            self.server_draft.pop();
        }
    }

    /// Abre el editor con la fecha viva de hoy.
    pub(crate) fn start_manual(&mut self, now: crate::general::PageNow) {
        self.manual_editing = true;
        self.day = now.day;
        self.month = now.month.clamp(1, 12);
        self.year = now.year.clamp(1970, 2100);
        self.hour = now.hour.min(23);
        self.minute = now.minute.min(59);
        self.clamp_day();
    }

    /// Cierra el editor sin aplicar.
    pub(crate) fn cancel_manual(&mut self) {
        self.manual_editing = false;
    }

    /// Suma un campo con tope, manteniendo fechas válidas.
    pub(crate) fn step(&mut self, field: u8, delta: i8) {
        if !self.manual_editing {
            return;
        }
        match field {
            0 => self.day = step_clamped(self.day, 1, days_in_month(self.year, self.month), delta),
            1 => {
                self.month = step_clamped(self.month, 1, 12, delta);
                self.clamp_day();
            }
            2 => {
                self.year = step_clamped(self.year, 1970, 2100, delta);
                self.clamp_day();
            }
            3 => self.hour = step_clamped(self.hour, 0, 23, delta),
            _ => self.minute = step_clamped(self.minute, 0, 59, delta),
        }
    }

    /// Abre o cierra el desplegable de zonas.
    pub(crate) fn toggle_zones(&mut self) {
        if !self.zones.is_empty() {
            self.zone_open = !self.zone_open;
        }
    }

    /// Mueve la lista con la rueda, sin salirse.
    pub(crate) fn scroll_zones(&mut self, up: bool) {
        let max = self.zone_max();
        if up {
            self.zone_scroll = self.zone_scroll.saturating_sub(1);
        } else if self.zone_scroll < max {
            self.zone_scroll += 1;
        }
    }

    fn zone_max(&self) -> usize {
        self.zones.len().saturating_sub(visible_zones()).min(512)
    }

    fn clamp_day(&mut self) {
        let max = days_in_month(self.year, self.month);
        if self.day < 1 {
            self.day = 1;
        } else if self.day > max {
            self.day = max;
        }
    }
}

/// Filas visibles del desplegable a la vez.
pub(crate) fn visible_zones() -> usize {
    6
}

/// Días del mes con bisiestos, siempre válidos.
pub(crate) fn days_in_month(year: u16, month: u16) -> u16 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap =
                year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
            if leap { 29 } else { 28 }
        }
        _ => 31,
    }
}

/// Suma con topes, sin pasar de `min` ni de `max`.
fn step_clamped(value: u16, min: u16, max: u16, delta: i8) -> u16 {
    let next = i32::from(value) + i32::from(delta);
    next.clamp(i32::from(min), i32::from(max)) as u16
}

/// Servidor válido: 1-253 letras, números, puntos y guiones.
pub(crate) fn is_valid_server(server: &str) -> bool {
    if server.is_empty() || server.len() > 253 {
        return false;
    }
    server
        .bytes()
        .all(|unit| unit.is_ascii_alphanumeric() || unit == b'.' || unit == b'-')
}

fn zones_empty(zones: &[TimezoneEntry]) -> bool {
    zones.is_empty()
}

impl crate::state::Settings {
    /// Fija si lo protegido responde y las zonas del desplegable.
    pub fn set_datetime_info(&mut self, can_change: bool, zones: Vec<TimezoneEntry>) {
        self.datetime_edit.set_info(can_change, zones);
    }

    /// Editor de fecha y zona de la página.
    pub(crate) fn datetime_edit(&self) -> &DateTimeEdit {
        &self.datetime_edit
    }

    pub(crate) fn datetime_edit_mut(&mut self) -> &mut DateTimeEdit {
        &mut self.datetime_edit
    }
}
