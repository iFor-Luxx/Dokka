use windows::Win32::UI::Input::KeyboardAndMouse::{
    TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent, VK_ESCAPE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    MSG, WM_KEYDOWN, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_RBUTTONUP,
};

use crate::SurfaceEvent;

/// `WM_MOUSELEAVE`, que la librería declara junto a controles que no usamos.
const MOUSE_LEFT: u32 = 0x02A3;
/// Bit de `wParam` que indica la tecla Mayús pulsada.
const SHIFT_HELD: usize = 0x0004;

/// Traduce mensajes del mouse. `is_pressed` recuerda que el botón bajó aquí.
pub(crate) fn read_pointer(
    message: &MSG,
    scale: f32,
    is_pressed: &mut bool,
) -> Option<SurfaceEvent> {
    // Las coordenadas llegan como dos enteros de 16 bits con signo, en píxeles.
    let x = f32::from(message.lParam.0 as i16) / scale;
    let y = f32::from((message.lParam.0 >> 16) as i16) / scale;

    match message.message {
        WM_KEYDOWN if message.wParam.0 == VK_ESCAPE.0 as usize => Some(SurfaceEvent::Dismissed),
        WM_MOUSEMOVE => {
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
        WM_LBUTTONUP if std::mem::take(is_pressed) => Some(SurfaceEvent::Clicked {
            x,
            y,
            with_shift: message.wParam.0 & SHIFT_HELD != 0,
        }),
        WM_RBUTTONUP => Some(SurfaceEvent::RightClicked { x, y }),
        _ => None,
    }
}
