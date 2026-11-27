//! Bandeja de Windows: iconos de apps en segundo plano.
//!
//! Un hilo propio con una ventana oculta recibe los iconos y reenvía los
//! clics; todo lo dirigido a la barra se reenvía a la real para que
//! Windows siga funcionando igual. Pieza aislada: si falla, la bandeja
//! queda vacía y Dokka sigue funcionando.
#![allow(unsafe_code)]

mod click;
mod forward;
mod intake;
mod notify;
mod owner;
mod state;
mod tray_host;
mod window;

pub use tray_host::TrayHost;
