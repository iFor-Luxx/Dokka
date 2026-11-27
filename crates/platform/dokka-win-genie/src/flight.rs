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
use std::sync::mpsc::{Receiver, channel};
use std::time::Instant;

use dokka_animation::Genie;
use windows::UI::Composition::{
    CompositionPropertySet, ContainerVisual, Desktop::DesktopWindowTarget,
};
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
use crate::overlay::{create_overlay, create_window, destroy, show_no_activate};
use crate::snapshot::Snapshot;
use crate::strips::{GenieCompositor, bind, clip_at_slot, cut, launch};

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

pub(crate) struct Captured {
    pub(crate) image: GenieImage,
    pub(crate) restore: GenieImage,
}

pub(crate) enum Phase {
    Standing {
        thumbnail: isize,
        frames: u32,
        shot: Receiver<Option<Captured>>,
        started: Instant,
    },
    Flying {
        start: Instant,
        _drive: CompositionPropertySet,
        stand_in: Option<(isize, u32)>,
    },
    Settling {
        until: Instant,
    },
}

/// Un vuelo en curso hacia el dock o de vuelta.
pub struct Flight {
    pub hwnd: isize,
    pub(crate) created_at: Instant,
    pub(crate) seconds: f32,
    pub(crate) foreground: isize,
    pub(crate) overlay: HWND,
    pub(crate) stand: Option<HWND>,
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
    /// `from` es la ventana entera en pixeles de pantalla.
    pub(crate) fn start(
        gfx: &GenieCompositor,
        hwnd: isize,
        from: PixelRect,
        screen: PixelRect,
        seconds: f32,
        edge: GenieEdge,
        foreground: isize,
    ) -> Option<Self> {
        let turn = Turn::new(edge, screen);
        let _permit = CapturePermit::acquire()?;
        let visible = from.intersect(screen);
        if visible.is_empty() {
            return None;
        }
        let stand = create_window(screen)?;
        let Some((overlay, target, root)) = create_overlay(&gfx.compositor, screen) else {
            destroy(stand);
            return None;
        };
        if turn.apply(&root).is_err() {
            destroy(stand);
            destroy(overlay);
            return None;
        }
        // SAFETY: ambas son ventanas propias y vivas; si falla se destruyen
        // aqui, o las libera `Drop`.
        let thumbnail = unsafe {
            let registered = DwmRegisterThumbnail(stand, HWND(hwnd as *mut _)).ok()?;
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

    /// Vuelo de vuelta desde la foto reducida hasta su hueco original.
    pub(crate) fn restore(
        gfx: &GenieCompositor,
        hwnd: isize,
        snapshot: &Snapshot,
        slot: PixelRect,
        screen: PixelRect,
        edge: GenieEdge,
        foreground: isize,
    ) -> Option<Self> {
        let source = snapshot.window.intersect(screen);
        if source.is_empty() || snapshot.edge != edge {
            return None;
        }
        let turn = Turn::new(edge, screen);
        let (overlay, target, root) = create_overlay(&gfx.compositor, screen)?;
        let genie = Genie {
            window: turn.edges(to_local(source, screen)),
            slot: turn.edges(to_local(slot, screen)),
        };
        let height = (genie.window.bottom - genie.window.top) as u32;
        let image = &snapshot.image;
        let size = (image.width as f32, image.height as f32);
        let drive = turn
            .apply(&root)
            .and_then(|()| clip_at_slot(gfx, &root, &genie, turn))
            .and_then(|()| cut(gfx, &root, image, height))
            .and_then(|pieces| bind(gfx, &genie, &pieces, size, true))
            .and_then(|drive| {
                launch(gfx, &genie, &drive, true, dokka_animation::GENIE_SECONDS).map(|()| drive)
            });
        let Ok(drive) = drive else {
            destroy(overlay);
            return None;
        };
        let flight = Self {
            hwnd,
            created_at: Instant::now(),
            seconds: dokka_animation::GENIE_SECONDS,
            foreground,
            overlay,
            stand: None,
            _target: target,
            root,
            screen,
            turn,
            source,
            genie,
            aimed: true,
            phase: Phase::Flying {
                start: Instant::now(),
                _drive: drive,
                stand_in: None,
            },
            restoring: true,
            restore_pending: true,
            captured: None,
        };
        show_no_activate(flight.overlay);
        Some(flight)
    }
}
