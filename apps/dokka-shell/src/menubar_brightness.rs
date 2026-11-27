//! Brillo de la barra: lectura sin esperar y ejecución de su acción.
//!
//! La isla guarda el nivel y decide el gesto; aquí solo se lee el último valor
//! conocido de `SystemBrightness` y se envía la orden sin esperar (el DDC
//! tarda decenas o cientos de milisegundos en su propio hilo). La relectura
//! DDC se pide solo al abrir el centro de control, no en bucle.

use dokka_contracts::BrightnessControl;

use crate::dock_session::DockSession;

/// Último brillo conocido a la isla; sin DDC se oculta la tarjeta.
/// Barato y sin esperar: el hilo DDC ya hizo el trabajo.
pub(crate) fn refresh_brightness(session: &mut DockSession) {
    session
        .menubar
        .set_brightness(session.brightness.brightness());
}

/// Relee el brillo en el hilo DDC (por si cambió con los botones del monitor).
/// Llamar al abrir el centro de control; el valor llega en los `Tick` siguientes.
pub(crate) fn request_brightness(session: &mut DockSession) {
    session.brightness.request_refresh();
    refresh_brightness(session);
}

/// Fija el nivel elegido en el deslizador y adelanta la isla para que no salte.
pub(crate) fn set_brightness_level(session: &mut DockSession, level: f32) {
    session.brightness.set_brightness(level);
    refresh_brightness(session);
}
