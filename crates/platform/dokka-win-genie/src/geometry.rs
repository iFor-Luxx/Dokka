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

//! Rectangulos en pixeles y marco girado para docks laterales.
//!
//! Con el dock abajo el embudo baja tal cual; en laterales el marco gira un
//! cuarto para que la ventana fluya igual hacia el borde.

use dokka_animation::GenieEdges;
use dokka_model::DockPosition;
use windows::UI::Composition::ContainerVisual;
use windows::Win32::Foundation::RECT;
use windows::core::Result;
use windows_numerics::{Matrix4x4, Vector2};

/// Rectangulo en pixeles de pantalla.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PixelRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl PixelRect {
    pub(crate) fn from_win(rect: RECT) -> Self {
        Self {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }

    pub(crate) fn to_win(self) -> RECT {
        RECT {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }

    pub(crate) fn width(self) -> i32 {
        self.right - self.left
    }

    pub(crate) fn height(self) -> i32 {
        self.bottom - self.top
    }

    pub(crate) fn is_empty(self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    pub(crate) fn intersect(self, other: Self) -> Self {
        Self {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }
}

/// Borde del dock visto por el vuelo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GenieEdge {
    Bottom,
    Left,
    Right,
}

impl GenieEdge {
    pub(crate) fn from_position(position: DockPosition) -> Self {
        match position {
            DockPosition::Left => Self::Left,
            DockPosition::Right => Self::Right,
            _ => Self::Bottom,
        }
    }

    pub(crate) fn is_vertical(self) -> bool {
        !matches!(self, Self::Bottom)
    }
}

/// Marco del vuelo: con laterales gira un cuarto sin espejar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Turn {
    pub edge: GenieEdge,
    pub width: f32,
    pub height: f32,
}

impl Turn {
    pub(crate) fn new(edge: GenieEdge, screen: PixelRect) -> Self {
        Self {
            edge,
            width: screen.width() as f32,
            height: screen.height() as f32,
        }
    }

    pub(crate) fn size(self) -> (f32, f32) {
        if self.edge.is_vertical() {
            (self.height, self.width)
        } else {
            (self.width, self.height)
        }
    }

    pub(crate) fn edges(self, local: GenieEdges) -> GenieEdges {
        match self.edge {
            GenieEdge::Bottom => local,
            GenieEdge::Left => GenieEdges {
                left: local.top,
                right: local.bottom,
                top: self.width - local.right,
                bottom: self.width - local.left,
            },
            GenieEdge::Right => GenieEdges {
                left: self.height - local.bottom,
                right: self.height - local.top,
                top: local.left,
                bottom: local.right,
            },
        }
    }

    fn matrix(self) -> Option<Matrix4x4> {
        let (m11, m12, m21, m22, m41, m42) = match self.edge {
            GenieEdge::Bottom => return None,
            GenieEdge::Left => (0.0, 1.0, -1.0, 0.0, self.width, 0.0),
            GenieEdge::Right => (0.0, -1.0, 1.0, 0.0, 0.0, self.height),
        };
        Some(Matrix4x4 {
            M11: m11,
            M12: m12,
            M21: m21,
            M22: m22,
            M33: 1.0,
            M41: m41,
            M42: m42,
            M44: 1.0,
            ..Matrix4x4::default()
        })
    }

    pub(crate) fn apply(self, root: &ContainerVisual) -> Result<()> {
        if let Some(matrix) = self.matrix() {
            let (width, height) = self.size();
            root.SetRelativeSizeAdjustment(Vector2 { X: 0.0, Y: 0.0 })?;
            root.SetSize(Vector2 {
                X: width,
                Y: height,
            })?;
            root.SetTransformMatrix(matrix)?;
        }
        Ok(())
    }

    /// Copia girada para leer igual en el marco. Rotacion pura, sin espejo.
    pub(crate) fn turn_image(self, image: crate::GenieImage) -> crate::GenieImage {
        if !self.edge.is_vertical() {
            return image;
        }
        let (w, h) = (image.width as usize, image.height as usize);
        let mut bgra = vec![0u8; w * h * 4];
        for fy in 0..w {
            for fx in 0..h {
                let (x, y) = match self.edge {
                    GenieEdge::Left => (w - 1 - fy, fx),
                    _ => (fy, h - 1 - fx),
                };
                let from = (y * w + x) * 4;
                let to = (fy * h + fx) * 4;
                bgra[to..to + 4].copy_from_slice(&image.bgra[from..from + 4]);
            }
        }
        crate::GenieImage {
            width: image.height,
            height: image.width,
            bgra,
        }
    }
}

/// Convierte un rectangulo de pantalla a coordenadas del overlay.
pub(crate) fn to_local(rect: PixelRect, screen: PixelRect) -> GenieEdges {
    GenieEdges {
        left: (rect.left - screen.left) as f32,
        top: (rect.top - screen.top) as f32,
        right: (rect.right - screen.left) as f32,
        bottom: (rect.bottom - screen.top) as f32,
    }
}

/// Monitor que contiene esta ventana, en pixeles de pantalla.
/// Si no se sabe, el primario completo. Nunca falla el vuelo.
pub fn monitor_rect_for_window(window: isize) -> [i32; 4] {
    use std::ffi::c_void;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITORINFO};
    use windows::Win32::Graphics::Gdi::{MONITOR_DEFAULTTONEAREST, MonitorFromWindow};
    let handle = HWND(window as *mut c_void);
    // SAFETY: consultas de solo lectura; un handle destruido da monitor nulo.
    unsafe {
        let monitor = MonitorFromWindow(handle, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if GetMonitorInfoW(monitor, &mut info).as_bool() {
            let area = info.rcMonitor;
            if area.right > area.left && area.bottom > area.top {
                return [area.left, area.top, area.right, area.bottom];
            }
        }
        primary_screen_rect()
    }
}

/// Primario completo en pixeles. Reserva si el monitor no se sabe.
pub fn primary_screen_rect() -> [i32; 4] {
    use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
    // SAFETY: metricas del sistema sin argumentos de salida.
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN).max(1);
        let height = GetSystemMetrics(SM_CYSCREEN).max(1);
        [0, 0, width, height]
    }
}

/// Hueco logico del dock a pixeles de pantalla con su rectangulo y escala.
pub fn dock_slot_to_screen(dock: [i32; 4], slot: [f32; 4], scale: f32) -> [i32; 4] {
    let left = dock[0] + (slot[0] * scale).round() as i32;
    let top = dock[1] + (slot[1] * scale).round() as i32;
    let right = dock[0] + ((slot[0] + slot[2]) * scale).round() as i32;
    let bottom = dock[1] + ((slot[1] + slot[3]) * scale).round() as i32;
    [left, top, right.max(left + 1), bottom.max(top + 1)]
}
