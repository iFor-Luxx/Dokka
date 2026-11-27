//! Preferencias en el dock: identidad y trato especial.
//!
//! Solo la ventana de Preferencias (nunca dock, barra, paneles, vista grande,
//! Launchpad ni bandeja) va a la derecha del separador como minimizada. No se
//! puede fijar al dock ni lanzar otra instancia: "cerrar" solo la oculta.

use dokka_model::DockApp;

/// Identidad estable de Preferencias en el dock, la misma que publica el
/// seguimiento. No es un ejecutable real.
pub const SETTINGS_APP_ID: &str = "dokka-preferencias";

/// `true` si esta app del dock es la ventana de Preferencias.
pub fn is_settings_app(app: &DockApp) -> bool {
    app.info.id.as_str() == SETTINGS_APP_ID
}
