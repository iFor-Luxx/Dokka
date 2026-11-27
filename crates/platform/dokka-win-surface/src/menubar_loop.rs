use windows::Win32::UI::WindowsAndMessaging::MSG;

use crate::SurfaceEvent;
use crate::read_pointer::read_pointer;

/// Traduce un mensaje del ratón o del teclado de la barra a su evento propio.
/// Las teclas del Launchpad (letras, flechas, rueda) no tienen sentido aquí y
/// se ignoran; Esc llega como `MenubarDismissed` igual que la pérdida de foco.
pub(crate) fn read_menubar(
    message: &MSG,
    scale: f32,
    is_pressed: &mut bool,
) -> Option<SurfaceEvent> {
    let event = read_pointer(message, scale, is_pressed)?;
    Some(match event {
        SurfaceEvent::PointerMoved { x, y } => SurfaceEvent::MenubarMoved { x, y },
        SurfaceEvent::PointerLeft => SurfaceEvent::MenubarLeft,
        SurfaceEvent::Clicked { x, y, .. } => SurfaceEvent::MenubarClicked { x, y },
        SurfaceEvent::Dismissed => SurfaceEvent::MenubarDismissed,
        _ => return None,
    })
}
