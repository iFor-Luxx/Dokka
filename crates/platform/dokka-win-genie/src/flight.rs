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

//! Vuelo del genio: foto fija, tiras y restauracion al final.
//!
//! Al minimizar se muestra una miniatura DWM donde estaba la ventana mientras
//! un hilo copia la pantalla; al llegar la copia despegan las tiras. Al
//! restaurar vuelan desde la foto reducida y la ventana real aparece debajo
//! al llegar a tamano completo.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use dokka_animation::Genie;
use windows::UI::Composition::{ContainerVisual, Desktop::DesktopWindowTarget};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::{
    DWM_THUMBNAIL_PROPERTIES, DWM_TNP_OPACITY, DWM_TNP_RECTDESTINATION,
    DWM_TNP_SOURCECLIENTAREAONLY, DWM_TNP_VISIBLE, DwmRegisterThumbnail, DwmUnregisterThumbnail,
    DwmUpdateThumbnailProperties,
};
use windows::Win32::UI::WindowsAndMessaging::{
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
};

use crate::GenieImage;
use crate::capture::{capture_win, halved};
use crate::geometry::{GenieEdge, PixelRect, Turn, to_local};
use crate::overlay::{create_overlay, destroy, show_no_activate};
use crate::phase::{Captured, Phase};
use crate::strips::GenieCompositor;

/// Solo una copia de pantalla viva a la vez, incluido un hilo huerfano.
static CAPTURE_BUSY: AtomicBool = AtomicBool::new(false);

struct CapturePermit;

impl CapturePermit {
    fn acquire() -> Option<Self> {
        CAPTURE_BUSY
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .ok()
            .map(|_| Self)
    }
}

impl Drop for CapturePermit {
    fn drop(&mut self) {
        CAPTURE_BUSY.store(false, Ordering::Release);
    }
}

/// Un vuelo en curso hacia el dock o de vuelta.
pub struct Flight {
    pub hwnd: isize,
    pub(crate) created_at: Instant,
    pub(crate) seconds: f32,
    pub(crate) foreground: isize,
    pub(crate) overlay: HWND,
    pub(crate) stand: Option<HWND>,
    /// Sin lienzo, DWM no pinta la miniatura en una ventana sin redirección.
    pub(crate) _stand_target: Option<DesktopWindowTarget>,
    pub(crate) _target: DesktopWindowTarget,
    pub(crate) root: ContainerVisual,
    pub(crate) screen: PixelRect,
    pub(crate) turn: Turn,
    pub(crate) source: PixelRect,
    pub(crate) genie: Genie,
    pub(crate) aimed: bool,
    pub(crate) phase: Phase,
    pub(crate) restoring: bool,
    pub(crate) restore_pending: bool,
    pub(crate) captured: Option<(GenieImage, PixelRect)>,
}

impl Flight {
    /// Foto fija donde estaba la ventana; despegara con `aim`.
    /// `from` es la ventana entera en pixeles de pantalla (con bordes
    /// invisibles, donde va la miniatura DWM); `frame` es lo que se veia
    /// (lo que se copia y vuela). `screen` es el monitor del overlay.
    pub(crate) fn start(
        gfx: &GenieCompositor,
        hwnd: isize,
        (from, frame): (PixelRect, PixelRect),
        screen: PixelRect,
        seconds: f32,
        edge: GenieEdge,
        foreground: isize,
    ) -> Option<Self> {
        use std::sync::mpsc::channel;
        let turn = Turn::new(edge, screen);
        let _permit = CapturePermit::acquire().or_else(|| {
            eprintln!("genio: sin vuelo para {hwnd}: otra copia de pantalla en curso");
            None
        })?;
        // Solo vuela lo que se ve.
        let visible = frame.intersect(screen);
        if visible.is_empty() {
            eprintln!(
                "genio: sin vuelo para {hwnd}: nada visible en [{}, {}, {}, {}]",
                frame.left, frame.top, frame.right, frame.bottom
            );
            return None;
        }
        let (stand, stand_target, _) = create_overlay(&gfx.compositor, screen).or_else(|| {
            eprintln!("genio: sin vuelo para {hwnd}: no se creo la ventana de la foto fija");
            None
        })?;
        let Some((overlay, target, root)) = create_overlay(&gfx.compositor, screen) else {
            eprintln!("genio: sin vuelo para {hwnd}: no se creo el overlay del vuelo");
            destroy(stand);
            return None;
        };
        if turn.apply(&root).is_err() {
            eprintln!("genio: sin vuelo para {hwnd}: no se giro el marco del vuelo");
            destroy(stand);
            destroy(overlay);
            return None;
        }
        // SAFETY: ambas son ventanas propias y vivas; si falla se destruyen
        // aqui, o las libera `Drop`.
        let thumbnail = unsafe {
            let registered = DwmRegisterThumbnail(stand, HWND(hwnd as *mut _)).inspect_err(|error| {
                let code = error.code().0;
                eprintln!("genio: sin vuelo para {hwnd}: no se registro la miniatura DWM (HRESULT {code:#X})");
            }).ok()?;
            let properties = DWM_THUMBNAIL_PROPERTIES {
                dwFlags: DWM_TNP_RECTDESTINATION
                    | DWM_TNP_VISIBLE
                    | DWM_TNP_OPACITY
                    | DWM_TNP_SOURCECLIENTAREAONLY,
                rcDestination: windows::Win32::Foundation::RECT {
                    left: from.left - screen.left,
                    top: from.top - screen.top,
                    right: from.right - screen.left,
                    bottom: from.bottom - screen.top,
                },
                opacity: 255,
                fVisible: true.into(),
                fSourceClientAreaOnly: false.into(),
                ..Default::default()
            };
            let _ = DwmUpdateThumbnailProperties(registered, &properties);
            show_no_activate(overlay);
            let _ = SetWindowPos(
                stand,
                Some(overlay),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
            registered
        };
        let visible_win = visible.to_win();
        let (sender, shot) = channel();
        let built = std::thread::Builder::new()
            .name("dokka-genie-capture".into())
            .spawn(move || {
                let _permit = _permit;
                let shot = capture_win(visible_win).map(|image| {
                    let image = turn.turn_image(image);
                    Captured {
                        restore: halved(&image),
                        image,
                    }
                });
                let _ = sender.send(shot);
            });
        if built.is_err() {
            eprintln!("genio: sin vuelo para {hwnd}: no se lanzo el hilo de copia");
            // SAFETY: recursos creados para este vuelo fallido.
            unsafe {
                let _ = DwmUnregisterThumbnail(thumbnail);
            }
            destroy(stand);
            destroy(overlay);
            return None;
        }
        let window = turn.edges(to_local(visible, screen));
        Some(Self {
            hwnd,
            created_at: Instant::now(),
            seconds,
            foreground,
            overlay,
            stand: Some(stand),
            _stand_target: Some(stand_target),
            _target: target,
            root,
            screen,
            turn,
            source: visible,
            genie: Genie {
                window,
                slot: window,
            },
            aimed: false,
            phase: Phase::Standing {
                thumbnail,
                frames: 0,
                shot,
                started: Instant::now(),
            },
            restoring: false,
            restore_pending: false,
            captured: None,
        })
    }
}
