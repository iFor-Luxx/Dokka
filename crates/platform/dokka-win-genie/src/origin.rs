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

//! De dónde sale el vuelo: ventana entera y parte visible antes de minimizar.
//!
//! La ventana ya no dice dónde estaba cuando avisa (está en -32000), así que
//! se usa lo último visto. Una maximizada llena su área de trabajo más sus
//! bordes: su rectángulo "normal" es el pequeño al que vuelve, y volar desde
//! él hacía saltar el genio. Por eso la foto fija va donde estaba la ventana
//! entera (con bordes invisibles) y la copia vuela desde lo que se veía.

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_STYLE, GetSystemMetrics, GetWindowLongW, GetWindowPlacement, GetWindowRect, IsIconic,
    SM_CXPADDEDBORDER, SM_CXSIZEFRAME, WINDOWPLACEMENT, WPF_RESTORETOMAXIMIZED, WS_THICKFRAME,
};

use crate::geometry::PixelRect;

fn win_rect(area: PixelRect) -> RECT {
    area.to_win()
}

/// Rectángulo restaurado de una ventana, válido aun minimizada.
/// `rcNormalPosition` usa coordenadas del área de trabajo: se suma el
/// desplazamiento entre área y pantalla para volver a píxeles de pantalla.
fn normal_rect(window: HWND) -> Option<PixelRect> {
    // SAFETY: la estructura declara su tamaño y vive durante la llamada.
    let mut placement = WINDOWPLACEMENT {
        length: size_of::<WINDOWPLACEMENT>() as u32,
        ..Default::default()
    };
    // SAFETY: `placement` declara su tamaño y es escribible.
    unsafe { GetWindowPlacement(window, &mut placement) }.ok()?;
    // Desplazamiento entre área de trabajo y pantalla en el monitor primario
    // (la barra superior de Dokka lo mueve, como la de Muelle).
    let (dx, dy) = primary_work_offset();
    let local = placement.rcNormalPosition;
    let rect = PixelRect {
        left: local.left + dx,
        top: local.top + dy,
        right: local.right + dx,
        bottom: local.bottom + dy,
    };
    (!rect.is_empty()).then_some(rect)
}

/// Desplazamiento entre área de trabajo y pantalla del monitor primario.
fn primary_work_offset() -> (i32, i32) {
    use windows::Win32::Graphics::Gdi::{MONITOR_DEFAULTTOPRIMARY, MonitorFromWindow};
    use windows::Win32::UI::WindowsAndMessaging::{SPI_GETWORKAREA, SystemParametersInfoW};
    // SAFETY: consultas de solo lectura con estructuras propias.
    unsafe {
        let monitor = MonitorFromWindow(HWND(core::ptr::null_mut()), MONITOR_DEFAULTTOPRIMARY);
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(monitor, &mut info).as_bool() {
            return (0, 0);
        }
        let mut work = RECT::default();
        if SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some((&raw mut work).cast()),
            Default::default(),
        )
        .is_err()
        {
            return (0, 0);
        }
        (
            work.left - info.rcMonitor.left,
            work.top - info.rcMonitor.top,
        )
    }
}

/// Dónde está una ventana ahora mismo, si no está minimizada.
fn live_rect(window: HWND) -> Option<PixelRect> {
    // SAFETY: consultas de solo lectura; un handle destruido devuelve cero.
    unsafe {
        if IsIconic(window).as_bool() {
            return None;
        }
        let mut rect = RECT::default();
        GetWindowRect(window, &mut rect).ok()?;
        let rect = PixelRect::from_win(rect);
        (!rect.is_empty()).then_some(rect)
    }
}

/// Área de trabajo del monitor de esta ventana, en píxeles de pantalla.
fn work_area(window: HWND) -> Option<PixelRect> {
    // SAFETY: monitor e info propios y vivos durante la llamada.
    unsafe {
        let monitor = MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(monitor, &mut info).as_bool() {
            return None;
        }
        let area = PixelRect::from_win(info.rcWork);
        (!area.is_empty()).then_some(area)
    }
}

/// Ventana entera (con bordes invisibles) y parte visible antes de minimizar.
/// `last_seen` es lo último visto sin maximizar, si lo hay.
pub(crate) fn rect_before_minimize_raw(
    hwnd: isize,
    last_seen: Option<PixelRect>,
) -> Option<(PixelRect, PixelRect)> {
    let window = HWND(hwnd as *mut core::ffi::c_void);
    let mut placement = WINDOWPLACEMENT {
        length: size_of::<WINDOWPLACEMENT>() as u32,
        ..Default::default()
    };
    // SAFETY: la estructura declara su tamaño y vive durante la llamada.
    unsafe { GetWindowPlacement(window, &mut placement) }.ok()?;
    if placement.flags.0 & WPF_RESTORETOMAXIMIZED.0 == 0 {
        let rect = last_seen
            .or_else(|| live_rect(window))
            .or_else(|| normal_rect(window))?;
        return Some((rect, rect));
    }
    // Maximizada: la ventana entera desborda el área con sus bordes.
    // SAFETY: solo lee estilos y métricas del sistema.
    let resizable = unsafe { GetWindowLongW(window, GWL_STYLE) as u32 & WS_THICKFRAME.0 != 0 };
    let border = if resizable {
        // SAFETY: consultas de medidas del sistema sin punteros.
        unsafe { GetSystemMetrics(SM_CXSIZEFRAME) + GetSystemMetrics(SM_CXPADDEDBORDER) }
    } else {
        0
    };
    let work = work_area(window)?;
    let whole = PixelRect {
        left: work.left - border,
        top: work.top - border,
        right: work.right + border,
        bottom: work.bottom + border,
    };
    Some((whole, work))
}

/// Versión con arreglos `[izquierda, arriba, derecha, abajo]` para el shell.
pub fn rect_before_minimize(
    hwnd: isize,
    last_seen: Option<[i32; 4]>,
) -> Option<([i32; 4], [i32; 4])> {
    let seen = last_seen.map(|area| PixelRect {
        left: area[0],
        top: area[1],
        right: area[2],
        bottom: area[3],
    });
    let (from, frame) = rect_before_minimize_raw(hwnd, seen)?;
    let rect = win_rect(from);
    let frame = win_rect(frame);
    Some((
        [rect.left, rect.top, rect.right, rect.bottom],
        [frame.left, frame.top, frame.right, frame.bottom],
    ))
}

/// `true` si la ventana sigue minimizada ahora mismo.
pub(crate) fn is_iconic_now(hwnd: isize) -> bool {
    // SAFETY: consulta de solo lectura; un handle destruido dice que no.
    unsafe { IsIconic(HWND(hwnd as *mut core::ffi::c_void)).as_bool() }
}
