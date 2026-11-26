use dokka_scene::Drawing;
use dokka_theme::IconColors;

use crate::shapes::{rounded, tile};

/// Carpeta para el explorador de archivos.
pub fn explorer_icon(colors: &IconColors) -> Drawing {
    Drawing {
        shapes: vec![
            tile(colors.explorer_background),
            rounded(18.0, 24.0, 30.0, 18.0, 6.0, colors.folder_back),
            rounded(18.0, 30.0, 64.0, 46.0, 7.0, colors.folder_back),
            rounded(18.0, 40.0, 64.0, 36.0, 7.0, colors.folder_front),
        ],
    }
}
