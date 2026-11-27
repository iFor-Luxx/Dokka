//! Salidas de audio activas de Windows y cambio de predeterminada.
//!
//! La lista usa la API oficial (`IMMDeviceEnumerator`); el cambio usa la
//! interfaz sin documentar `IPolicyConfig`, aislado en este crate para que
//! un fallo no arrastre al resto.
#![allow(unsafe_code)]

mod outputs;
mod policy;

pub use outputs::WinAudioOutputs;
