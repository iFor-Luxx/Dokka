use dokka_scene::{Color, Drawing};

use crate::shapes::{WHITE, rounded, tile};

const BACKGROUND: Color = Color::rgba(32, 150, 243, 255);
const FOLDER_BACK: Color = Color::rgba(188, 224, 252, 255);

/// Carpeta sobre fondo azul para el explorador de archivos.
pub fn explorer_icon() -> Drawing {
    Drawing {
        shapes: vec![
            tile(BACKGROUND),
            rounded(18.0, 24.0, 30.0, 18.0, 6.0, FOLDER_BACK),
            rounded(18.0, 30.0, 64.0, 46.0, 7.0, FOLDER_BACK),
            rounded(18.0, 40.0, 64.0, 36.0, 7.0, WHITE),
        ],
    }
}
