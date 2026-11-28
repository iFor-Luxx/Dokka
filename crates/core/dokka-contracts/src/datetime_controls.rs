//! Escritura de fecha, hora y zona horaria desde Preferencias.
//!
//! Contrato pequeño y aparte del de lectura (`SystemInfo`): la isla solo
//! emite acciones y el shell conecta con quien lo cumple. Los cambios son
//! del usuario y no se deshacen al cerrar Dokka. Si algo falla, quien lo
//! cumple lo registra, relee y la pantalla muestra el valor real.

/// Zona horaria de Windows para el desplegable: clave en inglés y nombre
/// visible traducido, ordenadas por nombre visible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimezoneOption {
    /// Clave del registro, como "SA Pacific Standard Time".
    pub key: String,
    /// Nombre visible traducido, como "(UTC-05:00) Bogotá, Lima...".
    pub display: String,
}

/// Fecha y hora que Dokka puede cambiar, sin bloquear a la isla.
///
/// Todo se ejecuta en el hilo propio, nunca en el de la interfaz.
/// `can_change_protected` dice si el proceso está elevado; sin elevar,
/// lo protegido se ve atenuado y no responde.
pub trait DateTimeControls {
    /// `true` si el proceso está elevado y puede cambiar lo protegido.
    fn can_change_protected(&self) -> bool;
    /// Pide la lista de zonas en el hilo propio, sin bloquear.
    fn request_timezones(&self);
    /// Última lista conocida de zonas, si ya se leyó.
    fn timezones(&self) -> Option<Vec<TimezoneOption>>;
    /// Automáticas (`NTP`) o manual (`NoSync`), con aviso al servicio.
    fn set_automatic_time(&self, enabled: bool);
    /// Servidor NTP validado (solo letras, números, puntos y guiones).
    fn set_ntp_server(&self, server: String);
    /// Fecha y hora manual; solo con las automáticas apagadas.
    fn set_manual_time(&self, date: ManualDateTime);
    /// Zona automática (servicio `tzautoupdate`: 3 manual, 4 deshabilitado).
    fn set_automatic_timezone(&self, enabled: bool);
    /// Zona por su clave; activa `SeTimeZonePrivilege` solo en la llamada.
    fn set_timezone(&self, key: String);
}

/// Día y hora que escribe "Fecha y hora" con "Aplicar".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManualDateTime {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
}
