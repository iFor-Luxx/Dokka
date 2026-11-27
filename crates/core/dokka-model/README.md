# dokka-model

**Hace:** define qué es una app, una ventana y una app dentro del dock.

**No hace:** no cambia el estado, no habla con Windows, no dibuja. No depende de ningún otro crate.

También define las preferencias del usuario (`Preferences`) y un icono de la bandeja (`TrayItem`).
El volumen trae además la salida activa (`AudioOutput`).
