use windows::Win32::System::SystemInformation::GetLocalTime;

/// Fecha y hora locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalTime {
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}

pub fn local_time() -> LocalTime {
    // SAFETY: no recibe argumentos y devuelve una estructura por valor.
    let now = unsafe { GetLocalTime() };
    LocalTime {
        month: now.wMonth,
        day: now.wDay,
        hour: now.wHour,
        minute: now.wMinute,
        second: now.wSecond,
    }
}
