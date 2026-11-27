//! Reenvía el clic a la app dueña del icono.
//!
//! Antes se deja a la app pasar al frente; luego se le publica su mensaje
//! con el botón izquierdo o el derecho. La versión 4 lleva el cursor y el
//! evento empaquetados, como manda la bandeja de Windows.

use std::ffi::c_void;

use dokka_contracts::ContractError;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    AllowSetForegroundWindow, GetCursorPos, GetWindowThreadProcessId, IsWindow, PostMessageW,
    WM_CONTEXTMENU, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP,
};

use crate::state::ClickTarget;

/// La app lo trata como el clic izquierdo.
const NIN_SELECT: u32 = 1024;

/// Publica el clic; `right` pide el menú en vez de activar.
pub(crate) fn press(target: &ClickTarget, right: bool) -> Result<(), ContractError> {
    let window = HWND(target.hwnd_raw as usize as *mut c_void);
    // SAFETY: `IsWindow` admite cualquier valor; el resto avisa a la app
    // sin bloquear y deja que el menú lo abra ella.
    unsafe {
        if !IsWindow(Some(window)).as_bool() {
            return Err(ContractError::NotFound);
        }
        let mut owner = 0;
        GetWindowThreadProcessId(window, Some(&mut owner));
        if owner != 0 {
            let _ = AllowSetForegroundWindow(owner);
        }
        if target.version >= 4 {
            press_v4(window, target, right)
        } else {
            press_legacy(window, target, right)
        }
    }
}

/// Versión clásica: el número va en `wParam` y el evento en `lParam`.
fn press_legacy(window: HWND, target: &ClickTarget, right: bool) -> Result<(), ContractError> {
    let (down, up) = if right {
        (WM_RBUTTONDOWN, WM_RBUTTONUP)
    } else {
        (WM_LBUTTONDOWN, WM_LBUTTONUP)
    };
    // SAFETY: publicado sin espera; un handle destruido falla sin más.
    unsafe {
        let first = PostMessageW(
            Some(window),
            target.callback,
            WPARAM(target.uid as usize),
            LPARAM(down as isize),
        );
        let second = PostMessageW(
            Some(window),
            target.callback,
            WPARAM(target.uid as usize),
            LPARAM(up as isize),
        );
        if first.is_err() && second.is_err() {
            return Err(ContractError::Failed(
                "la app no recibió el clic".to_owned(),
            ));
        }
    }
    Ok(())
}

/// Versión 4: el cursor va en `wParam` y (evento, número) en `lParam`.
fn press_v4(window: HWND, target: &ClickTarget, right: bool) -> Result<(), ContractError> {
    // SAFETY: lectura del cursor; si falla se publica desde la esquina.
    let (x, y) = unsafe {
        let mut point = POINT::default();
        if GetCursorPos(&mut point).is_err() {
            (0, 0)
        } else {
            (point.x, point.y)
        }
    };
    let cursor = WPARAM(((x as u32) & 0xFFFF | (y as u32).wrapping_shl(16)) as usize);
    let (down, up, last) = if right {
        (WM_RBUTTONDOWN, WM_RBUTTONUP, WM_CONTEXTMENU)
    } else {
        (WM_LBUTTONDOWN, WM_LBUTTONUP, NIN_SELECT)
    };
    // SAFETY: publicados sin espera, en el orden que espera la bandeja.
    unsafe {
        let first = PostMessageW(
            Some(window),
            target.callback,
            cursor,
            LPARAM(pack(down, target.uid)),
        );
        let second = PostMessageW(
            Some(window),
            target.callback,
            cursor,
            LPARAM(pack(up, target.uid)),
        );
        let third = PostMessageW(
            Some(window),
            target.callback,
            cursor,
            LPARAM(pack(last, target.uid)),
        );
        if first.is_err() && second.is_err() && third.is_err() {
            return Err(ContractError::Failed(
                "la app no recibió el clic".to_owned(),
            ));
        }
    }
    Ok(())
}

/// Empaqueta (evento abajo, número arriba) sin desbordar en depuración.
fn pack(event: u32, uid: u32) -> isize {
    (uid.wrapping_shl(16) | (event & 0xFFFF)) as isize
}
