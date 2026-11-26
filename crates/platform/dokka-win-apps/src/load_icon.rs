use std::path::Path;

use dokka_scene::Bitmap;
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::System::Com::IBindCtx;
use windows::Win32::UI::Shell::{
    IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_ICONONLY,
};
use windows::core::{HSTRING, Result};

use crate::read_bitmap::read_bitmap;

/// Lado pedido en píxeles; sobra resolución para pantallas escaladas.
const ICON_PIXELS: i32 = 128;

pub(crate) fn load_icon(executable: &Path) -> Result<Bitmap> {
    let path = HSTRING::from(executable.as_os_str());
    // SAFETY: `path` vive durante la llamada y el HBITMAP devuelto se libera aquí.
    unsafe {
        let factory: IShellItemImageFactory =
            SHCreateItemFromParsingName(&path, None::<&IBindCtx>)?;
        let size = SIZE {
            cx: ICON_PIXELS,
            cy: ICON_PIXELS,
        };
        let handle = factory.GetImage(size, SIIGBF_ICONONLY)?;
        let bitmap = read_bitmap(handle);
        let _ = DeleteObject(handle.into());
        bitmap
    }
}
