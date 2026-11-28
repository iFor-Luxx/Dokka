//! Datos del equipo para "Acerca de este PC", "Fecha y hora",
//! "Idioma y región" y "Almacenamiento": pedir, pasar y abrir.
//!
//! La isla no habla con Windows: el shell pide el resumen al entrar en cada
//! apartado con datos y el desglose al entrar en "Almacenamiento", los pasa
//! cuando los hilos los leen y abre las páginas de los botones. Falla sola:
//! si no se lee, el campo queda en "No disponible" o "Calculando…". La fecha
//! en vivo la pasa el `Tick` solo con su apartado abierto.

use dokka_contracts::{DateTimeControls as _, SystemInfo as _};
use dokka_settings::{AboutButton, DateTimeEditManual, GeneralButton, GeneralPage, PageNow};

use super::dock_session::DockSession;

/// Cuentas variables de General: idiomas leídos y otros discos de hoy.
pub(crate) fn list_counts(session: &DockSession) -> dokka_settings::ListCounts {
    dokka_settings::ListCounts::new(
        session.settings.language_count(),
        session
            .settings
            .storage_breakdown()
            .map(|known| known.other_disks.len())
            .unwrap_or(0),
    )
}

/// Pide el resumen del equipo una vez y pasa lo que ya haya al abrir un
/// apartado con datos ("Acerca", "Fecha y hora" o "Idioma y región").
pub(crate) fn request_system_data(session: &mut DockSession) {
    if let Some(summary) = session.system_info.summary() {
        session.settings.set_system_info(Some(summary));
    } else {
        session.system_info.request_refresh();
    }
    // "Fecha y hora" enseña la hora en vivo desde la primera pintura.
    refresh_now(session);
    if session.settings.general() == GeneralPage::DateTime {
        session.system_info.request_timezones();
        sync_datetime_info(session);
    }
}

/// Si se está en un apartado con datos ("Acerca", "Fecha y hora" o
/// "Idioma y región") y el hilo ya leyó, pasa el último valor a la isla
/// para que repinte sin pedir en bucle.
pub(crate) fn poll_system_data(session: &mut DockSession) {
    if !is_data_page(session.settings.general()) {
        return;
    }
    if let Some(summary) = session.system_info.summary()
        && session.settings.system_summary() != Some(summary.clone())
    {
        session.settings.set_system_info(Some(summary));
    }
    if session.settings.general() == GeneralPage::DateTime {
        sync_datetime_info(session);
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

/// Pasa a la isla si lo protegido responde y las zonas del desplegable.
/// Solo con "Fecha y hora" abierto; conserva los borradores en curso.
pub(crate) fn sync_datetime_info(session: &mut DockSession) {
    let can_change = session.system_info.can_change_protected();
    let zones = session
        .system_info
        .timezones()
        .map(|list| {
            list.into_iter()
                .map(|zone| dokka_settings::TimezoneEntry {
                    key: zone.key,
                    display: zone.display,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    session.settings.set_datetime_info(can_change, zones);
}

/// Aplica un cambio de fecha tras un clic; el hilo relee y la pantalla
/// muestra el valor real. Falla sola.
pub(crate) fn apply_datetime_auto_time(session: &DockSession, enabled: bool) {
    session.system_info.set_automatic_time(enabled);
}

/// Aplica un servidor validado; el hilo relee tras escribir.
pub(crate) fn apply_datetime_server(session: &DockSession, server: String) {
    session.system_info.set_ntp_server(server);
}

/// Aplica la fecha manual; solo con las automáticas apagadas.
pub(crate) fn apply_datetime_manual(session: &DockSession, date: DateTimeEditManual) {
    use dokka_contracts::ManualDateTime;
    session.system_info.set_manual_time(ManualDateTime {
        year: date.year,
        month: date.month,
        day: date.day,
        hour: date.hour,
        minute: date.minute,
    });
}

/// Aplica la zona automática del servicio `tzautoupdate`.
pub(crate) fn apply_datetime_auto_zone(session: &DockSession, enabled: bool) {
    session.system_info.set_automatic_timezone(enabled);
}

/// Aplica la zona elegida por su clave.
pub(crate) fn apply_datetime_zone(session: &DockSession, key: String) {
    session.system_info.set_timezone(key);
}

/// Pasa lo ya calculado y pide recalcular en el hilo. Solo calcula al
/// abrir el apartado, nunca en bucle: mientras tanto la isla sigue mostrando
/// el último resultado en memoria.
pub(crate) fn request_storage(session: &mut DockSession) {
    if let Some(breakdown) = session.system_info.storage() {
        session.settings.set_storage(Some(breakdown));
    }
    session.system_info.request_storage();
}

/// Si se está en "Almacenamiento" y el hilo ya calculó algo nuevo, pasa el
/// último valor a la isla para que repinte sin pedir en bucle.
pub(crate) fn poll_storage(session: &mut DockSession) {
    if session.settings.general() != GeneralPage::Storage {
        return;
    }
    if let Some(breakdown) = session.system_info.storage()
        && session.settings.storage_breakdown() != Some(breakdown.clone())
    {
        session.settings.set_storage(Some(breakdown));
    }
}

/// Detiene el cálculo al salir del apartado; conserva lo ya calculado.
pub(crate) fn cancel_storage(session: &DockSession) {
    session.system_info.cancel_storage();
}

/// Pasa a la isla lo nuevo de los dos hilos: el resumen en su apartado con
/// datos y el desglose en "Almacenamiento" (fuera de él detiene el cálculo).
pub(crate) fn poll_settings(session: &mut DockSession) {
    poll_system_data(session);
    if session.settings.general() == GeneralPage::Storage {
        poll_storage(session);
    } else {
        cancel_storage(session);
    }
}

/// Vacía la papelera con su confirmación y recalcula el apartado.
pub(crate) fn empty_trash(session: &mut DockSession) {
    use dokka_contracts::RecycleBin as _;
    if session.recycle_bin.empty().is_err() {
        return;
    }
    session.system_info.request_storage();
}

/// Pide los datos del apartado al que se acaba de entrar, si trae.
pub(crate) fn request_for_page(session: &mut DockSession, page: GeneralPage) {
    match page {
        GeneralPage::About | GeneralPage::DateTime | GeneralPage::Language => {
            request_system_data(session);
        }
        GeneralPage::Storage => {
            request_storage(session);
        }
        GeneralPage::Update => {
            crate::settings_uninstall::request_uninstall(session);
        }
        GeneralPage::Root => {}
    }
}

/// Repide los datos al volver con la píldora a un apartado que aún no los tiene.
pub(crate) fn refresh_after_navigation(session: &mut DockSession) {
    if is_data_page(session.settings.general()) && session.settings.system_summary().is_none() {
        request_system_data(session);
    }
    if session.settings.general() == GeneralPage::Storage
        && session.settings.storage_breakdown().is_none()
    {
        request_storage(session);
    }
    if session.settings.general() == GeneralPage::Update {
        crate::settings_uninstall::request_uninstall(session);
    }
}

/// Abre la página del sistema del botón pulsado. Falla sola.
pub(crate) fn open_about_page(session: &DockSession, button: AboutButton) {
    use dokka_contracts::SystemPage;
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
    use dokka_contracts::SystemPage;
    let page = match button {
        GeneralButton::DateTimeSettings => SystemPage::DateTime,
        GeneralButton::LanguageSettings => SystemPage::Region,
    };
    if let Err(error) = session.system_info.open_page(page) {
        eprintln!("Dokka no pudo abrir la página del sistema: {error}");
    }
}

/// Rueda sobre Preferencias: baja la ficha del apartado de General.
pub(crate) fn scroll_settings(session: &mut DockSession, delta: i16) {
    if session.settings.is_open() {
        session.settings.scroll_wheel(delta);
    }
}
