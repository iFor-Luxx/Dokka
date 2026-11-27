//! Convierte el icono que una app deja en la bandeja a imagen propia.

use dokka_scene::Bitmap;
use windows::Win32::UI::WindowsAndMessaging::HICON;

use crate::hicon_bitmap::read_hicon;

/// Lado en píxeles del icono de bandeja; sobra para pantallas escaladas.
const TRAY_PIXELS: i32 = 32;

/// Imagen del icono, o `None` si Windows no lo pudo dibujar.
/// No destruye el `HICON`: sigue siendo de la app que lo registró.
pub fn tray_icon_bitmap(icon: HICON) -> Option<Bitmap> {
    read_hicon(icon, TRAY_PIXELS).ok()
}
