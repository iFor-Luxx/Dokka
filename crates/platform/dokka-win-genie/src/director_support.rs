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

//! Apoyos del director: rectángulos, handles y creación del compositor.

use windows::Win32::Foundation::HWND;

use crate::strips::GenieCompositor;

/// Crea el compositor propio del vuelo si aún no existe.
/// Solo lo intenta una vez: si Windows falla, los vuelos siguen su curso.
pub(crate) fn ensure_compositor(
    gfx: &mut Option<GenieCompositor>,
    gfx_failed: &mut bool,
    hwnd: isize,
) -> Option<()> {
    if gfx.is_none() && !*gfx_failed {
        match GenieCompositor::new() {
            Ok(created) => *gfx = Some(created),
            Err(error) => {
                *gfx_failed = true;
                let code = error.code().0;
                eprintln!(
                    "genio: sin vuelo para {hwnd}: no se preparo el compositor (HRESULT {code:#X})"
                );
            }
        }
    }
    gfx.as_ref().map(|_| ())
}

pub(crate) fn rect_of(area: [i32; 4]) -> windows::Win32::Foundation::RECT {
    windows::Win32::Foundation::RECT {
        left: area[0],
        top: area[1],
        right: area[2],
        bottom: area[3],
    }
}

/// Deja pasar el tipo del handle sin exponer Windows fuera de `platform`.
pub(crate) fn dock_handle(raw: isize) -> HWND {
    HWND(raw as *mut std::ffi::c_void)
}
