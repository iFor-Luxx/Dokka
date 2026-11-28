//! Datos del equipo para "Acerca de este PC", "Fecha y hora" e
//! "Idioma y región": pedir, pasar y abrir.
//!
//! La isla no habla con Windows: el shell pide el resumen al entrar en cada
//! apartado, lo pasa cuando el hilo lo lee y abre las páginas de los botones.
//! Falla sola: si no se lee, el campo queda en "No disponible". La fecha en
//! vivo la pasa el `Tick` solo con su apartado abierto.

use dokka_settings::{AboutButton, GeneralButton, GeneralPage, PageNow};

use super::dock_session::DockSession;

/// Pide el resumen del equipo una vez y pasa lo que ya haya al abrir un
/// apartado con datos ("Acerca", "Fecha y hora" o "Idioma y región").
pub(crate) fn request_system_data(session: &mut DockSession) {
    use dokka_contracts::SystemInfo as _;
    if let Some(summary) = session.system_info.summary() {
        session.settings.set_system_info(Some(summary));
    } else {
        session.system_info.request_refresh();
    }
    // "Fecha y hora" enseña la hora en vivo desde la primera pintura.
    refresh_now(session);
}

/// Si se está en un apartado con datos y el hilo ya leyó, pasa el último
/// valor a la isla para que repinte sin pedir en bucle.
pub(crate) fn poll_system_data(session: &mut DockSession) {
    if !is_data_page(session.settings.general()) {
        return;
    }
    use dokka_contracts::SystemInfo as _;
    if let Some(summary) = session.system_info.summary()
        && session.settings.system_summary() != Some(summary.clone())
    {
        session.settings.set_system_info(Some(summary));
    }
}

/// `true` en los apartados que muestran datos leídos de Windows.
fn is_data_page(page: GeneralPage) -> bool {
    matches!(
        page,
        GeneralPage::About | GeneralPage::DateTime | GeneralPage::Language
    )
}

/// Pasa la fecha y hora en vivo a la isla, solo con "Fecha y hora" abierto:
/// así el `Tick` la actualiza sin repintar el resto del tiempo.
pub(crate) fn tick_datetime(session: &mut DockSession) {
    if !session.settings.is_open() || session.settings.general() != GeneralPage::DateTime {
        return;
    }
    refresh_now(session);
}

/// Lee el reloj local y lo deja en la isla si cambió, para repintar lo mínimo.
fn refresh_now(session: &mut DockSession) {
    let now = dokka_win_surface::local_time();
    let page_now = PageNow {
        year: now.year,
        month: now.month,
        day: now.day,
        weekday: now.weekday,
        hour: now.hour,
        minute: now.minute,
        second: now.second,
    };
    if session.settings.page_now() != Some(page_now) {
        session.settings.set_page_now(page_now);
    }
}

/// Abre la página del sistema del botón pulsado. Falla sola.
pub(crate) fn open_about_page(session: &DockSession, button: AboutButton) {
    use dokka_contracts::{SystemInfo as _, SystemPage};
    let page = match button {
        AboutButton::DisplaySettings => SystemPage::Display,
        AboutButton::StorageSettings => SystemPage::Storage,
        AboutButton::SystemReport => SystemPage::Report,
    };
    if let Err(error) = session.system_info.open_page(page) {
        eprintln!("Dokka no pudo abrir la página del sistema: {error}");
    }
}

/// Abre la página de Windows del botón de fecha o idioma. Falla sola.
pub(crate) fn open_general_page(session: &DockSession, button: GeneralButton) {
    use dokka_contracts::{SystemInfo as _, SystemPage};
    let page = match button {
        GeneralButton::DateTimeSettings => SystemPage::DateTime,
        GeneralButton::LanguageSettings => SystemPage::Region,
    };
    if let Err(error) = session.system_info.open_page(page) {
        eprintln!("Dokka no pudo abrir la página del sistema: {error}");
    }
}

/// Rueda sobre Preferencias: baja las fichas con datos ("Acerca",
/// "Fecha y hora" e "Idioma y región").
pub(crate) fn scroll_settings(session: &mut DockSession, delta: i16) {
    if session.settings.is_open() {
        session.settings.scroll_wheel(delta);
    }
}
