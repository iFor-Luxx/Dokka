# dokka-win-thumbnail

**Hace:** da la imagen viva de otra ventana (aunque esté minimizada) como un visual de
composición, sin copiar la pantalla, y el compositor en el que se puede usar.

**Riesgo:** usa funciones de Windows sin documentar (`DwmpCreateSharedThumbnailVisual`, ordinal 147
de `dwmapi.dll`, y el compositor «interop»). Una actualización de Windows podría romperlas. Si algo
falla devuelve un error y quien la usa sigue sin efecto.

**Interruptor:** lo manda quien la usa (hoy «Efecto genio al minimizar»).

**No hace:** no decide qué ventana mostrar, no anima, no crea ventanas.

Referencias: blog.adeltax.com/dwm-thumbnails-but-with-idcompositionvisual.
