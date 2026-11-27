// Adaptado de https://github.com/crafter-station/Muelle (commit 37e843d).
// MIT License
//
// Copyright (c) 2026 Muelle contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Transiciones del sistema: apagarlas para que Windows no anime por su cuenta.
//!
//! Minimizar ya no se puede cancelar cuando avisa: la animacion propia ya
//! empezo. Por eso se apagan antes con `DWMWA_TRANSITIONS_FORCEDISABLED` y se
//! devuelven al apagar la opcion, al cerrar y al arrancar.

use std::ffi::c_void;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::{DWMWA_TRANSITIONS_FORCEDISABLED, DwmSetWindowAttribute};
use windows::Win32::UI::WindowsAndMessaging::{SPI_GETCLIENTAREAANIMATION, SystemParametersInfoW};
use windows::core::BOOL;

/// `true` si el usuario deja las animaciones de Windows activas.
pub fn animations_enabled() -> bool {
    let mut enabled = BOOL::from(false);
    // SAFETY: `enabled` es un `BOOL` escribible del tamano pedido.
    let ok = unsafe {
        SystemParametersInfoW(
            SPI_GETCLIENTAREAANIMATION,
            0,
            Some((&raw mut enabled).cast()),
            Default::default(),
        )
    };
    ok.is_ok() && enabled.as_bool()
}

/// Enciende o apaga la animacion propia de esta ventana. Nunca falla el vuelo.
fn set_transitions(window: isize, enabled: bool) {
    let handle = HWND(window as *mut c_void);
    let disabled = BOOL::from(!enabled);
    // SAFETY: `disabled` vive durante la llamada y el atributo pide un `BOOL`.
    unsafe {
        let _ = DwmSetWindowAttribute(
            handle,
            DWMWA_TRANSITIONS_FORCEDISABLED,
            (&raw const disabled).cast(),
            size_of::<BOOL>() as u32,
        );
    }
}

/// Apaga la animacion propia de esta ventana para que el genio la sustituya.
pub(crate) fn calm_window(window: isize) {
    set_transitions(window, false);
}

/// Devuelve la animacion propia a esta ventana.
pub(crate) fn restore_window(window: isize) {
    set_transitions(window, true);
}

/// Devuelve la animacion a todas estas ventanas. Se llama al apagar la opcion,
/// al cerrar Dokka y al arrancar por si un cierre brusco las dejo apagadas.
pub fn restore_all_transitions(windows: &[isize]) {
    for &window in windows {
        restore_window(window);
    }
}

/// Al arrancar, devuelve la animacion a ventanas conocidas por si quedaron
/// apagadas. Quien llama pasa las abiertas actuales; si no puede saberlas,
/// llama con una lista vacia y sigue igual.
pub fn ensure_clean_startup(windows: &[isize]) {
    restore_all_transitions(windows);
}
