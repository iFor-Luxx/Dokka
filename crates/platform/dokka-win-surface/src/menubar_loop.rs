use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetFocus, ReleaseCapture, SetCapture, TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent, VK_BACK,
    VK_DOWN, VK_ESCAPE, VK_RETURN, VK_UP,
};
use windows::Win32::UI::WindowsAndMessaging::{
    MSG, WM_CHAR, WM_KEYDOWN, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
};

use crate::SurfaceEvent;
use crate::read_pointer::pointer_position;

/// `WM_MOUSELEAVE`, que la librería declara junto a controles que no usamos.
const MOUSE_LEFT: u32 = 0x02A3;

/// Traduce un mensaje del ratón o del teclado de la barra a su evento propio.
/// El botón abajo avisa como `MenubarPressed` y captura el ratón para que el
/// movimiento y la suelta lleguen aunque el cursor salga de la ventana; al
/// soltar se libera la captura. Las teclas solo llegan con foco en la barra;
/// Esc sigue llegando como `MenubarDismissed` igual que la pérdida de foco.
pub(crate) fn read_menubar(
    message: &MSG,
    scale: f32,
    is_pressed: &mut bool,
) -> Option<SurfaceEvent> {
    match message.message {
        WM_KEYDOWN if message.wParam.0 == VK_ESCAPE.0 as usize => {
            Some(SurfaceEvent::MenubarDismissed)
        }
        WM_KEYDOWN if message.wParam.0 == VK_RETURN.0 as usize => {
            focused_event(message.hwnd, SurfaceEvent::MenubarEnter)
        }
        WM_KEYDOWN if message.wParam.0 == VK_BACK.0 as usize => {
            focused_event(message.hwnd, SurfaceEvent::MenubarBackspace)
        }
        WM_KEYDOWN if message.wParam.0 == VK_UP.0 as usize => {
            focused_event(message.hwnd, SurfaceEvent::MenubarUp)
        }
        WM_KEYDOWN if message.wParam.0 == VK_DOWN.0 as usize => {
            focused_event(message.hwnd, SurfaceEvent::MenubarDown)
        }
        WM_CHAR => typed_character(message),
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
            Some(SurfaceEvent::MenubarMoved { x, y })
        }
        MOUSE_LEFT => {
            if *is_pressed {
                // Con captura el movimiento sigue llegando fuera; no se pierde la pulsación.
                return None;
            }
            Some(SurfaceEvent::MenubarLeft)
        }
        WM_LBUTTONDOWN => {
            *is_pressed = true;
            // SAFETY: la ventana recibe el mensaje y sigue viva; captura el ratón.
            unsafe {
                SetCapture(message.hwnd);
            }
            let (x, y) = pointer_position(message, scale);
            Some(SurfaceEvent::MenubarPressed { x, y })
        }
        WM_LBUTTONUP => {
            let was_pressed = std::mem::take(is_pressed);
            // SAFETY: libera la captura tomada al pulsar; sin captura es inofensivo.
            unsafe {
                let _ = ReleaseCapture();
            }
            if !was_pressed {
                return None;
            }
            let (x, y) = pointer_position(message, scale);
            Some(SurfaceEvent::MenubarClicked { x, y })
        }
        _ => None,
    }
}

/// Solo entrega el evento si la barra tiene el foco del teclado.
fn focused_event(window: HWND, event: SurfaceEvent) -> Option<SurfaceEvent> {
    if has_keyboard_focus(window) {
        Some(event)
    } else {
        None
    }
}

/// Letra de `WM_CHAR`; los controles se ignoran. Solo con foco.
fn typed_character(message: &MSG) -> Option<SurfaceEvent> {
    if !has_keyboard_focus(message.hwnd) {
        return None;
    }
    let character = char::from_u32(message.wParam.0 as u32)?;
    if character.is_control() {
        return None;
    }
    Some(SurfaceEvent::MenubarTyped { character })
}

/// `true` si la ventana tiene el foco del teclado de este hilo.
fn has_keyboard_focus(window: HWND) -> bool {
    // SAFETY: consulta de solo lectura del foco del hilo.
    unsafe { GetFocus() == window }
}
