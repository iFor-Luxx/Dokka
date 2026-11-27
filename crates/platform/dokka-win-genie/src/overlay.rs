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

//! Ventana transparente que cubre la pantalla solo durante el vuelo.
//!
//! No recibe clics y esta por encima de todo sin activarse. La foto fija va
//! en una ventana propia debajo de las tiras para que DWM la componga aparte.

use std::sync::OnceLock;

use windows::Foundation::TimeSpan;
use windows::UI::Composition::Compositor;
use windows::UI::Composition::{ContainerVisual, Desktop::DesktopWindowTarget};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::WinRT::Composition::ICompositorDesktopInterop;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassW, SW_SHOWNOACTIVATE,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetWindowPos, ShowWindow, WNDCLASSW, WS_EX_LAYERED,
    WS_EX_NOACTIVATE, WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
    WS_EX_TRANSPARENT, WS_POPUP,
};
use windows::core::{Interface, Result, w};

use crate::geometry::PixelRect;

unsafe extern "system" fn overlay_proc(
    window: HWND,
    message: u32,
    wparam: windows::Win32::Foundation::WPARAM,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    // SAFETY: reenvio estandar; el overlay no atiende mensajes.
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// Ventana transparente a pantalla completa para un vuelo.
pub(crate) fn create_window(screen: PixelRect) -> Option<HWND> {
    static CLASS: OnceLock<bool> = OnceLock::new();
    let class = w!("DokkaGenie");
    // SAFETY: registro y creacion estandar en el hilo de interfaz; si falla la
    // ventana propia se destruye aqui, o la libera `Flight::drop`.
    unsafe {
        let instance = GetModuleHandleW(None).ok()?;
        CLASS.get_or_init(|| {
            RegisterClassW(&WNDCLASSW {
                lpfnWndProc: Some(overlay_proc),
                hInstance: instance.into(),
                lpszClassName: class,
                ..Default::default()
            }) != 0
        });
        CreateWindowExW(
            WS_EX_LAYERED
                | WS_EX_TRANSPARENT
                | WS_EX_NOREDIRECTIONBITMAP
                | WS_EX_TOPMOST
                | WS_EX_TOOLWINDOW
                | WS_EX_NOACTIVATE,
            class,
            w!(""),
            WS_POPUP,
            screen.left,
            screen.top,
            screen.width(),
            screen.height(),
            None,
            None,
            Some(instance.into()),
            None,
        )
        .ok()
    }
}

/// Arbol visual del overlay ya enlazado a su ventana.
pub(crate) fn create_overlay(
    compositor: &Compositor,
    screen: PixelRect,
) -> Option<(HWND, DesktopWindowTarget, ContainerVisual)> {
    let overlay = create_window(screen)?;
    // SAFETY: `overlay` es la ventana viva creada arriba; si falla se destruye
    // aqui, o la libera `Flight::drop`.
    unsafe {
        let tree = || -> Result<(DesktopWindowTarget, ContainerVisual)> {
            let target = compositor
                .cast::<ICompositorDesktopInterop>()?
                .CreateDesktopWindowTarget(overlay, true)?;
            let root = compositor.CreateContainerVisual()?;
            root.SetRelativeSizeAdjustment(windows_numerics::Vector2 { X: 1.0, Y: 1.0 })?;
            target.SetRoot(&root)?;
            Ok((target, root))
        };
        match tree() {
            Ok((target, root)) => Some((overlay, target, root)),
            Err(_) => {
                let _ = DestroyWindow(overlay);
                None
            }
        }
    }
}

/// Muestra una ventana propia sin activarla.
pub(crate) fn show_no_activate(window: HWND) {
    // SAFETY: ventana propia y viva.
    unsafe {
        let _ = ShowWindow(window, SW_SHOWNOACTIVATE);
    }
}

/// Pone el vuelo justo debajo del dock para que pase tras su cristal.
pub(crate) fn tuck_under(overlay: HWND, stand: Option<HWND>, dock: HWND) {
    let flags = SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE;
    // SAFETY: solo reordena ventanas propias y vivas.
    unsafe {
        let _ = SetWindowPos(overlay, Some(dock), 0, 0, 0, 0, flags);
        if let Some(stand) = stand {
            let _ = SetWindowPos(stand, Some(overlay), 0, 0, 0, 0, flags);
        }
    }
}

/// Duracion del vuelo para el compositor.
pub(crate) fn flight_duration(seconds: f32) -> TimeSpan {
    TimeSpan {
        Duration: (seconds * 1e7) as i64,
    }
}

/// Destruye una ventana propia una sola vez.
pub(crate) fn destroy(window: HWND) {
    // SAFETY: ventana propia creada en `create_window`.
    unsafe {
        let _ = DestroyWindow(window);
    }
}
