use dokka_scene::TextCursor;

/// Único cursor para las tres casillas: la escena dice detrás de qué texto va
/// (`text_x`), con qué estilo (`content`, `size`) y hasta dónde puede llegar
/// (`max_x`); la plataforma mide ese texto y coloca ahí el cursor. Con la
/// casilla vacía no lleva aire: queda al inicio.
pub fn text_cursor(mut cursor: TextCursor) -> TextCursor {
    if cursor.content.is_empty() {
        cursor.gap = 0.0;
    }
    cursor
}
