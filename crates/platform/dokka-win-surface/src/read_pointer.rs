use windows::Win32::UI::Input::KeyboardAndMouse::{
    TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent, VK_BACK, VK_ESCAPE, VK_RETURN,
};
use windows::Win32::UI::WindowsAndMessaging::{
    MSG, WM_CHAR, WM_KEYDOWN, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL,
    WM_RBUTTONUP,
};

use crate::SurfaceEvent;

/// `WM_MOUSELEAVE`, que la librería declara junto a controles que no usamos.
const MOUSE_LEFT: u32 = 0x02A3;
/// Bit de `wParam` que indica la tecla Mayús pulsada.
const SHIFT_HELD: usize = 0x0004;

/// Traduce mensajes del mouse y del teclado. `is_pressed` recuerda que el botón bajó aquí.
/// Las teclas de escritura llegan por `WM_CHAR`, que Windows genera al traducir
/// `WM_KEYDOWN`; las de control se leen de `WM_KEYDOWN` y la rueda de `WM_MOUSEWHEEL`.
pub(crate) fn read_pointer(
    message: &MSG,
    scale: f32,
    is_pressed: &mut bool,
) -> Option<SurfaceEvent> {
    match message.message {
        WM_KEYDOWN if message.wParam.0 == VK_ESCAPE.0 as usize => Some(SurfaceEvent::Dismissed),
        WM_KEYDOWN if message.wParam.0 == VK_RETURN.0 as usize => {
            Some(SurfaceEvent::LaunchpadEnter)
        }
        WM_KEYDOWN if message.wParam.0 == VK_BACK.0 as usize => {
            Some(SurfaceEvent::LaunchpadBackspace)
        }
        WM_CHAR => typed_character(message),
        WM_MOUSEWHEEL => Some(SurfaceEvent::LaunchpadWheel {
            delta: wheel_delta(message),
        }),
        WM_MOUSEMOVE => {
            let (x, y) = pointer_position(message, scale);
            let mut request = TRACKMOUSEEVENT {
                cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
                dwFlags: TME_LEAVE,
                hwndTrack: message.hwnd,
                dwHoverTime: 0,
            };
            // SAFETY: `request` está completa; pide el aviso de salida del cursor.
            let _ = unsafe { TrackMouseEvent(&mut request) };
            Some(SurfaceEvent::PointerMoved { x, y })
        }
        MOUSE_LEFT => {
            *is_pressed = false;
            Some(SurfaceEvent::PointerLeft)
        }
        WM_LBUTTONDOWN => {
            *is_pressed = true;
            None
        }
        WM_LBUTTONUP if std::mem::take(is_pressed) => {
            let (x, y) = pointer_position(message, scale);
            Some(SurfaceEvent::Clicked {
                x,
                y,
                with_shift: message.wParam.0 & SHIFT_HELD != 0,
            })
        }
        WM_RBUTTONUP => {
            let (x, y) = pointer_position(message, scale);
            Some(SurfaceEvent::RightClicked { x, y })
        }
        _ => None,
    }
}

/// Letra de `WM_CHAR`; los controles se ignoran.
fn typed_character(message: &MSG) -> Option<SurfaceEvent> {
    let character = char::from_u32(message.wParam.0 as u32)?;
    if character.is_control() {
        return None;
    }
    Some(SurfaceEvent::LaunchpadTyped { character })
}

/// Desplazamiento de la rueda, en unidades de Windows (120 por paso).
fn wheel_delta(message: &MSG) -> i16 {
    (message.wParam.0 >> 16) as i16
}

/// Las coordenadas llegan como dos enteros de 16 bits con signo, en píxeles.
fn pointer_position(message: &MSG, scale: f32) -> (f32, f32) {
    let x = f32::from(message.lParam.0 as i16) / scale;
    let y = f32::from((message.lParam.0 >> 16) as i16) / scale;
    (x, y)
}
