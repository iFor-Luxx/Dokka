# dokka-win-tray

**Hace:** recibe los iconos que las apps ponen en la bandeja de Windows y les
reenvía los clics. Un hilo propio con una ventana oculta `Shell_TrayWnd`
atiende `Shell_NotifyIcon` por `WM_COPYDATA`, convierte cada `HICON` al
`Bitmap` de Dokka y reenvía todo a la barra real, así Windows sigue
funcionando igual. Al soltarlo, Windows vuelve a recibirlo todo directo.

**No hace:** no dibuja ni decide qué se muestra. Pieza aislada y apagable:
si algo falla, `start` devuelve error o la bandeja queda vacía; nunca se
cierra Dokka.
